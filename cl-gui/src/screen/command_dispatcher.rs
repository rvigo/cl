use crate::clipboard::Clipboard;
use crate::component::{ClipboardStatus, EditableTextbox, FutureEventType, List, Popup, TextBox};
use crate::observer::event::ClipboardAction::Copied;
use crate::observer::event::{Event, ListEvent, PopupEvent, PopupType, TextBoxEvent};
use crate::observer::observable::Observable;
use crate::observer::subscription::SubscriptionSet;
use crate::screen::key_mapping::command::{ScreenCommand, ScreenCommandCallback};
use crate::screen::layer::{Layer, MainScreenLayer, PopupLayer};
use crate::screen::theme::Theme;
use crate::signal_handler::Signal::UserInt;
use crate::signal_handler::SignalHandler;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::CurrentCommand;
use cl_core::Command;
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, trace};
use tui::Frame;

/// Read-only snapshot of the current command list for UI-local navigation.
///
/// j/k keys index into this snapshot instead of round-tripping through the
/// `StateActor` via oneshot channels.  The snapshot is rebuilt on tab switch,
/// mutation (add/edit/delete), and search.
#[derive(Default)]
pub struct NavigationSnapshot {
    pub items: Vec<Command<'static>>,
    pub selected_idx: usize,
}

impl NavigationSnapshot {
    pub fn next_idx(&self) -> usize {
        if self.items.is_empty() {
            return 0;
        }
        (self.selected_idx + 1) % self.items.len()
    }

    pub fn prev_idx(&self) -> usize {
        if self.items.is_empty() {
            return 0;
        }
        (self.selected_idx + self.items.len() - 1) % self.items.len()
    }
}

/// Manages the layer stack (push/pop/replace) and listener bookkeeping.
pub struct LayerStack {
    pub subscriptions: SubscriptionSet<TypeId, Rc<RefCell<dyn Observable>>>,
    pub layers: Vec<Box<dyn Layer>>,
    pub snapshot: NavigationSnapshot,
}

impl LayerStack {
    pub fn new(initial: MainScreenLayer) -> Self {
        let subscriptions = SubscriptionSet::from(initial.get_listeners());
        Self {
            subscriptions,
            layers: vec![Box::new(initial)],
            snapshot: NavigationSnapshot::default(),
        }
    }

    /// Push a new layer onto the stack.
    ///
    /// Registers the layer's listeners, then calls
    /// [`Layer::on_attach`] so the layer can pre-populate its state.
    pub async fn add_layer(
        &mut self,
        layer: Box<dyn Layer + 'static>,
        state_tx: &Sender<StateEvent>,
    ) {
        debug!("adding layer");
        self.layers.push(layer);
        let subscriptions = {
            let top = self.layers.last().expect("just pushed a layer");
            SubscriptionSet::from(top.get_listeners())
        };
        self.subscriptions.extend(subscriptions);
        if let Some(top) = self.layers.last_mut() {
            top.on_attach(state_tx);
        }
    }

    /// Pop the topmost layer from the stack.
    ///
    /// Calls [`Layer::on_detach`] before removing listeners.
    pub fn remove_last_layer(&mut self) {
        if self.layers.len() == 1 {
            return;
        }

        if let Some(mut last) = self.layers.pop() {
            last.on_detach();
            for key in last.get_listeners().keys() {
                self.subscriptions.remove(key);
            }
        }
    }

    /// Replace the current top layer with `layer`.
    ///
    /// Calls `on_detach` on the old layer, then registers the new layer and
    /// calls `on_attach` on it.
    pub async fn replace_current_layer(
        &mut self,
        layer: Box<dyn Layer + 'static>,
        state_tx: &Sender<StateEvent>,
    ) {
        debug!("replacing current layer");
        if let Some(mut old) = self.layers.pop() {
            old.on_detach();
            for key in old.get_listeners().keys() {
                self.subscriptions.remove(key);
            }
        }

        self.layers.push(layer);
        let subscriptions = {
            let top = self.layers.last().expect("just pushed a layer");
            SubscriptionSet::from(top.get_listeners())
        };
        self.subscriptions.extend(subscriptions);
        if let Some(top) = self.layers.last_mut() {
            top.on_attach(state_tx);
        }
    }

    pub async fn notify(&mut self, id: TypeId, event: Event) {
        if let Some(subscriptions) = self.subscriptions.get(&id) {
            trace!("notifying {} listeners for {:?}", subscriptions.len(), id);
            for sub in subscriptions {
                let maybe_task = sub.listener.borrow_mut().on_listen(event.clone());
                if let Some(task) = maybe_task {
                    task.await;
                }
            }
        } else {
            debug!("no listeners registered for TypeId {:?}", id);
        }
    }

    pub fn render_layers(&mut self, frame: &mut Frame, theme: &Theme) {
        for layer in &mut self.layers {
            layer.render(frame, theme);
        }
    }
}

/// Dispatches `ScreenCommand` variants, delegating layer management to
/// [`LayerStack`] and handling clipboard, callbacks, forms, navigation,
/// and quit.
pub struct CommandDispatcher;

impl CommandDispatcher {
    pub async fn dispatch(
        commands: Vec<ScreenCommand>,
        layer_stack: &mut LayerStack,
        clipboard: &mut Option<Clipboard>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
    ) {
        for cmd in commands {
            match cmd {
                ScreenCommand::AddLayer(layer) => {
                    layer_stack.add_layer(layer, state_tx).await;
                }
                ScreenCommand::PopLastLayer(mut callback_receiver) => {
                    layer_stack.remove_last_layer();

                    if let Some(ref mut events) = callback_receiver {
                        Self::handle_callback_receiver(
                            events,
                            layer_stack,
                            clipboard,
                            state_tx,
                            sig_handler,
                        )
                        .await;
                    }
                }
                ScreenCommand::Notify((tid, event)) => {
                    layer_stack.notify(tid, event).await;
                }
                ScreenCommand::Quit => {
                    if let Err(e) = sig_handler.send_signal(UserInt) {
                        tracing::error!("failed to send quit signal: {e}");
                    }
                }
                ScreenCommand::CopyToClipboard => {
                    if let Some(cb) = clipboard {
                        if let Ok(Some(cmd)) = oneshot!(state_tx, CurrentCommand) {
                            if let Err(e) = cb.set_content(cmd.value.command) {
                                tracing::error!("failed to copy to clipboard: {e}");
                            }
                            layer_stack
                                .notify(
                                    TypeId::of::<ClipboardStatus>(),
                                    Event::ClipboardStatus(Copied),
                                )
                                .await;
                        }
                    }
                }
                ScreenCommand::Callback(cb) => {
                    if let Some(sub_commands) = cb.handle(state_tx).await {
                        // Use Box::pin to allow recursive dispatch
                        Box::pin(Self::dispatch(
                            sub_commands,
                            layer_stack,
                            clipboard,
                            state_tx,
                            sig_handler,
                        ))
                        .await;
                    }
                }
                ScreenCommand::ReplaceCurrentLayer(layer) => {
                    layer_stack.replace_current_layer(layer, state_tx).await;
                }
                ScreenCommand::GetFieldContent => {
                    debug!("notifying observers to collect field content");
                    layer_stack
                        .notify(
                            TypeId::of::<EditableTextbox>(),
                            Event::EditableTextbox(
                                crate::observer::event::EditableTextboxEvent::GetFieldContent(
                                    state_tx.clone(),
                                ),
                            ),
                        )
                        .await;
                }
                ScreenCommand::Form(cb) => {
                    if let Err(error_msg) = cb.handle(state_tx.clone()).await {
                        layer_stack
                            .add_layer(Box::new(PopupLayer::default()), state_tx)
                            .await;
                        let popup_event = PopupEvent::Create(PopupType::Dialog(
                            format!("Error: {error_msg}"),
                            FutureEventType::State(|_| Box::pin(async { Ok(()) })),
                            ScreenCommandCallback::DoNothing,
                        ));
                        layer_stack
                            .notify(TypeId::of::<Popup>(), Event::Popup(popup_event))
                            .await;
                    }
                }
                ScreenCommand::NavigateNext => {
                    Self::navigate(layer_stack, state_tx, true).await;
                }
                ScreenCommand::NavigatePrev => {
                    Self::navigate(layer_stack, state_tx, false).await;
                }
                ScreenCommand::SetSnapshot {
                    items,
                    selected_idx,
                } => {
                    layer_stack.snapshot = NavigationSnapshot {
                        items,
                        selected_idx,
                    };
                }
            }
        }
    }

    /// Navigate the UI-local snapshot and emit notify events.
    ///
    /// Sends a fire-and-forget `SyncSelection` to keep the `StateActor`
    /// in sync without blocking.
    async fn navigate(layer_stack: &mut LayerStack, state_tx: &Sender<StateEvent>, forward: bool) {
        if layer_stack.snapshot.items.is_empty() {
            return;
        }

        let new_idx = if forward {
            layer_stack.snapshot.next_idx()
        } else {
            layer_stack.snapshot.prev_idx()
        };
        layer_stack.snapshot.selected_idx = new_idx;

        // Extract data from snapshot before borrowing layer_stack for notify
        let cmd = layer_stack.snapshot.items.get(new_idx).cloned();

        if let Some(cmd) = cmd {
            let list_event = if forward {
                ListEvent::Next(new_idx)
            } else {
                ListEvent::Previous(new_idx)
            };

            layer_stack
                .notify(TypeId::of::<List>(), Event::List(list_event))
                .await;
            layer_stack
                .notify(
                    TypeId::of::<TextBox>(),
                    Event::TextBox(TextBoxEvent::UpdateCommand(cmd)),
                )
                .await;

            // Fire-and-forget: keep StateActor selection in sync
            if let Err(e) = state_tx.send(StateEvent::SyncSelection(new_idx)).await {
                tracing::error!("failed to sync selection: {e}");
            }
        }
    }

    async fn handle_callback_receiver(
        callback_receiver: &mut Receiver<ScreenCommandCallback>,
        layer_stack: &mut LayerStack,
        clipboard: &mut Option<Clipboard>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
    ) {
        match callback_receiver.recv().await {
            Some(message) => match message {
                ScreenCommandCallback::ExitEditScreen => {
                    layer_stack
                        .replace_current_layer(Box::new(MainScreenLayer::default()), state_tx)
                        .await;
                    if let Some(sub_commands) =
                        ScreenCommandCallback::UpdateAll.handle(state_tx).await
                    {
                        Box::pin(Self::dispatch(
                            sub_commands,
                            layer_stack,
                            clipboard,
                            state_tx,
                            sig_handler,
                        ))
                        .await;
                    }
                }
                _ => {
                    let sub_commands = message.handle(state_tx).await.unwrap_or_else(|| {
                        tracing::warn!("callback handler returned no events");
                        vec![]
                    });
                    Box::pin(Self::dispatch(
                        sub_commands,
                        layer_stack,
                        clipboard,
                        state_tx,
                        sig_handler,
                    ))
                    .await;
                }
            },
            None => debug!("callback channel closed with no message"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::key_mapping::command::ScreenCommand;
    use cl_core::CommandBuilder;

    #[test]
    fn layer_stack_starts_with_one_layer() {
        let stack = LayerStack::new(MainScreenLayer::default());
        assert_eq!(stack.layers.len(), 1);
    }

    #[test]
    fn remove_last_layer_does_not_remove_only_layer() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        stack.remove_last_layer();
        assert_eq!(stack.layers.len(), 1, "should not pop the only layer");
    }

    #[tokio::test]
    async fn add_and_remove_layer() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let (tx, _rx) = tokio::sync::mpsc::channel(1);

        stack.add_layer(Box::new(PopupLayer::default()), &tx).await;
        assert_eq!(stack.layers.len(), 2);

        stack.remove_last_layer();
        assert_eq!(stack.layers.len(), 1);
    }

    #[tokio::test]
    async fn replace_current_layer_keeps_count() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let (tx, _rx) = tokio::sync::mpsc::channel(1);

        stack.add_layer(Box::new(PopupLayer::default()), &tx).await;
        assert_eq!(stack.layers.len(), 2);

        stack
            .replace_current_layer(Box::new(PopupLayer::default()), &tx)
            .await;
        assert_eq!(
            stack.layers.len(),
            2,
            "replace should not change layer count"
        );
    }

    #[tokio::test]
    async fn dispatch_quit_sends_signal() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        CommandDispatcher::dispatch(
            vec![ScreenCommand::Quit],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;
    }

    #[tokio::test]
    async fn dispatch_add_layer_pushes() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        CommandDispatcher::dispatch(
            vec![ScreenCommand::AddLayer(Box::new(PopupLayer::default()))],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.layers.len(), 2);
    }

    #[tokio::test]
    async fn dispatch_pop_last_layer_pops() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        stack
            .add_layer(Box::new(PopupLayer::default()), &state_tx)
            .await;
        assert_eq!(stack.layers.len(), 2);

        CommandDispatcher::dispatch(
            vec![ScreenCommand::PopLastLayer(None)],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.layers.len(), 1);
    }

    fn make_cmd(alias: &'static str) -> Command<'static> {
        CommandBuilder::default()
            .alias(alias)
            .namespace("ns")
            .command("echo test")
            .build()
    }

    #[test]
    fn snapshot_next_idx_wraps() {
        let snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 2,
        };
        assert_eq!(snapshot.next_idx(), 0);
    }

    #[test]
    fn snapshot_prev_idx_wraps() {
        let snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 0,
        };
        assert_eq!(snapshot.prev_idx(), 2);
    }

    #[test]
    fn snapshot_empty_returns_zero() {
        let snapshot = NavigationSnapshot::default();
        assert_eq!(snapshot.next_idx(), 0);
        assert_eq!(snapshot.prev_idx(), 0);
    }

    #[tokio::test]
    async fn navigate_next_updates_snapshot() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        stack.snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 0,
        };
        let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(16);

        CommandDispatcher::navigate(&mut stack, &state_tx, true).await;

        assert_eq!(stack.snapshot.selected_idx, 1);

        // Should have sent SyncSelection(1) fire-and-forget
        if let Some(StateEvent::SyncSelection(idx)) = state_rx.recv().await {
            assert_eq!(idx, 1);
        } else {
            panic!("expected SyncSelection event");
        }
    }

    #[tokio::test]
    async fn navigate_prev_updates_snapshot() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        stack.snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 0,
        };
        let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(16);

        CommandDispatcher::navigate(&mut stack, &state_tx, false).await;

        assert_eq!(stack.snapshot.selected_idx, 2);

        if let Some(StateEvent::SyncSelection(idx)) = state_rx.recv().await {
            assert_eq!(idx, 2);
        } else {
            panic!("expected SyncSelection event");
        }
    }

    #[tokio::test]
    async fn set_snapshot_updates_items() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        let items = vec![make_cmd("x"), make_cmd("y")];
        CommandDispatcher::dispatch(
            vec![ScreenCommand::SetSnapshot {
                items: items.clone(),
                selected_idx: 1,
            }],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.snapshot.items.len(), 2);
        assert_eq!(stack.snapshot.selected_idx, 1);
    }
}

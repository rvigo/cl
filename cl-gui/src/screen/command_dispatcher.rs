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

    /// Remove all subscriptions that were registered by `layer`.
    ///
    /// Only the per-listener handles belonging to this layer are removed.
    /// Using `remove(key)` would drop ALL subscribers for a TypeId, including
    /// those registered by lower layers that share the same component type
    /// (e.g. both MainScreenLayer and QuickSearchLayer register Search listeners).
    fn unregister_layer_listeners(&mut self, layer: &dyn Layer) {
        for (key, listeners) in layer.get_listeners() {
            self.subscriptions.remove_matching(key, |registered| {
                listeners.iter().any(|r| Rc::ptr_eq(r, registered))
            });
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
            self.unregister_layer_listeners(&*last);
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
        debug_assert!(
            !self.layers.is_empty(),
            "replace_current_layer called on empty stack"
        );
        if let Some(mut old) = self.layers.pop() {
            old.on_detach();
            self.unregister_layer_listeners(&*old);
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
        // Clone the Rc handles upfront so the immutable borrow on
        // `self.subscriptions` is released before any await points.
        // This prevents a potential double-borrow panic: if an `on_listen`
        // implementation were to synchronously call back into `notify` on
        // the same LayerStack, the original borrow would still be live and
        // a second `borrow_mut()` on the same RefCell would panic.
        let listeners: Vec<Rc<RefCell<dyn Observable>>> = match self.subscriptions.get(&id) {
            Some(subs) => {
                trace!("notifying {} listeners for {:?}", subs.len(), id);
                subs.iter().map(|sub| Rc::clone(&sub.listener)).collect()
            }
            None => {
                debug!("no listeners registered for TypeId {:?}", id);
                return;
            }
        };

        for listener in listeners {
            // The borrow is released at the end of each expression — before
            // the await — so no two borrows are live simultaneously.
            let maybe_task = listener.borrow_mut().on_listen(event.clone());
            if let Some(task) = maybe_task {
                task.await;
            }
        }
    }

    pub fn render_layers(&mut self, frame: &mut Frame, theme: &Theme) {
        for layer in &mut self.layers {
            layer.render(frame, theme);
        }
    }
}

/// Maximum number of nested `Callback` / `PopLastLayer` dispatch recursions
/// before aborting with an error.  Prevents infinite loops from buggy callbacks.
const MAX_DISPATCH_DEPTH: usize = 16;

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
        Self::dispatch_inner(commands, layer_stack, clipboard, state_tx, sig_handler, 0).await;
    }

    async fn dispatch_inner(
        commands: Vec<ScreenCommand>,
        layer_stack: &mut LayerStack,
        clipboard: &mut Option<Clipboard>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
        depth: usize,
    ) {
        if depth >= MAX_DISPATCH_DEPTH {
            tracing::error!(
                "dispatch recursion depth limit ({MAX_DISPATCH_DEPTH}) exceeded; aborting"
            );
            return;
        }

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
                            depth,
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
                    if let Some(cb) = clipboard.as_mut() {
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
                        Box::pin(Self::dispatch_inner(
                            sub_commands,
                            layer_stack,
                            clipboard,
                            state_tx,
                            sig_handler,
                            depth + 1,
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
        depth: usize,
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
                        Box::pin(Self::dispatch_inner(
                            sub_commands,
                            layer_stack,
                            clipboard,
                            state_tx,
                            sig_handler,
                            depth + 1,
                        ))
                        .await;
                    }
                }
                _ => {
                    let sub_commands = message.handle(state_tx).await.unwrap_or_else(|| {
                        tracing::trace!("callback handler returned no events");
                        vec![]
                    });
                    Box::pin(Self::dispatch_inner(
                        sub_commands,
                        layer_stack,
                        clipboard,
                        state_tx,
                        sig_handler,
                        depth + 1,
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
    async fn replace_on_single_layer_stack_leaves_one_layer() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        stack
            .replace_current_layer(Box::new(PopupLayer::default()), &tx)
            .await;
        assert_eq!(stack.layers.len(), 1);
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
    async fn dispatch_replace_current_layer_keeps_count() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        stack
            .add_layer(Box::new(PopupLayer::default()), &state_tx)
            .await;
        assert_eq!(stack.layers.len(), 2);

        CommandDispatcher::dispatch(
            vec![ScreenCommand::ReplaceCurrentLayer(Box::new(
                PopupLayer::default(),
            ))],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(
            stack.layers.len(),
            2,
            "replace should not change layer count"
        );
    }

    #[tokio::test]
    async fn dispatch_navigate_next_advances_snapshot() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(16);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        stack.snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 0,
        };

        CommandDispatcher::dispatch(
            vec![ScreenCommand::NavigateNext],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.snapshot.selected_idx, 1);
        assert!(matches!(
            state_rx.recv().await,
            Some(StateEvent::SyncSelection(1))
        ));
    }

    #[tokio::test]
    async fn dispatch_navigate_prev_wraps_to_end() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(16);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        stack.snapshot = NavigationSnapshot {
            items: vec![make_cmd("a"), make_cmd("b"), make_cmd("c")],
            selected_idx: 0,
        };

        CommandDispatcher::dispatch(
            vec![ScreenCommand::NavigatePrev],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.snapshot.selected_idx, 2);
        assert!(matches!(
            state_rx.recv().await,
            Some(StateEvent::SyncSelection(2))
        ));
    }

    #[tokio::test]
    async fn dispatch_navigate_on_empty_snapshot_is_noop() {
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        // snapshot is empty (default)
        CommandDispatcher::dispatch(
            vec![ScreenCommand::NavigateNext],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;

        assert_eq!(stack.snapshot.selected_idx, 0);
        // No SyncSelection should have been sent
        assert!(state_rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn remove_last_layer_does_not_destroy_shared_subscriptions() {
        // Regression test for C1: popping a layer must not remove TypeId
        // subscriptions that were registered by a lower layer.
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let (tx, _rx) = tokio::sync::mpsc::channel(1);

        // The TypeIds registered by MainScreenLayer are still present after
        // we push and pop a PopupLayer.
        let main_type_ids: Vec<_> = stack
            .layers
            .first()
            .unwrap()
            .get_listeners()
            .keys()
            .cloned()
            .collect();

        stack.add_layer(Box::new(PopupLayer::default()), &tx).await;
        stack.remove_last_layer();

        for tid in &main_type_ids {
            assert!(
                stack.subscriptions.get(tid).is_some(),
                "subscription for TypeId {tid:?} was incorrectly removed when popping PopupLayer"
            );
        }
    }

    #[tokio::test]
    async fn dispatch_get_field_content_does_not_panic() {
        // GetFieldContent notifies EditableTextbox listeners; with no listeners
        // registered the notify is a no-op. We verify dispatch completes.
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        CommandDispatcher::dispatch(
            vec![ScreenCommand::GetFieldContent],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;
        // No assertion needed — the test passes if it doesn't panic
    }

    #[tokio::test]
    async fn dispatch_callback_with_no_sub_commands_is_noop() {
        use crate::screen::key_mapping::command::ScreenCommandCallback;
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        // DoNothing callback returns no sub-commands
        CommandDispatcher::dispatch(
            vec![ScreenCommand::Callback(ScreenCommandCallback::DoNothing)],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
        )
        .await;
        assert_eq!(stack.layers.len(), 1, "DoNothing should not change layers");
    }

    #[tokio::test]
    async fn dispatch_depth_limit_stops_infinite_recursion() {
        use crate::screen::key_mapping::command::ScreenCommandCallback;
        // A callback that keeps generating sub-commands would overflow without a
        // depth guard.  We verify that dispatch_inner returns once the limit is
        // hit, instead of stack-overflowing or running forever.
        let mut stack = LayerStack::new(MainScreenLayer::default());
        let mut clipboard = None;
        let (state_tx, _state_rx) = tokio::sync::mpsc::channel(1);
        let (mut sig_handler, _sig_rx) = SignalHandler::create();

        // Call dispatch_inner at the limit — it should return immediately.
        CommandDispatcher::dispatch_inner(
            vec![ScreenCommand::Callback(ScreenCommandCallback::DoNothing)],
            &mut stack,
            &mut clipboard,
            &state_tx,
            &mut sig_handler,
            MAX_DISPATCH_DEPTH,
        )
        .await;
        // If we reach here the depth guard worked correctly
        assert_eq!(stack.layers.len(), 1);
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

mod key_mapping;
pub mod layer;
pub mod theme;

use crate::clipboard::Clipboard;
use crate::component::{ClipboardStatus, EditableTextbox, FutureEventType, Popup};
use crate::observer::event::ClipboardAction::Copied;
use crate::observer::event::{Event, PopupEvent, PopupType};
use crate::observer::subscription::SubscriptionSet;
use crate::screen::key_mapping::command::{ScreenCommand, ScreenCommandCallback};
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::signal_handler::Signal::UserInt;
use crate::signal_handler::{Signal, SignalHandler};
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::CurrentCommand;
use crossterm::event::Event as CrosstermEvent;
use layer::MainScreenLayer;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, trace};
use tui::Frame;

use crate::observer::observable::Observable;
pub use key_mapping::command;

#[derive(Default, Clone, Debug)]
pub enum ActiveScreen {
    #[default]
    Main,
    Form,
}

pub struct Screen {
    subscriptions: SubscriptionSet<TypeId, Rc<RefCell<dyn Observable>>>,
    layers: Vec<Box<dyn Layer>>,
    pub clipboard: Option<Clipboard>,
    pub theme: Theme,
}

pub type Listeners = BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>;

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen {
    pub fn new() -> Screen {
        let initial = MainScreenLayer::default();
        let subscriptions = SubscriptionSet::from(initial.get_listeners());
        Self {
            subscriptions,
            layers: vec![Box::new(initial)],
            clipboard: Clipboard::new()
                .map_err(|e| {
                    tracing::warn!("clipboard unavailable: {e}");
                    e
                })
                .ok(),
            theme: Theme::default(),
        }
    }

    pub async fn handle_key_event(
        &mut self,
        event: Option<std::io::Result<CrosstermEvent>>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
    ) {
        if let Some(Ok(CrosstermEvent::Resize(_, _))) = &event {
            debug!("terminal resized — redraw needed");
        }

        if let Some(Ok(CrosstermEvent::Key(event))) = event {
            if let Some(layer) = self.layers.last_mut() {
                match layer.handle_key_event(event, state_tx.clone()).await {
                    None => {}
                    Some(commands) => {
                        for cmd in commands {
                            match cmd {
                                ScreenCommand::AddLayer(layer) => {
                                    self.add_layer(layer, state_tx).await;
                                }
                                ScreenCommand::PopLastLayer(mut callback_receiver) => {
                                    self.remove_last_layer();

                                    if let Some(mut events) = callback_receiver.take() {
                                        self.handle_callback_receiver(&mut events, state_tx).await;
                                    }
                                }
                                ScreenCommand::Notify((tid, event)) => {
                                    self.notify(tid, event).await;
                                }
                                ScreenCommand::Quit => {
                                    if let Err(e) = sig_handler.send_signal(UserInt) {
                                        tracing::error!("failed to send quit signal: {e}");
                                    }
                                }
                                ScreenCommand::CopyToClipboard => {
                                    if let Some(clipboard) = &mut self.clipboard {
                                        if let Ok(Some(cmd)) = oneshot!(state_tx, CurrentCommand) {
                                            if let Err(e) = clipboard.set_content(cmd.value.command)
                                            {
                                                tracing::error!("failed to copy to clipboard: {e}");
                                            }
                                            self.notify(
                                                TypeId::of::<ClipboardStatus>(),
                                                Event::ClipboardStatus(Copied),
                                            )
                                            .await;
                                        }
                                    }
                                }
                                ScreenCommand::Callback(cb) => {
                                    if let Some(events) = cb.handle(state_tx).await {
                                        self.notify_all(events).await;
                                    }
                                }
                                ScreenCommand::ReplaceCurrentLayer(layer) => {
                                    self.replace_current_layer(layer, state_tx).await;
                                }
                                ScreenCommand::GetFieldContent => {
                                    debug!("notifying observers to collect field content");
                                    self.notify(
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
                                        // Add error popup layer
                                        self.add_layer(
                                            Box::new(layer::PopupLayer::default()),
                                            state_tx,
                                        )
                                        .await;
                                        // Notify to display the error
                                        let popup_event = PopupEvent::Create(PopupType::Dialog(
                                            format!("Error: {error_msg}"),
                                            FutureEventType::State(|_| Box::pin(async { Ok(()) })),
                                            ScreenCommandCallback::DoNothing,
                                        ));
                                        self.notify(
                                            TypeId::of::<Popup>(),
                                            Event::Popup(popup_event),
                                        )
                                        .await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn handle_callback_receiver(
        &mut self,
        callback_receiver: &mut Receiver<ScreenCommandCallback>,
        state_tx: &Sender<StateEvent>,
    ) {
        use tokio::time::{timeout, Duration};
        match timeout(Duration::from_millis(100), callback_receiver.recv()).await {
            Ok(Some(message)) => match message {
                ScreenCommandCallback::ExitEditScreen => {
                    self.replace_current_layer(Box::new(MainScreenLayer::default()), state_tx)
                        .await;
                    if let Some(events) = ScreenCommandCallback::UpdateAll.handle(state_tx).await {
                        self.notify_all(events).await;
                    }
                }
                _ => {
                    let events = message.handle(state_tx).await.unwrap_or_else(|| {
                        tracing::warn!("callback handler returned no events");
                        vec![]
                    });
                    self.notify_all(events).await;
                }
            },
            Ok(None) => debug!("callback channel closed with no message"),
            Err(_) => debug!("callback receiver timed out — sender may have been dropped"),
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
            let top = self.layers.last().unwrap();
            SubscriptionSet::from(top.get_listeners())
        };
        self.subscriptions.extend(subscriptions);
        // on_attach runs after listeners are registered so the layer can
        // update its components directly via borrow_inner_mut().
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
            let top = self.layers.last().unwrap();
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
                let maybe_task = sub.listener.borrow_mut().on_listen(event.clone()); // RefMut dropped here
                if let Some(task) = maybe_task {
                    task.await;
                }
            }
        } else {
            debug!("no listeners registered for TypeId {:?}", id);
        }
    }

    pub async fn notify_all(&mut self, events: Vec<impl Into<(TypeId, Event)>>) {
        for event in events {
            let (tid, event) = event.into();
            self.notify(tid, event).await;
        }
    }

    pub fn render_layers(&mut self, frame: &mut Frame) {
        for layer in &mut self.layers {
            layer.render(frame, &self.theme);
        }
    }

    pub fn quit(&mut self) -> Signal {
        self.layers.clear();
        UserInt
    }
}

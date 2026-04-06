mod key_mapping;
pub mod layer;
pub mod theme;

use crate::clipboard::Clipboard;
use crate::component::EditableTextbox;
use crate::observer::event::ClipboardAction::Copied;
use crate::observer::event::Event;
use crate::observer::subscription::SubscriptionSet;
use crate::oneshot;
use crate::screen::key_mapping::command::{ScreenCommand, ScreenCommandCallback};
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::signal_handler::Signal::UserInt;
use crate::signal_handler::{SignalHandler, Signal};
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::CurrentCommand;
use crossterm::event::Event as CrosstermEvent;
use layer::MainScreenLayer;
use log::{debug, error, trace};
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tokio::sync::mpsc::{Receiver, Sender};
use tui::Frame;

use crate::observer::observable::Observable;
pub use key_mapping::command;

#[derive(Default, Clone, Debug)]
pub enum ActiveScreen {
    #[default]
    Main,
}

pub struct Screen {
    pub active_screen: ActiveScreen,
    pub subscriptions: SubscriptionSet<TypeId, Rc<RefCell<dyn Observable>>>,
    pub layers: Vec<Box<dyn Layer>>,
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
        let mut screens = Self {
            active_screen: ActiveScreen::Main,
            subscriptions: SubscriptionSet::default(),
            layers: Vec::new(),
            clipboard: Clipboard::new().ok(),
            theme: Theme::load(),
        };

        let active_screen = screens.get_active_screen_mut();
        let listeners = active_screen.get_listeners();

        let layers: Vec<Box<dyn Layer>> = vec![Box::new(active_screen)];

        screens.subscriptions = SubscriptionSet::from(listeners);
        screens.layers = layers;

        screens
    }

    pub async fn handle_key_event(
        &mut self,
        event: Option<std::io::Result<CrosstermEvent>>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
    ) {
        if let Some(Ok(CrosstermEvent::Key(event))) = event {
            if let Some(layer) = self.layers.last_mut() {
                match layer.handle_key_event(event, state_tx.clone()).await {
                    None => {}
                    Some(commands) => {
                        // TODO normalize this enum handler
                        for cmd in commands {
                            match cmd {
                                ScreenCommand::AddLayer(layer) => {
                                    self.add_layer(layer).await;
                                }
                                ScreenCommand::PopLastLayer(mut callback_receiver) => {
                                    self.remove_last_layer().await;

                                    if let Some(mut events) = callback_receiver.take() {
                                        self.handle_callback_receiver(&mut events, state_tx).await
                                    }
                                }
                                ScreenCommand::Notify((tid, event)) => {
                                    self.notify(tid, event).await;
                                }
                                ScreenCommand::Quit => {
                                    sig_handler.send_signal(UserInt).ok();
                                }
                                ScreenCommand::CopyToClipboard => {
                                    if let Some(clipboard) = &mut self.clipboard {
                                        if let Some(Some(cmd)) = oneshot!(state_tx, CurrentCommand)
                                        {
                                            clipboard.set_content(cmd.value.command).ok();
                                            self.notify(
                                                TypeId::of::<Clipboard>(),
                                                Event::Clipboard(Copied),
                                            )
                                            .await
                                        }
                                    }
                                }
                                ScreenCommand::Callback(cb) => {
                                    if let Some(events) = cb.handle(state_tx).await {
                                        self.notify_all(events).await
                                    }
                                }
                                ScreenCommand::ReplaceCurrentLayer(layer) => {
                                    self.replace_current_layer(layer).await
                                }
                                ScreenCommand::GetFieldContent => {
                                    debug!("Notifying observers to get field content");
                                    self.notify(
                                        TypeId::of::<EditableTextbox>(),
                                        Event::GetFieldContent(state_tx.clone()),
                                    )
                                    .await;
                                }
                                ScreenCommand::Edit(cb) => {
                                    cb.handle(state_tx.clone()).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn handle_callback_receiver(
        &mut self,
        callback_receiver: &mut Receiver<ScreenCommandCallback>,
        state_tx: &Sender<StateEvent>,
    ) {
        if let Some(message) = callback_receiver.recv().await {
            match message {
                ScreenCommandCallback::ExitEditScreen => {
                    self.replace_current_layer(Box::new(MainScreenLayer::new())).await;
                    if let Some(events) = ScreenCommandCallback::UpdateAll.handle(state_tx).await {
                        self.notify_all(events).await;
                    }
                }
                _ => {
                    let events = message.handle(state_tx).await;
                    self.notify_all(events.unwrap_or_default()).await;
                }
            }
        }
    }

    pub async fn add_layer(&mut self, layer: Box<dyn Layer + 'static>) {
        debug!("adding layer");
        self.layers.push(layer);
        self.update_listeners().await;
    }

    pub async fn remove_last_layer(&mut self) {
        if self.layers.len() == 1 {
            return;
        }

        if let Some(last) = self.layers.pop() {
            let listeners = last.get_listeners();
            for (key, _) in listeners {
                self.subscriptions.remove(&key);
            }
        }
    }

    pub async fn replace_current_layer(&mut self, layer: Box<dyn Layer + 'static>) {
        debug!("removing listeners from the last layer");
        if let Some(last) = self.layers.pop() {
            let listeners = last.get_listeners();
            debug!("removing {:#?} listeners", listeners.len());
            for (key, _) in listeners {
                self.subscriptions.remove(&key);
            }
        }

        debug!("adding new layer");
        self.layers.push(layer);
        debug!("updating listeners");
        self.update_listeners().await;
    }

    pub async fn update_listeners(&mut self) {
        let mut listeners = Listeners::new();

        for layer in &self.layers {
            let layer_listeners = layer.get_listeners();
            listeners.extend(layer_listeners);
        }

        debug!(
            "Adding {} listeners to current {}",
            listeners.len(),
            self.subscriptions.subscriptions.len()
        );
        self.subscriptions.extend(SubscriptionSet::from(listeners));
    }

    // TODO rethink this method name
    pub async fn notify(&mut self, id: TypeId, event: Event) {
        if let Some(subscriptions) = self.subscriptions.get(&id) {
            trace!("notifying {} listeners", subscriptions.len());
            for sub in subscriptions {
                sub.listener.borrow_mut().on_listen(event.clone()).await;
            }
        } else {
            error!("No listeners found for TypeId {:?}", id);
        }
    }

    pub async fn notify_all(&mut self, events: Vec<impl Into<(TypeId, Event)>>) {
        for event in events {
            let (tid, event) = event.into();
            self.notify(tid, event).await
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

    fn get_active_screen_mut(&mut self) -> impl Layer {
        match self.active_screen {
            ActiveScreen::Main => MainScreenLayer::new(),
        }
    }
}

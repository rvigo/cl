mod key_mapping;
pub mod layer;

use crate::component::SharedComponent;
use crate::observer::event::Event;
use crate::observer::subscription::SubscriptionSet;
use crate::screen::key_mapping::ScreenCommand;
use crate::screen::layer::Layer;
use crate::signal_handler::Signal::UserInt;
use crate::signal_handler::{SigHandler, Signal};
use crate::state::state_event::StateEvent;
use crossterm::event::Event as CrosstermEvent;
use layer::MainScreenLayer;
use log::error;
use std::any::TypeId;
use std::collections::BTreeMap;
use tokio::sync::mpsc::Sender;
use tui::Frame;

#[derive(Default)]
pub enum ActiveScreen {
    #[default]
    Main,
}

pub struct Screens {
    pub active_screen: ActiveScreen,
    pub subscriptions: SubscriptionSet<TypeId, SharedComponent>,
    pub layers: Vec<Box<dyn Layer>>,
}

pub type Listeners = BTreeMap<TypeId, Vec<SharedComponent>>;

impl Screens {
    pub fn new() -> Screens {
        /* TODO
           need to handle a `quit` event.
           may it be a signal?
           may it be a `break`?
           we'll see
        */

        /* TODO
           how can I handle error at the `backend` level?
           should it have its own signal handler? Should I check the returns of the methods?
        */
        let mut screens = Self {
            active_screen: ActiveScreen::Main,
            subscriptions: SubscriptionSet::new(),
            layers: Vec::new(),
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
        sig_handler: &mut SigHandler,
    ) {
        if let Some(Ok(CrosstermEvent::Key(event))) = event {
            if let Some(layer) = self.layers.last_mut() {
                match layer.handle_key_event(event, state_tx.clone()).await {
                    None => {}
                    Some(commands) => {
                        for cmd in commands {
                            match cmd {
                                ScreenCommand::AddLayer(layer) => {
                                    self.add_layer(layer).await;
                                }
                                ScreenCommand::PopLastLayer => {
                                    self.remove_last_layer().await;
                                }
                                ScreenCommand::Notify((tid, event)) => {
                                    self.notify(tid, event).await;
                                }
                                ScreenCommand::Quit => {
                                    sig_handler.send_signal(UserInt).ok();
                                }
                            };
                        }
                    }
                }
            }
        }
    }

    pub async fn add_layer(&mut self, layer: Box<dyn Layer + 'static>) {
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
                self.subscriptions.remove(key);
            }
        }
    }

    pub async fn update_listeners(&mut self) {
        let mut listeners = Listeners::new();

        for layer in &self.layers {
            let layer_listeners = layer.get_listeners();
            listeners.extend(layer_listeners);
        }

        self.subscriptions.extend(SubscriptionSet::from(listeners));
    }

    // TODO rethink this method name
    pub async fn notify(&mut self, id: TypeId, event: Event) {
        if let Some(subscriptions) = self.subscriptions.get(&id) {
            for sub in subscriptions {
                sub.listener.borrow_mut().on_listen(event.clone()).await;
            }
        } else {
            error!("No listeners found for TypeId {:?}", id);
        }
    }

    pub fn render_layers(&mut self, frame: &mut Frame) {
        for layer in &mut self.layers {
            layer.render(frame);
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

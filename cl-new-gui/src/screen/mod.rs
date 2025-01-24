mod key_mapping;
pub mod layer;

use crate::component::SharedComponent;
use crate::observer::event::Event;
use crate::observer::subscription::SubscriptionSet;
use crate::screen::key_mapping::ScreenCommand;
use crate::screen::layer::Layer;
use crate::state::state_event::StateEvent;
use crossterm::event::Event as CrosstermEvent;
use layer::MainScreenLayer;
use log::{debug, error};
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
        .  add handlers to this field [layers] (getters, setters, add, etc)
        .  remove the main screen from the options and change the get_active_screen_mut method
           to return the current layer pile
        .  all items inside the layers field should be rendered from the lowest to the highest value

        .  when an item is registered in the layers field, it should be added to the subscriptions field
        .  when an item is removed from the layers field, it should be removed from the subscriptions field
        */

        /* TODO
           need a way to isolate the keystroke handler from each layer
           should I create a `handler` structure again?
           where should it go?
        */
        
        /* TODO
            need to handle a `quit` event.
            may it be a signal?
            may it be a `break`?
            we'll see
        
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
    ) {
        if let Some(Ok(CrosstermEvent::Key(event))) = event {
            if let Some(layer) = self.layers.last_mut() {
                match layer.handle_key_event(event, state_tx.clone()).await {
                    None => {}
                    Some(commands) => {
                        for cmd in commands {
                            match cmd {
                                ScreenCommand::AddLayer(layer) => self.add_layer(layer).await,
                                ScreenCommand::PopLastLayer => self.remove_last_layer().await,
                                ScreenCommand::Notify((tid, event)) => {
                                    self.notify(tid, event).await
                                }
                                ScreenCommand::Quit => todo!(),
                            }
                        }
                    }
                }
            }
        };
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

    fn get_active_screen_mut(&mut self) -> impl Layer {
        match self.active_screen {
            ActiveScreen::Main => MainScreenLayer::new(),
        }
    }
}

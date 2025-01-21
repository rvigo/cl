pub mod layer;

use crate::component::SharedComponent;
use crate::listen;
use crate::observer::event::Event;
use crate::observer::subscription::SubscriptionSet;
use crate::screen::layer::Layer;
use layer::MainScreenLayer;
use log::error;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::Frame;

#[macro_export]
macro_rules! listen {
    ($what:expr, $event:expr) => {
        $what.borrow_mut().on_listen($event.clone())
    };
}

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

    pub async fn add_layer(&mut self, layer: impl Layer + 'static) {
        self.layers.push(Box::new(layer));
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
                listen!(sub.listener, event);
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

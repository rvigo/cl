mod main_screen;

use crate::component::{BTreeMapExt, SharedComponent, SharedStatefulComponent};
use crate::observer::event::Event;
use crate::observer::subscription::SubscriptionSet;
use crate::screen::main_screen::MainScreen;
use log::debug;
use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use tui::Frame;

#[derive(Default)]
pub enum ActiveScreen {
    #[default]
    Main,
}

impl ActiveScreen {
    fn into_iter() -> impl Iterator<Item = ActiveScreen> {
        vec![ActiveScreen::Main].into_iter()
    }
}

#[macro_export]
macro_rules! listen {
    ($what:expr, $type:tt, $event:expr) => {
        $what
            .downcast_ref::<$type>()
            .expect("Cannot cast component")
            .borrow_mut()
            .on_listen($event.clone())
    };
}

pub struct Screens {
    pub active_screen: ActiveScreen,
    pub main: MainScreen,
    pub subscriptions: SubscriptionSet<TypeId, Box<dyn Any + 'static>>,
}

pub type Listeners = BTreeMap<TypeId, Vec<SharedComponent>>;
pub type StatefulListeners = BTreeMap<TypeId, Vec<SharedStatefulComponent>>;

impl Screens {
    pub fn new() -> Screens {
        let mut screens = Self {
            active_screen: ActiveScreen::Main,
            main: MainScreen::new(),
            subscriptions: SubscriptionSet::new(),
        };

        let mut listeners = screens.main.listeners.clone().map_value_to_any();
        let stateful_listeners = screens.main.stateful_listeners.clone().map_value_to_any();
        
        listeners.extend(stateful_listeners);
        screens.subscriptions = SubscriptionSet::from(listeners);

        screens
    }

    pub fn get_active_screen_mut(&mut self) -> &mut impl Screen {
        match self.active_screen {
            ActiveScreen::Main => &mut self.main,
        }
    }

    pub fn notify(&mut self, id: TypeId, event: Event) {
        debug!("Notifying event {:?} to TypeId {:?}", event, id);
        if let Some(subscriptions) = self.subscriptions.get(&id) {
            debug!(
                "Found listeners for TypeId {:?} - Count: {}",
                id,
                subscriptions.len()
            );

            for sub in subscriptions {
                if sub.is_stateful {
                    listen!(sub.listener, SharedStatefulComponent, event);
                } else {
                    listen!(sub.listener, SharedComponent, event);
                }
            }
        } else {
            debug!("No listeners found for TypeId {:?}", id);
        }
    }
}

pub trait Screen {
    fn new() -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<SharedComponent>>;

    fn get_stateful_listeners(&self) -> &BTreeMap<TypeId, Vec<SharedStatefulComponent>>;
}

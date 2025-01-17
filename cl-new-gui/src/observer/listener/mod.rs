mod list_listener;
mod tabs_listener;
mod textbox_listener;

use crate::SharedCell;

pub struct Listener<C> {
    component: SharedCell<C>,
}

impl<C> Listener<C>
where
    C: Observable,
{
    pub fn new(component: C) -> Self {
        Self {
            component: SharedCell::new(component),
        }
    }

    pub fn listen(&mut self, event: C::EventType) {
        let mut component = self.component.borrow_mut();

        component.on_listen(event);
    }
}

impl<T> From<SharedCell<T>> for Listener<T> {
    fn from(value: SharedCell<T>) -> Self {
        Self { component: value }
    }
}

pub trait Observable {
    type EventType: Clone;

    fn on_listen(&mut self, event: Self::EventType);
}

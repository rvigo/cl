use crate::component::{Popup, Component};
use crate::screen::layer::Layer;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::Frame;

pub struct PopupLayer {
    pub popup: Component,
    pub listeners: BTreeMap<TypeId, Vec<Component>>,
}

impl Layer for PopupLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let popup = Popup::default();
        let shared = Component::new(popup);

        let mut listeners = BTreeMap::new();

        listeners.insert(TypeId::of::<Popup>(), vec![shared.clone()]);

        Self {
            popup: shared,
            listeners,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        self.popup.borrow_mut().render(frame, frame.size());
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>> {
        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Popup>(), vec![self.popup.clone()]);

        listeners
    }
}

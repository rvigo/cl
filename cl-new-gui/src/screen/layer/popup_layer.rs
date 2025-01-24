use crate::component::{Popup, SharedComponent};
use crate::screen::layer::Layer;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::Frame;

pub struct PopupLayer {
    pub popup: SharedComponent,
    pub listeners: BTreeMap<TypeId, Vec<SharedComponent>>,
}

impl Layer for PopupLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let popup = Popup::default();
        let shared = SharedComponent::new(popup);

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

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<SharedComponent>> {
        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Popup>(), vec![self.popup.clone()]);

        listeners
    }
}

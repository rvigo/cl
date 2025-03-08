use crate::component::{Component, Popup, Renderable};
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
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

    // TODO adjust theme
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        self.popup.render(frame, frame.area(), theme);
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>> {
        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Popup>(), vec![self.popup.clone()]);

        listeners
    }
}

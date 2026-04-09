use crate::component::{Component, Popup, RenderableComponent};
use crate::observer::observable::Observable;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::Frame;

pub struct PopupLayer {
    pub popup: RenderableComponent<Popup>,
    pub listeners: BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>,
}

impl Default for PopupLayer {
    fn default() -> Self {
        let popup = Popup::default();
        let shared = RenderableComponent(Component::new(popup));

        let mut listeners = BTreeMap::new();

        listeners.insert(TypeId::of::<Popup>(), vec![shared.get_observable()]);

        Self {
            popup: shared,
            listeners,
        }
    }
}

impl Layer for PopupLayer {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        self.popup.render(frame, frame.area(), theme);
    }

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        &self.listeners
    }
}

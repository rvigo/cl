use crate::component::{Component, Popup, RenderableComponent};
use crate::observer::observable::Observable;
use crate::screen::key_mapping::command::ScreenCommand;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use crossterm::event::KeyEvent;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
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
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        self.map_key_event(key, state_tx)
    }

    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        self.popup.render(frame, frame.area(), theme);
    }

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        &self.listeners
    }
}

mod button;
mod clipboard_status;
mod list;
mod popup;
mod renderable;
mod search;
mod static_info;
mod tabs;
mod textbox;

pub use clipboard_status::ClipboardStatus;
pub use list::List;
pub use popup::Popup;
pub use renderable::Renderable;
pub use search::Search;
pub use static_info::StaticInfo;
pub use tabs::Tabs;
pub use textbox::TextBox;
pub use textbox::TextBoxName;

use crate::observer::ObservableComponent;
use crate::screen::theme::Theme;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use tui::layout::Rect;
use tui::Frame;

#[derive(Debug)]
pub struct Component(pub Rc<RefCell<dyn ObservableComponent + 'static>>);

impl Component {
    pub fn new(component: impl ObservableComponent + 'static) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }

    pub fn borrow(&self) -> Ref<dyn ObservableComponent> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<dyn ObservableComponent> {
        self.0.borrow_mut()
    }
}

impl Clone for Component {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self))
    }
}

impl Deref for Component {
    type Target = Rc<RefCell<dyn ObservableComponent + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Renderable for Component {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        self.borrow_mut().render(frame, area, theme)
    }
}

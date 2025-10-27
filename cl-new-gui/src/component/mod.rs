mod button;
mod clipboard_status;
mod editable_textbox;
mod list;
mod popup;
mod renderable;
mod search;
mod static_info;
mod tabs;
mod textbox;
mod screen_state;

pub use clipboard_status::ClipboardStatus;
pub use editable_textbox::EditableTextbox;
pub use editable_textbox::EditableTextboxName;
pub use list::List;
pub use popup::Popup;
pub use renderable::Renderable;
pub use screen_state::ScreenState;
pub use search::Search;
pub use static_info::StaticInfo;
use std::any::Any;
pub use tabs::Tabs;
pub use textbox::TextBox;
pub use textbox::TextBoxName;

use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::screen::theme::Theme;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use tui::layout::Rect;
use tui::Frame;

#[derive(Debug)]
pub struct Component<T: ?Sized + Observable>(pub Rc<RefCell<T>>);

impl<T> Component<T>
where
    T: 'static + Observable,
{
    pub fn new(component: T) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }
}

impl<T> Clone for Component<T>
where
    T: Observable + Clone,
{
    fn clone(&self) -> Self {
        Self(Rc::clone(self))
    }
}

impl<T> Deref for Component<T>
where
    T: Observable,
{
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Component<T>
where
    T: Observable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Component<T>
where
    T: Observable + Debug + Any,
{
    pub fn borrow(&self) -> Ref<dyn ObservableComponent> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<dyn ObservableComponent> {
        self.0.borrow_mut()
    }

    /// Returns a Ref to the inner value as a dyn Observable
    pub fn get_observable(&self) -> Rc<RefCell<dyn Observable>> {
        self.0.clone()
    }

    /// Returns a RefMut to the inner value as a dyn Observable
    pub fn get_observable_mut(&self) -> RefMut<dyn Observable> {
        RefMut::map(self.0.borrow_mut(), |inner| inner as &mut dyn Observable)
    }
}

//TODO create new constructor for RenderableComponent that takes T directly
pub struct RenderableComponent<T: Renderable + ObservableComponent>(pub Component<T>);

impl<T> RenderableComponent<T>
where
    T: Renderable + ObservableComponent,
{
    pub fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        self.borrow_mut()
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
            .render(frame, area, theme);
    }
}

impl<T> Deref for RenderableComponent<T>
where
    T: Renderable + ObservableComponent,
{
    type Target = Component<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct StateComponent<T: ObservableComponent>(pub Component<T>);

impl<T> Deref for StateComponent<T>
where
    T: ObservableComponent,
{
    type Target = Component<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

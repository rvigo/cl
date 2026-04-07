mod button;
mod clipboard_status;
mod editable_textbox;
mod list;
mod popup;
mod renderable;
mod screen_state;
mod search;
mod static_info;
pub mod table;
mod tabs;
mod textbox;

pub use button::FutureEventType;
pub use clipboard_status::ClipboardStatus;
pub use editable_textbox::EditableTextbox;
pub use list::List;
pub use popup::Popup;
pub use renderable::Renderable;
pub use screen_state::ScreenState;
pub use search::Search;
pub use static_info::StaticInfo;
use std::any::Any;
pub use tabs::Tabs;
pub use textbox::TextBox;

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
    T: Observable,
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
    /// Borrow the inner value as a type-erased `ObservableComponent`.
    pub fn as_observable(&self) -> Ref<dyn ObservableComponent> {
        self.0.borrow()
    }

    /// Mutably borrow the inner value as a type-erased `ObservableComponent`.
    pub fn as_observable_mut(&self) -> RefMut<dyn ObservableComponent> {
        self.0.borrow_mut()
    }

    /// Borrow the inner `T` directly, with no downcast required.
    ///
    /// Prefer this over [`borrow`](Self::borrow) + downcast whenever the
    /// concrete type is known (e.g. reading `ScreenState` fields from
    /// `FormScreenLayer`).
    pub fn borrow_inner(&self) -> Ref<T> {
        self.0.borrow()
    }

    /// Mutably borrow the inner `T` directly, with no downcast required.
    pub fn borrow_inner_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    /// Returns a shared `Rc<RefCell<dyn Observable>>` for use in subscription
    /// registries.
    pub fn get_observable(&self) -> Rc<RefCell<dyn Observable>> {
        self.0.clone()
    }

    /// Returns a `RefMut<dyn Observable>` for the inner value.
    pub fn get_observable_mut(&self) -> RefMut<dyn Observable> {
        RefMut::map(self.0.borrow_mut(), |inner| inner as &mut dyn Observable)
    }
}

// ---------------------------------------------------------------------------
// Wrapper types
// ---------------------------------------------------------------------------

pub struct RenderableComponent<T: Renderable + ObservableComponent>(pub Component<T>);

impl<T> RenderableComponent<T>
where
    T: Renderable + ObservableComponent,
{
    pub fn new(component: T) -> Self {
        Self(Component::new(component))
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        self.borrow_inner_mut().render(frame, area, theme);
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

impl<T> StateComponent<T>
where
    T: ObservableComponent,
{
    pub fn new(component: T) -> Self {
        Self(Component::new(component))
    }
}

impl<T> Deref for StateComponent<T>
where
    T: ObservableComponent,
{
    type Target = Component<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// Downcastable (kept for rare cases where the concrete type is not known)
// ---------------------------------------------------------------------------

pub trait Downcastable {
    fn downcast_to<T: Any>(&self) -> Option<&T>;
}

impl Downcastable for dyn ObservableComponent {
    fn downcast_to<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::ScreenState;
    use crate::state::state_event::FieldName;

    #[test]
    fn borrow_inner_returns_correct_value() {
        let state = ScreenState::new(FieldName::Command);
        let comp = StateComponent::new(state);

        assert_eq!(comp.borrow_inner().current_field, FieldName::Command);
    }

    #[test]
    fn borrow_inner_mut_can_mutate() {
        let state = ScreenState::new(FieldName::Alias);
        let comp = StateComponent::new(state);

        comp.borrow_inner_mut().current_field = FieldName::Tags;
        assert_eq!(comp.borrow_inner().current_field, FieldName::Tags);
    }

    #[test]
    fn borrow_inner_does_not_panic_unlike_old_downcast() {
        // Previously get_current_field() would unwrap() a downcast; now
        // borrow_inner() is always safe when T is the correct type.
        let state = ScreenState::new(FieldName::Namespace);
        let comp = StateComponent::new(state);
        let field = comp.borrow_inner().current_field.clone();
        assert_eq!(field, FieldName::Namespace);
    }
}

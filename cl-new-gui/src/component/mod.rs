mod button;
mod list;
mod popup;
mod shared_component;
mod shared_stateful_component;
mod tabs;
mod textbox;

use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use tui::layout::Rect;
use tui::Frame;

pub use list::List;
pub use popup::Popup;
pub use shared_component::SharedComponent;
pub use shared_stateful_component::SharedStatefulComponent;
pub use tabs::Tabs;
pub use textbox::TextBox;
pub use textbox::TextBoxName;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub trait StatefulComponent {
    fn render_stateful(&mut self, frame: &mut Frame, area: Rect);
}

pub trait AnyComponent: Any {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl AnyComponent for SharedComponent {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl AnyComponent for SharedStatefulComponent {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait BTreeMapExt {
    fn map_value_to_any(self) -> BTreeMap<TypeId, Vec<Box<dyn Any>>>;
}

impl<V: AnyComponent + Clone> BTreeMapExt for BTreeMap<TypeId, Vec<V>> {
    fn map_value_to_any(self) -> BTreeMap<TypeId, Vec<Box<dyn Any>>> {
        self.into_iter()
            .map(|(key, value)| {
                (
                    key,
                    value
                        .into_iter()
                        .map(|item| AnyComponent::into_any(Box::new(item)))
                        .collect::<Vec<Box<dyn Any + 'static>>>(),
                )
            })
            .collect::<BTreeMap<TypeId, Vec<Box<dyn Any>>>>()
    }
}

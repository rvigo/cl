mod main_screen_layer;
mod popup_layer;

pub use main_screen_layer::MainScreenLayer;
pub use popup_layer::PopupLayer;

use crate::component::Component;
use crate::screen::key_mapping::KeyMapping;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::Frame;

pub trait Layer: KeyMapping {
    fn new() -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>>;
}

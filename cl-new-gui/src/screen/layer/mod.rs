mod edit_screen_layer;
mod main_screen_layer;
mod popup_layer;
mod quick_search_layer;

pub use edit_screen_layer::EditScreenLayer;
pub use main_screen_layer::MainScreenLayer;
pub use popup_layer::PopupLayer;
pub use quick_search_layer::QuickSearchLayer;

use crate::observer::observable::Observable;
use crate::screen::key_mapping::KeyMapping;
use crate::screen::theme::Theme;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::Frame;

pub trait Layer: KeyMapping {
    fn new() -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame, theme: &Theme);

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>;
}

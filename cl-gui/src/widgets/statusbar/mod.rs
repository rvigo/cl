pub mod help;
pub mod info;
pub mod navigation_info;
pub mod querybox;

use tui::widgets::Widget;

pub trait StatusBarItem: Clone + Widget {}

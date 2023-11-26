use tui::widgets::Widget;

pub mod help;
pub mod navigation_info;
pub mod querybox;

pub trait StatusBarItem: Clone + Widget {}

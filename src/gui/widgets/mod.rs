pub mod base_widget;
pub mod display;
pub mod fields;
pub mod help_footer;
pub mod help_popup;
pub mod highlight;
pub mod list;
pub mod navigation_footer;
pub mod popup;
pub mod querybox;
pub mod text_field;

use tui::widgets::Widget;

pub trait Footer: Clone + Widget {}

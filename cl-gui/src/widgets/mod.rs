pub mod alias_list;
pub mod base_widget;
pub mod display;
pub mod fields;
pub mod highlight;
pub mod macros;
pub mod popup;
pub mod statusbar;
pub mod text_field;

use self::statusbar::StatusBarItem;
use crossterm::event::KeyEvent;

/// Handles use key input
pub trait WidgetKeyHandler {
    fn handle_input(&mut self, input: KeyEvent);
}

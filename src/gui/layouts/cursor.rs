use crate::gui::contexts::context::Item;
use tui::{backend::Backend, layout::Rect, Frame};
use unicode_width::UnicodeWidthStr;

pub fn set_cursor_positition<B: Backend>(frame: &mut Frame<B>, item: &Item, area: Rect) {
    frame.set_cursor(area.x + item.input.width() as u16 + 1, area.y + 1);
}

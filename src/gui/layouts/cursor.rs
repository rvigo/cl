use crate::gui::entities::field::Field;
use tui::{backend::Backend, layout::Rect, Frame};
use unicode_width::UnicodeWidthStr;

pub fn set_cursor_positition<B: Backend>(frame: &mut Frame<B>, item: &Field, area: Rect) {
    let y: u16 = area.y + 1 + (item.input.width() as u16 / (area.width - 2));
    let x: u16 = area.x + 1 + (item.input.width() as u16 % (area.width - 2));

    frame.set_cursor(x, y);
}

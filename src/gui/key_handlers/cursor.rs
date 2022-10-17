use crate::gui::entities::field::Field;
use tui::{backend::Backend, layout::Rect, Frame};

pub fn set_cursor_positition<B: Backend>(frame: &mut Frame<B>, field: &Field, area: Rect) {
    let y: u16 = area.y + 1 + (field.cursor_position() / (area.width - 2));
    let x: u16 = area.x + 1 + (field.cursor_position() % (area.width - 2));

    frame.set_cursor(x, y);
}

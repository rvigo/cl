use crate::gui::contexts::state::State;
use tui::{backend::Backend, layout::Rect, Frame};
use unicode_width::UnicodeWidthStr;

pub fn set_cursor_positition<B: Backend>(frame: &mut Frame<B>, state: &mut State, area: Rect) {
    frame.set_cursor(
        area.x + state.context.get_current_in_focus().input.width() as u16 + 1,
        area.y + 1,
    );
}

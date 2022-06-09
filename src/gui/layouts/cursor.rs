use tui::{backend::Backend, layout::Rect, Frame};

use unicode_width::UnicodeWidthStr;

use crate::gui::structs::state::State;

pub fn set_cursor_positition<B: Backend>(frame: &mut Frame<B>, state: &mut State, area: Rect) {
    frame.set_cursor(
        // Put cursor past the end of the input text
        area.x + state.focus.get_current_in_focus().input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        area.y + 1,
    );
}

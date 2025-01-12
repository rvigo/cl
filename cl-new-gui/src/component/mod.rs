mod list;
mod textbox;

pub use list::List;
pub use textbox::TextBox;
pub use textbox::TextBoxName;

use tui::layout::Rect;
use tui::Frame;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub trait StatefulComponent {
    fn render_stateful(&mut self, frame: &mut Frame, area: Rect);
}

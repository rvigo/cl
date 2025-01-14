mod list;
mod textbox;
mod tabs;

pub use list::List;
pub use textbox::TextBox;
pub use textbox::TextBoxName;
pub use tabs::Tabs;

use tui::layout::Rect;
use tui::Frame;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub trait StatefulComponent {
    fn render_stateful(&mut self, frame: &mut Frame, area: Rect);
}

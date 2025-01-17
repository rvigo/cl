mod button;
mod list;
mod popup;
mod tabs;
mod textbox;

pub use list::List;
pub use tabs::Tabs;
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

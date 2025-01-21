mod button;
mod list;
mod popup;
mod shared_component;
mod tabs;
mod textbox;

use tui::layout::Rect;
use tui::Frame;

pub use button::Button;
pub use list::List;
pub use popup::Popup;
pub use shared_component::SharedComponent;
pub use tabs::Tabs;
pub use textbox::TextBox;
pub use textbox::TextBoxName;

pub trait Component {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}

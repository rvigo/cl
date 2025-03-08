use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::Frame;

pub trait Renderable {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme);
}

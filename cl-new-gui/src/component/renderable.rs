use tui::Frame;
use tui::layout::Rect;

pub trait Renderable {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}

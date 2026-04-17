use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::Frame;

pub trait Renderable {
    /// Called before render to update state (e.g. timers). Default is no-op.
    fn pre_render(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme);
}

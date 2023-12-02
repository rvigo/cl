use super::StatusBarItem;
use crate::widgets::display::DisplayWidget;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone)]
pub struct NavigationInfo {
    content: String,
}

impl StatusBarItem for NavigationInfo {}

impl NavigationInfo {
    pub fn new() -> NavigationInfo {
        Self {
            content: String::from("<Tab> Rigth <S+Tab> Left <C+S> Save <Esc> Return"),
        }
    }
}

impl Widget for NavigationInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display = DisplayWidget::new(&self.content, true, false);

        display.render(area, buf)
    }
}

use super::StatusBarItem;
use crate::screens::widgets::display::DisplayWidget;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone)]
pub struct NavigationInfo {
    content: String,
}

impl StatusBarItem for NavigationInfo {}

impl NavigationInfo {
    pub fn new() -> NavigationInfo {
        Self {
            content: String::from("<Tab> Move rigth | <Shift + Tab> Move left | <Ctrl + S> Save item | <Esc> Return to main screen" ),
        }
    }
}

impl Widget for NavigationInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display = DisplayWidget::new(&self.content, true, false);

        display.render(area, buf)
    }
}

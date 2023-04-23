use super::{display::DisplayWidget, Footer, WidgetExt};
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone)]
pub struct NavigationFooter {
    content: String,
}

impl Footer for NavigationFooter {}

impl NavigationFooter {
    pub fn new() -> NavigationFooter {
        Self {
            content: String::from("<Tab> Move rigth | <Shift + Tab> Move left | <Ctrl + S> Save item | <Esc / Ctrl + Q> Return to main screen" ),
        }
    }
}

impl Widget for NavigationFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = "Navigation";
        let display =
            DisplayWidget::new(&self.content, true, false).block(self.default_block(title));

        display.render(area, buf)
    }
}

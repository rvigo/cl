use crate::widget::{display::DisplayWidget, text_field::FieldType};
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone)]
pub struct NavigationInfo {
    content: String,
}

impl NavigationInfo {
    pub fn new() -> NavigationInfo {
        Self {
            content: String::from("<Tab> Rigth <S-Tab> Left <C-S> Save <Esc> Return"),
        }
    }
}

impl Widget for NavigationInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display = DisplayWidget::new(FieldType::Info, &self.content, true, false);

        display.render(area, buf)
    }
}

impl Default for NavigationInfo {
    fn default() -> Self {
        Self::new()
    }
}

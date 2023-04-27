use super::{display::DisplayWidget, Footer, WidgetExt};
use crate::gui::entities::states::vi_state::ViState;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone)]
pub struct ViFooter {
    state: ViState,
}

impl ViFooter {
    pub fn new(state: &mut ViState) -> ViFooter {
        Self {
            state: state.to_owned(),
        }
    }
}

impl Footer for ViFooter {}

impl Widget for ViFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = "Mode";
        let display = DisplayWidget::new(&self.state.mode().to_string(), true, false)
            .block(self.default_block(title));

        display.render(area, buf)
    }
}

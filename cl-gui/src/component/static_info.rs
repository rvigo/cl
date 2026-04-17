use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

#[derive(Clone, Default)]
pub struct StaticInfo {
    pub content: String,
}

impl StaticInfo {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Renderable for StaticInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let paragraph = Paragraph::new(self.content.as_str())
            .block(
                Block::bordered().style(
                    Style::default()
                        .fg(theme.text_color.into())
                        .bg(theme.background_color.into()),
                ),
            )
            .alignment(Center);

        frame.render_widget(paragraph, area);
    }
}

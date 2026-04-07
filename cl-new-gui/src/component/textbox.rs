use crate::component::Renderable;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldName;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct TextBox {
    pub name: FieldName,
    pub content: Option<String>,
    pub placeholder: Option<String>,
}

impl TextBox {
    pub fn update_content(&mut self, content: Option<impl Into<String>>) {
        self.content = content.map(|content| content.into());
    }
}

impl Renderable for TextBox {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        fn match_content(content: Option<String>, fallback: Option<String>) -> String {
            if let Some(c) = content {
                if !c.is_empty() {
                    return c;
                }
            }
            fallback.unwrap_or_default()
        }
        let content = match_content(self.content.clone(), self.placeholder.clone());

        let style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());

        let paragraph = Paragraph::new(content).block(Block::bordered().style(style));

        frame.render_widget(paragraph, area)
    }
}

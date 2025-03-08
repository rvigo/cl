use crate::component::Renderable;
use crate::screen::theme::Theme;
use std::fmt::Display;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TextBox {
    pub name: TextBoxName,
    pub content: Option<String>,
}

impl TextBox {
    pub fn update_content(&mut self, content: Option<impl Into<String>>) {
        self.content = content.map(|content| content.into());
    }
}

impl Renderable for TextBox {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        let content = match &self.content {
            None => "",
            Some(c) => &c,
        };
        let style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());
        let paragraph = Paragraph::new(content).block(Block::bordered().style(style));

        frame.render_widget(paragraph, area)
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TextBoxName {
    #[default]
    Command,
    Description,
    Tags,
    Namespace,
}

impl Display for TextBoxName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextBoxName::Command => write!(f, "Command"),
            TextBoxName::Description => write!(f, "Description"),
            TextBoxName::Tags => write!(f, "Tags"),
            TextBoxName::Namespace => write!(f, "Namespace"),
        }
    }
}

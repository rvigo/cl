use std::fmt::Display;
use crate::component::Component;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

#[derive(Default, Clone)]
pub struct TextBox {
    pub name: TextBoxName,
    pub content: String,
}

impl TextBox {
    pub fn update_content(&mut self, content: String) {
        self.content = content
    }
}

impl Component for TextBox {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(format!("{:#?}", self.content))
            .block(Block::default().borders(Borders::all()));

        frame.render_widget(paragraph, area)
    }
}

#[derive(Default, Clone, Debug)]
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

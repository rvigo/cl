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
    pub show_title: bool,
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

        let block = if self.show_title {
            Block::bordered().style(style).title(self.name.to_string())
        } else {
            Block::bordered().style(style)
        };
        let paragraph = Paragraph::new(content).block(block);

        frame.render_widget(paragraph, area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_no_title_shown() {
        let tb = TextBox::default();
        assert_eq!(tb.name, FieldName::Alias);
        assert_eq!(tb.content, None);
        assert_eq!(tb.placeholder, None);
        assert!(!tb.show_title);
    }

    #[test]
    fn show_title_can_be_enabled() {
        let tb = TextBox {
            name: FieldName::Command,
            show_title: true,
            ..Default::default()
        };
        assert!(tb.show_title);
        assert_eq!(tb.name, FieldName::Command);
    }

    #[test]
    fn update_content_sets_value() {
        let mut tb = TextBox::default();
        tb.update_content(Some("hello"));
        assert_eq!(tb.content, Some("hello".to_owned()));
    }

    #[test]
    fn update_content_clears_with_none() {
        let mut tb = TextBox {
            content: Some("existing".to_owned()),
            ..Default::default()
        };
        tb.update_content(None::<String>);
        assert_eq!(tb.content, None);
    }
}

impl crate::observer::event::NotifyTarget for TextBox {
    type Payload = crate::observer::event::TextBoxEvent;
    fn wrap(payload: Self::Payload) -> crate::observer::event::Event {
        crate::observer::event::Event::TextBox(payload)
    }
}

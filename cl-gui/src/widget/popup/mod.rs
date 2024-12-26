mod choice;
mod macros;
mod popup_type;

pub use choice::Choice;
pub use popup_type::Type;

use super::tabs::Tabs;
use crate::{
    centered_rect,
    context::PopupContext,
    default_popup_block,
    event::PopupCallbackAction,
    theme::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR},
};
use std::{ops::Deref, rc::Rc};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct Content(Vec<String>);

impl Deref for Content {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for Content {
    fn from(content: Vec<String>) -> Self {
        Self(content)
    }
}

impl From<String> for Content {
    fn from(content: String) -> Self {
        Self(vec![content])
    }
}

#[derive(Clone, Debug)]
pub struct Popup {
    content: Content,
    pub choices: Vec<Choice>,
    pub r#type: Type,
    pub callback: PopupCallbackAction,
}

impl Popup {
    pub fn new(
        content: impl Into<Content>,
        choices: Vec<Choice>,
        r#type: Type,
        callback: PopupCallbackAction,
    ) -> Popup {
        Self {
            content: content.into(),
            choices,
            r#type,
            callback,
        }
    }

    fn button_widget(&self, selected: usize) -> Tabs<'_> {
        let choices = self
            .choices
            .iter()
            .map(|choice| choice.to_string())
            .collect();

        Tabs::new(choices)
            .block(Block::default().borders(Borders::NONE))
            .select(selected)
            .highlight_style(
                Style::default()
                    .fg(DEFAULT_SELECTED_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(' ')
    }

    fn create_buttom_area(&self, area: Rect) -> Rect {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(self.create_buttom_layout(area)[4]);

        let constraints = if self.choices.len() == 2 {
            vec![Constraint::Min(50)]
        } else {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        };
        let buttom_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[0]);

        buttom_area[buttom_area.len() - 1]
    }

    fn create_buttom_layout(&self, area: Rect) -> Rc<[Rect]> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Length(5), //keeps the options inside the box
            ])
            .split(layout[3])
    }

    fn content_width(&self) -> u16 {
        self.content
            .iter()
            .map(|line| line.width())
            .max()
            .unwrap_or(0) as u16
    }

    fn content_height(&self) -> u16 {
        const MIN_HEIGHT: usize = 5;

        let lines = self.content.len();
        MIN_HEIGHT.max(lines) as u16
    }

    fn get_popup_area(&self, area: Rect) -> Rect {
        let content_width = self.content_width();
        let content_height = self.content_height();

        const SCALE_FACTOR: u16 = 100;
        const MAX_SCALE_RATIO: f32 = 2.0;

        let scaled_height = (SCALE_FACTOR * (content_height * 2)) / area.height;
        let max_height = (area.height as f32 * MAX_SCALE_RATIO) as u16;
        let final_height = std::cmp::min(scaled_height, max_height) as u16;

        centered_rect!(content_width, final_height, area)
    }
}

impl Widget for Popup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self, area, buf, &mut PopupContext::default());
    }
}

impl StatefulWidget for Popup {
    type State = PopupContext;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut PopupContext) {
        let block = default_popup_block!(self.r#type);
        let content = self.content.join("\n");
        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block.to_owned());

        let popup_area = self.get_popup_area(area);

        Clear::render(Clear, popup_area, buf);
        paragraph.render(popup_area, buf);

        let options = self.button_widget(state.selected_choice_idx());
        let buttom_area = self.create_buttom_area(popup_area);
        options.render(buttom_area, buf);
    }
}

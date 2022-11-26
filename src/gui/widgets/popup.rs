use std::fmt;
use tui::widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Tabs,
};

use crate::gui::layouts::{centered_rect, DEFAULT_TEXT_COLOR};

#[derive(Clone, Debug)]
pub enum MessageType {
    Error,
    Delete,
}
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::Error => write!(f, " Error "),
            MessageType::Delete => write!(f, " Warning "),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Answer {
    Ok,
    Cancel,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::Ok => write!(f, "Ok"),
            Answer::Cancel => write!(f, "Cancel"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Popup<'a> {
    message: String,
    pub message_type: Option<MessageType>,
    pub choices: Vec<Answer>,
    block: Option<Block<'a>>,
}

impl<'a> Popup<'a> {
    pub fn new<MESSAGE, TITLE>(
        message: MESSAGE,
        title: TITLE,
        message_type: Option<MessageType>,
    ) -> Popup<'a>
    where
        MESSAGE: Into<String>,
        TITLE: Into<String>,
    {
        Popup {
            message: message.into(),
            message_type: message_type.clone(),
            choices: if let Some(msg_type) = message_type {
                match msg_type {
                    MessageType::Error => vec![Answer::Ok],
                    MessageType::Delete => vec![Answer::Cancel, Answer::Ok],
                }
            } else {
                vec![]
            },
            block: Some(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(format!(" {} ", title.into()))
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Plain),
            ),
        }
    }

    pub fn clear(&mut self) {
        self.message.clear();
        self.message_type = None;
        self.choices = vec![];
    }

    fn create_buttom_area(&self, area: Rect) -> Vec<Rect> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(self.create_buttom_layout(area)[4]);

        let constraints = if self.choices.len() == 2 {
            vec![Constraint::Min(50)]
        } else {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        };
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(layout[0])
    }

    //TODO center buttons in the popup
    // uses the lower right space to render buttons
    fn create_buttom_layout(&self, area: Rect) -> Vec<Rect> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Length(3), //keeps the options inside the box
                ]
                .as_ref(),
            )
            .split(layout[3])
    }
}

#[derive(Default)]
pub struct ChoicesState {
    offset: usize,
    selected: Option<usize>,
}

impl ChoicesState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn next(&mut self, choices: Vec<Answer>) {
        let i = match self.selected() {
            Some(i) => {
                if i >= choices.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.select(Some(i));
    }

    pub fn previous(&mut self, choices: Vec<Answer>) {
        let i = match self.selected() {
            Some(i) => {
                if i == 0 {
                    choices.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.select(Some(i));
    }
}

impl<'a> StatefulWidget for Popup<'a> {
    type State = ChoicesState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let block = self.block.as_ref().unwrap();
        let p = Paragraph::new(self.message.clone())
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block.to_owned());

        let area = centered_rect(45, 40, area);
        let buttom_area = self.create_buttom_area(area);
        Clear::render(Clear, area, buf);

        p.render(area, buf);

        let tab_menu: Vec<Spans> = self
            .choices
            .iter()
            .map(|tab| Spans::from(vec![Span::styled(tab.to_string(), Style::default())]))
            .collect();

        let tabs = Tabs::new(tab_menu)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default())
            .select(state.selected().unwrap_or(0))
            .highlight_style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(Span::raw(""));
        tabs.render(buttom_area[buttom_area.len() - 1], buf);
    }
}

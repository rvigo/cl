use super::WidgetExt;
use crate::gui::{
    entities::{
        events::app_events::PopupCallbackAction,
        states::{answer_state::AnswerState, State},
    },
    DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR,
};
use log::{error, warn};
use std::fmt;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};

#[derive(Clone, Debug)]
pub struct Popup {
    message: String,
    message_type: Option<MessageType>,
    choices: Vec<Answer>,
    callback_action: PopupCallbackAction,
}

impl Popup {
    pub fn from_error<T>(message: T, additional_info: Option<T>) -> Popup
    where
        T: Into<String>,
    {
        let message = if let Some(ai) = additional_info {
            format!("{}\n\n{}", message.into(), ai.into())
        } else {
            message.into()
        };
        error!("{message}");

        Popup {
            message,
            message_type: Some(MessageType::Error),
            choices: vec![Answer::Ok],

            callback_action: PopupCallbackAction::None,
        }
    }

    pub fn from_warning<T>(message: T, callback_action: PopupCallbackAction) -> Popup
    where
        T: Into<String>,
    {
        let message = message.into();
        warn!("{message}");

        Popup {
            message,
            message_type: Some(MessageType::Warning),
            choices: vec![Answer::Cancel, Answer::Ok],

            callback_action,
        }
    }
    pub fn callback(&self) -> PopupCallbackAction {
        self.callback_action.to_owned()
    }

    pub fn choices(&self) -> Vec<Answer> {
        self.choices.clone()
    }

    pub fn message_type(&self) -> Option<MessageType> {
        self.message_type.clone()
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

impl WidgetExt for Popup {}

impl StatefulWidget for Popup {
    type State = AnswerState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let messate_type_string = if let Some(message_type) = &self.message_type {
            message_type.to_string()
        } else {
            String::default()
        };
        let block = self.default_block(messate_type_string);
        let p = Paragraph::new(self.message.clone())
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block.to_owned());

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
                    .fg(DEFAULT_SELECTED_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(Span::raw(""));
        tabs.render(buttom_area[buttom_area.len() - 1], buf);
    }
}

impl Drop for Popup {
    fn drop(&mut self) {
        self.message.clear();
        self.message_type = None;
        self.choices = vec![];
    }
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Error,
    Warning,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::Error => write!(f, " Error "),
            MessageType::Warning => write!(f, " Warning "),
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

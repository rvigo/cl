pub mod choice;
pub mod help_popup;
pub mod popup_type;
pub mod question_popup;

use self::choice::Choice;
use crate::{
    centered_rect, entities::contexts::popup_context::PopupContext, DEFAULT_SELECTED_COLOR,
};
use std::{rc::Rc, vec};
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub trait Popup
where
    Self: Sized + Send + Sync,
{
    fn render(self, area: Rect, buf: &mut Buffer, state: Option<&mut PopupContext>);

    fn content_height(&self) -> u16;

    fn content_width(&self) -> u16;

    fn choices(&self) -> Vec<Choice> {
        Choice::empty()
    }

    fn get_render_position(&self, area: Rect) -> Rect {
        let width = self.content_width();
        let height = self.content_height();

        let dynamic_height = (100 * (height * 2)) / area.height;
        let real_height = std::cmp::max(dynamic_height, area.height);
        centered_rect!(width, real_height, area)
    }
}

pub trait WithChoices: Popup {
    fn button_widget(&self, selected: usize) -> Tabs<'_> {
        let choices = self
            .choices()
            .iter()
            .map(|tab| Line::from(tab.to_string()))
            .collect();

        Tabs::new(choices)
            .block(Block::default().borders(Borders::NONE))
            .select(selected)
            .highlight_style(
                Style::default()
                    .fg(DEFAULT_SELECTED_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(Span::raw(""))
    }

    fn create_buttom_area(&self, area: Rect) -> Rect {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(self.create_buttom_layout(area)[4]);

        let constraints = if self.choices().len() == 2 {
            vec![Constraint::Min(50)]
        } else {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        };
        let buttom_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(layout[0]);

        buttom_area[buttom_area.len() - 1]
    }

    fn create_buttom_layout(&self, area: Rect) -> Rc<[Rect]> {
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

pub trait RenderPopup {
    fn render_popup<P>(&mut self, popup: P, area: Rect)
    where
        P: Popup;

    fn render_stateful_popup<P>(&mut self, popup: P, area: Rect, state: &mut PopupContext)
    where
        P: Popup;
}

impl RenderPopup for Frame<'_> {
    fn render_popup<P>(&mut self, popup: P, area: Rect)
    where
        P: Popup,
    {
        popup.render(area, self.buffer_mut(), None);
    }

    fn render_stateful_popup<P>(&mut self, popup: P, area: Rect, state: &mut PopupContext)
    where
        P: Popup,
    {
        popup.render(area, self.buffer_mut(), Some(state))
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! popup {
        ($view_mode:expr) => {
            HelpPopup::new($view_mode)
        };

        ($info:expr, $choiches:expr) => {{
            use $crate::widgets::popup::question_popup::QuestionPopup;

            QuestionPopup::new(
                $info.message.to_owned(),
                $choiches,
                $info.popup_type.to_owned(),
            )
        }};
    }
}

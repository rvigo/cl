use super::Choice;
use crate::{centered_rect, theme::DEFAULT_SELECTED_COLOR};
use std::{rc::Rc, vec};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
};

pub trait PopupTrait
where
    Self: Send + Sync,
{
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

pub trait WithChoices: PopupTrait {
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
            .constraints([Constraint::Percentage(100)])
            .split(self.create_buttom_layout(area)[4]);

        let constraints = if self.choices().len() == 2 {
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
                Constraint::Length(3), //keeps the options inside the box
            ])
            .split(layout[3])
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! popup {
        ($view_mode:expr) => {{
            use $crate::widget::popup::HelpPopup;
            HelpPopup::new($view_mode)
        }};

        ($info:expr, $choiches:expr) => {{
            use $crate::widget::popup::P;

            P::new(
                $info.message.to_owned(),
                $choiches,
                $info.popup_type.to_owned(),
            )
        }};
    }

    #[macro_export]
    macro_rules! default_popup_block {
        ($popup_type:expr) => {{
            use tui::{
                layout::Alignment,
                style::{Color, Modifier, Style},
                widgets::{Block, BorderType, Borders, Padding},
            };
            use $crate::theme::{DEFAULT_BACKGROUND_COLOR, DEFAULT_TEXT_COLOR};
            use $crate::widget::popup::Type;

            let style = match $popup_type {
                Type::Error => Style::default()
                    .fg(Color::Rgb(243, 139, 168))
                    .add_modifier(Modifier::BOLD),

                Type::Warning => Style::default()
                    .fg(Color::Rgb(249, 226, 175))
                    .add_modifier(Modifier::BOLD),

                Type::Help => Style::default()
                    .fg(Color::Rgb(166, 227, 161))
                    .add_modifier(Modifier::BOLD),
            };
            Block::default()
                .borders(Borders::ALL)
                .title($popup_type.to_string())
                .title_alignment(Alignment::Left)
                .title_style(style)
                .style(
                    Style::default()
                        .fg(DEFAULT_TEXT_COLOR)
                        .bg(DEFAULT_BACKGROUND_COLOR),
                )
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(2))
        }};
    }
}

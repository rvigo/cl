pub mod base_widget;
pub mod display;
pub mod fields;
pub mod help_popup;
pub mod highlight;
pub mod list;
pub mod popup;
pub mod statusbar;
pub mod text_field;

use self::statusbar::StatusBarItem;
use crate::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};
use crossterm::event::KeyEvent;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

#[macro_export]
macro_rules! centered_rect {
    ($width: expr, $height: expr, $area: expr) => {{
        use tui::layout::{Constraint, Direction, Layout, Rect};

        fn centered_area(width: u16, height: u16, area: Rect) -> Rect {
            let height = if height > 100 { 100 } else { height };
            let width = if width > 100 { 100 } else { width };

            let new_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage((100 - height) / 2),
                        Constraint::Percentage(height),
                        Constraint::Percentage((100 - height) / 2),
                    ]
                    .as_ref(),
                )
                .split(area);
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage((100 - width) / 2),
                        Constraint::Percentage(width),
                        Constraint::Percentage((100 - width) / 2),
                    ]
                    .as_ref(),
                )
                .split(new_area[1])[1]
        }
        centered_area($width, $height, $area)
    }};
}

/// Widget extension functions and defaults
pub trait WidgetExt {
    fn default_block<'a, S>(&self, title: S) -> Block<'a>
    where
        S: Into<String>,
    {
        let title = title.into();
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {title} "))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Plain)
    }

    fn get_style(&self, in_focus: bool) -> Style {
        if in_focus {
            Style::default().fg(Color::Black).bg(DEFAULT_SELECTED_COLOR)
        } else {
            Style::default().fg(DEFAULT_TEXT_COLOR)
        }
    }
}

// Every tui Widget implements this
impl<T> WidgetExt for T where T: Widget {}

/// Handles use key input
pub trait WidgetKeyHandler {
    fn handle_input(&mut self, input: KeyEvent);
}

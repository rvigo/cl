mod base_widget;
pub(super) mod display;
pub mod fields;
pub(super) mod help_footer;
pub(super) mod help_popup;
pub(super) mod highlight;
pub(super) mod list;
pub(super) mod navigation_footer;
pub mod popup;
pub mod querybox;
pub mod text_field;
pub mod vi_footer;

use self::base_widget::BaseWidget;

use super::Screen;
use crate::gui::{entities::states::vi_state::ViState, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};
use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
    Frame,
};

/// Marks the struct as a `Footer`
pub trait Footer: Clone + Widget {}

/// Extension for `Screen`
pub trait ScreenExt<B>: Screen<B>
where
    B: Backend,
{
    fn render_base<F, H>(&self, frame: &mut Frame<B>, footer: Option<&F>, help_footer: H)
    where
        F: Footer,
        H: Footer,
    {
        let screen_size = self.get_screen_size();
        let base_widget = BaseWidget::new(&screen_size, footer, help_footer);
        frame.render_widget(base_widget, frame.size());
    }
}

impl<T, B> ScreenExt<B> for T
where
    T: Screen<B>,
    B: Backend,
{
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

    fn centered_area(&self, width: u16, height: u16, area: Rect) -> Rect {
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

pub trait WidgetKeyHandler {
    /// Handles user key input
    fn handle_input(&mut self, input: KeyEvent);
}

pub trait ViWidgetKeyHandler {
    /// Handles user key input using Vi keybindings. Note that it's a very basic "emulation", not every motion present in Vi will work
    fn handle_input(&mut self, input: KeyEvent, state: &mut ViState);
}

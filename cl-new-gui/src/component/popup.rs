use crate::component::button::{Button, FutureEventType};
use crate::component::table::{Cell, CustomWidth, Row, Table};
use crate::component::Renderable;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use crate::async_fn_body;
use log::debug;
use std::fmt::Debug;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
use tui::layout::Alignment::Center;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Clear, Paragraph, Wrap};
use tui::Frame;

#[derive(Debug, Clone, Default)]
pub struct PopupState {
    selected: usize,
}

impl PopupState {
    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Popup {
    pub title: String,
    pub content: String,
    pub buttons: Vec<Button>,
    pub state: PopupState,
}

impl Popup {
    pub fn next(&mut self) {
        let current = self.state.selected;
        let next = (current + 1) % self.buttons.len();
        self.state.select(next);
    }

    pub fn previous(&mut self) {
        let current = self.state.selected;
        let previous = (current + self.buttons.len() - 1) % self.buttons.len();
        self.state.select(previous);
    }

    pub async fn click(&mut self, state_tx: Sender<StateEvent>) -> anyhow::Result<ScreenCommandCallback> {
        if self.buttons.is_empty() {
            debug!("No buttons to click");
            return Ok(ScreenCommandCallback::DoNothing);
        }
        let selected_idx = self.state.selected;
        let callback = self.buttons[selected_idx].callback.clone();
        let on_click = self.buttons[selected_idx].on_click.clone();
        on_click.call(Some(state_tx), None).await?;
        Ok(callback)
    }
}

impl Popup {
    pub fn dialog(message: String, yes_action: FutureEventType, yes_callback: ScreenCommandCallback) -> Self {
        Popup {
            title: "Warning".to_string(),
            content: message,
            buttons: vec![
                Button::new("Yes", true, yes_action, yes_callback),
                Button::new(
                    "No",
                    false,
                    FutureEventType::State(|_| {
                        async_fn_body! {
                            Ok(())
                        }
                    }),
                    ScreenCommandCallback::DoNothing,
                ),
            ],
            ..Default::default()
        }
    }

    pub fn help_main() -> Self {
        Popup {
            title: "Help".to_string(),
            content: main_options().to_string(),
            buttons: vec![],
            ..Default::default()
        }
    }
}

impl Renderable for Popup {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let paragraph = Paragraph::new(self.content.to_owned())
            .alignment(Center)
            .style(
                Style::default()
                    .fg(theme.text_color.into())
                    .bg(theme.background_color.into()),
            )
            .wrap(Wrap { trim: true })
            .block(Block::bordered());

        let popup_area = compute_popup_area(&self.content, area);
        let buttons_area = split_content_and_buttons(popup_area)[1];
        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);

        // area for the buttons
        let button_area = button_area(self.buttons.len(), buttons_area);
        // render buttons inside that area
        button_area.iter().enumerate().for_each(|(i, area)| {
            let current_button = &mut self.buttons[i];
            current_button.is_active = i == self.state.selected;
            current_button.render(frame, *area, theme);
        });
    }
}

fn button_area(number_of_buttons: usize, area: Rect) -> Rc<[Rect]> {
    if number_of_buttons == 0 {
        return Rc::from([]);
    }

    let constraints =
        vec![Constraint::Percentage(100 / number_of_buttons as u16); number_of_buttons];

    create_button_layout(area, &constraints)
}

fn create_button_layout(area: Rect, constraints: &[Constraint]) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)
}

fn split_content_and_buttons(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(rect)
}

fn compute_popup_area(content: &str, area: Rect) -> Rect {
    use tui::layout::{Constraint, Direction, Layout};

    let width = content.custom_width();
    let height = 5;

    const SCALE_FACTOR: u16 = 100;
    const MAX_SCALE_RATIO: f32 = 2.0;

    let scaled_height = (SCALE_FACTOR * (height * 2)) / area.height;
    let max_height = (area.height as f32 * MAX_SCALE_RATIO) as u16;
    let final_height = std::cmp::min(scaled_height, max_height);

    let height = if final_height > 100 {
        100
    } else {
        final_height
    };

    let width = if width > 50 { 50 } else { width };

    let new_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(new_area)[1]
}

fn main_options() -> Table {
    vec![
        Row::from_iter([Cell::from("Quit"), Cell::from("<Q/Esc/Ctrl-C>")]),
        Row::from_iter([Cell::from("Create new command"), Cell::from("<I/Insert>")]),
        Row::from_iter([Cell::from("Delete selected command"), Cell::from("<D/Delete>")]),
        Row::from_iter([Cell::from("Edit selected command"), Cell::from("<E>")]),
        Row::from_iter([Cell::from("Move to next namespace"), Cell::from("<L/→/Tab>")]),
        Row::from_iter([Cell::from("Move to previous namespace"), Cell::from("<H/←/Shift-Tab>")]),
        Row::from_iter([Cell::from("Move up"), Cell::from("<K/↑>")]),
        Row::from_iter([Cell::from("Move down"), Cell::from("<J/↓>")]),
        Row::from_iter([Cell::from("Copy selected command"), Cell::from("<Y>")]),
        Row::from_iter([Cell::from("Search commands"), Cell::from("<F//>")]),
        Row::from_iter([Cell::from("Show help"), Cell::from("<F1/?>")]),
    ]
    .into()
}


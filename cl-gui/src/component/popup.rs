use crate::component::button::{Button, FutureEventType};
use crate::component::table::{Cell, CustomWidth, Row, Table};
use crate::component::Renderable;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::theme::Theme;
use std::fmt::Debug;
use std::rc::Rc;
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

    pub fn selected(&self) -> usize {
        self.selected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupType {
    Help,
    Dialog,
}

#[derive(Debug, Clone, Default)]
pub struct Popup {
    pub title: String,
    pub content: String,
    pub buttons: Vec<Button>,
    pub state: PopupState,
    pub popup_type: Option<PopupType>,
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
            popup_type: Some(PopupType::Help),
            ..Default::default()
        }
    }

    pub fn help_form() -> Self {
        Popup {
            title: "Help".to_string(),
            content: form_options().to_string(),
            buttons: vec![],
            popup_type: Some(PopupType::Help),
            ..Default::default()
        }
    }
}

impl Renderable for Popup {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let popup_area = compute_popup_area(&self.content, area, self.popup_type);
        let popup_style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());

        frame.render_widget(Clear, popup_area);

        if self.buttons.is_empty() {
            let content =
                center_content(&self.content, popup_area.width.saturating_sub(4) as usize);
            let paragraph = Paragraph::new(content)
                .alignment(Center)
                .style(popup_style)
                .wrap(Wrap { trim: true })
                .block(Block::bordered().style(popup_style));
            frame.render_widget(paragraph, popup_area);
        } else {
            let outer_block = Block::bordered().style(popup_style);
            let inner_area = outer_block.inner(popup_area);
            frame.render_widget(outer_block, popup_area);

            let [content_area, buttons_area] = split_content_and_buttons(inner_area);

            let paragraph = Paragraph::new(self.content.as_str())
                .alignment(Center)
                .style(popup_style)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, content_area);

            let button_rects = button_area(self.buttons.len(), buttons_area);
            button_rects.iter().enumerate().for_each(|(i, area)| {
                let current_button = &mut self.buttons[i];
                current_button.is_active = i == self.state.selected;
                current_button.render(frame, *area, theme);
            });
        }
    }
}

fn center_content(content: &str, available_width: usize) -> String {
    content
        .lines()
        .map(|line| {
            let line_width = line.len();
            if line_width >= available_width {
                line.to_string()
            } else {
                let padding = (available_width - line_width) / 2;
                format!("{}{}", " ".repeat(padding), line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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

fn split_content_and_buttons(rect: Rect) -> [Rect; 2] {
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(rect);
    [parts[0], parts[1]]
}

fn compute_popup_area(content: &str, area: Rect, popup_type: Option<PopupType>) -> Rect {
    // Content dimensions
    let line_count = content.lines().count() as u16;
    let content_width = content.custom_width();

    // 2 for border, 2 for horizontal padding
    let popup_width = (content_width + 4).min(area.width);
    // 2 for border; help has no buttons, dialogs reserve 3 rows for buttons
    let button_rows: u16 = if popup_type == Some(PopupType::Help) { 0 } else { 3 };
    let popup_height = (line_count + 2 + button_rows).min(area.height);

    let h_pad = area.width.saturating_sub(popup_width) / 2;
    let v_pad = area.height.saturating_sub(popup_height) / 2;

    let centered_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(v_pad),
            Constraint::Length(popup_height),
            Constraint::Min(0),
        ])
        .split(area)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(h_pad),
            Constraint::Length(popup_width),
            Constraint::Min(0),
        ])
        .split(centered_v)[1]
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
        Row::from_iter([Cell::from("Close search"), Cell::from("<Esc/Enter/↑/↓>")]),
        Row::from_iter([Cell::from("Show help"), Cell::from("<F1/?>")]),
    ]
    .into()
}

fn form_options() -> Table {
    vec![
        Row::from_iter([Cell::from("Save"), Cell::from("<Ctrl-S>")]),
        Row::from_iter([Cell::from("Cancel / exit"), Cell::from("<Esc/Ctrl-C>")]),
        Row::from_iter([Cell::from("Next field"), Cell::from("<Tab>")]),
        Row::from_iter([Cell::from("Previous field"), Cell::from("<Shift-Tab>")]),
        Row::from_iter([Cell::from("Show help"), Cell::from("<F1>")]),
    ]
    .into()
}

impl crate::observer::event::NotifyTarget for Popup {
    type Payload = crate::observer::event::PopupEvent;
    fn wrap(payload: Self::Payload) -> crate::observer::event::Event {
        crate::observer::event::Event::Popup(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_content_single_line() {
        let content = "Hello";
        let result = center_content(content, 15);
        assert!(result.starts_with("     ")); // 5 spaces for centering
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_center_content_multiple_lines() {
        let content = "Hello\nWorld";
        let result = center_content(content, 15);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        // Both lines should be padded
        assert!(lines[0].starts_with(" "));
        assert!(lines[1].starts_with(" "));
    }

    #[test]
    fn test_center_content_line_too_long() {
        let content = "This is a very long line that exceeds the available width";
        let result = center_content(content, 10);
        // Line should not be padded if it's too long
        assert_eq!(result, content);
    }

    #[test]
    fn test_popup_help_type() {
        let popup = Popup::help_main();
        assert_eq!(popup.popup_type, Some(PopupType::Help));

        let popup = Popup::help_form();
        assert_eq!(popup.popup_type, Some(PopupType::Help));
    }

    #[test]
    fn test_popup_dialog_type() {
        let popup = Popup::dialog(
            "Test".to_string(),
            FutureEventType::State(|_| {
                async_fn_body! {
                    Ok(())
                }
            }),
            ScreenCommandCallback::DoNothing,
        );
        assert_eq!(popup.popup_type, None);
    }
}


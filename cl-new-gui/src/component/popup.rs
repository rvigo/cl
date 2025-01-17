use crate::component::button::Button;
use crate::component::Component;
use crate::Pipe;
use std::fmt::Debug;
use std::rc::Rc;
use tui::layout::Alignment::Center;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct Popup {
    pub title: String,
    pub content: String,
    pub buttons: Vec<Button>,
}

impl Popup {
    pub fn new(title: impl Into<String>, content: impl Into<String>, buttons: Vec<Button>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            buttons,
        }
    }
}

impl PartialEq for Popup {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.content == other.content
    }
}

impl Eq for Popup {}

impl Component for Popup {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.content.to_owned())
            .alignment(Center)
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL));

        let popup_area = compute_popup_area(&self.content, area);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);

        // crate are for the buttons
        let button_area = button_area(self.buttons.len(), popup_area);
        // render buttons inside that area
        button_area.iter().enumerate().for_each(|(i, area)| {
            self.buttons[i].render(frame, *area);
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
        .constraints([Constraint::Percentage(100); 1])
        .split(area)[0]
        .pipe(|sub_area| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(25); 4])
                .split(sub_area)[3]
        })
        .pipe(|bottom_rect| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(bottom_rect)
        })
}

fn compute_popup_area(content: &str, area: Rect) -> Rect {
    use tui::layout::{Constraint, Direction, Layout};

    let width = content.width() as u16;
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
    let width = if width > 100 { 100 } else { width };

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area)[1]
        .pipe(|new_area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage((100 - width) / 2),
                    Constraint::Percentage(width),
                    Constraint::Percentage((100 - width) / 2),
                ])
                .split(new_area)[1]
        })
}

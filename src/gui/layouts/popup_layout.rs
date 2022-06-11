use crate::gui::contexts::state::State;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_popup<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let block = Paragraph::new(state.popup_state.message.clone())
        .style(Style::default().fg(Color::Rgb(229, 229, 229)))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(state.popup_state.message_type.to_string())
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Plain),
        );

    let area = centered_rect(50, 50, frame.size());

    frame.render_widget(Clear, area[1]);
    frame.render_widget(block, area[1]);
    draw_ok_button(frame, area[1]);
}

fn draw_ok_button<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let ok_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(area);

    let ok_layout = Layout::default()
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
        .margin(1)
        .split(ok_layout[2]);
    let ok_button_widget = Paragraph::new("Ok <Enter>")
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(ok_button_widget, ok_layout[3]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Vec<Rect> {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])
}

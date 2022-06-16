use crate::gui::contexts::{popup_state::MessageType, state::State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs, Wrap},
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

    let area = centered_rect(45, 45, frame.size());

    frame.render_widget(Clear, area[1]);
    frame.render_widget(block, area[1]);

    match state.popup_state.message_type {
        MessageType::Error => draw_option_buttons(frame, area[1], vec![String::from("Ok")]),
        MessageType::Confirmation => draw_option_buttons(
            frame,
            area[1],
            vec![String::from("Ok"), String::from("Cancel")],
        ),
        MessageType::None => {}
    }
}

fn draw_option_buttons<B: Backend>(frame: &mut Frame<B>, area: Rect, options: Vec<String>) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(create_layout(area)[3]);

    let tab_menu: Vec<Spans> = options
        .into_iter()
        .map(|tab| Spans::from(vec![Span::styled(tab.clone(), Style::default())]))
        .collect();

    let tab = Tabs::new(tab_menu)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Rgb(201, 165, 249)))
        .divider(Span::raw(""));

    frame.render_widget(tab, layout[1]);
}

fn create_layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
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
        .split(layout[3])
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

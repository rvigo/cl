use super::view_mode::ViewMode;
use crate::gui::contexts::state::State;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_helper_footer() -> Paragraph<'static> {
    let help_content = "Help <F1>";
    Paragraph::new(help_content)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .style(Style::default())
                .borders(Borders::ALL)
                .title(" Help ")
                .title_alignment(Alignment::Right)
                .border_type(BorderType::Plain),
        )
}

fn list_options() -> String {
    String::from(
        "\n \
        Quit <Q>\n\n \
        New command <I>\n\n \
        Delete <D>\n\n \
        Edit command <E>\n\n \
        Right <TAB>\n\n \
        Left <BACKTAB>\n\n \
        Up <ArrowUp>\n\n \
        Down <ArrowDown>",
    )
}

fn insert_or_update_options() -> String {
    String::from(
        "\n \
        Return <ESC>\n\n \
        Right <TAB>\n\n \
        Left <BACKTAB>\n\n \
        Create <Enter>",
    )
}

pub fn render_help<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let block = Paragraph::new(match state.view_mode {
        ViewMode::List => list_options(),
        _ => insert_or_update_options(),
    })
    .style(Style::default().fg(Color::Rgb(229, 229, 229)))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Help")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain),
    );

    let area = centered_rect(90, 90, frame.size());

    frame.render_widget(Clear, area[1]);
    frame.render_widget(block, area[1]);
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

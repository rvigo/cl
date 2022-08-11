use super::{layout_utils::centered_rect, view_mode::ViewMode};
use crate::gui::entities::state::State;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_helper_footer() -> Paragraph<'static> {
    let help_content = "Show help <F1>";
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
        Down <ArrowDown>\n\n \
        Find <F>",
    )
}

fn insert_options() -> String {
    String::from(
        "\n \
        Return <ESC>\n\n \
        Right <TAB>\n\n \
        Left <BACKTAB>\n\n \
        Create <Enter>",
    )
}

fn edit_options() -> String {
    String::from(
        "\n \
        Return <ESC>\n\n \
        Right <TAB>\n\n \
        Left <BACKTAB>\n\n \
        Update <Enter>",
    )
}

pub fn render_help<B: Backend>(frame: &mut Frame<B>, state: &State) {
    let block = Paragraph::new(match state.view_mode {
        ViewMode::List => list_options(),
        ViewMode::Edit => edit_options(),
        ViewMode::Insert => insert_options(),
    })
    .style(Style::default().fg(Color::Rgb(229, 229, 229)))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Help ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain),
    );

    let area = centered_rect(90, 90, frame.size());

    frame.render_widget(Clear, area[1]);
    frame.render_widget(block, area[1]);
}

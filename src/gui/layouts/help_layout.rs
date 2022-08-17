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

fn main_options() -> String {
    String::from(
        "\n \
        Quit <q>\n\n \
        New command <Insert / i>\n\n \
        Delete <Delete / d>\n\n \
        Edit command <e>\n\n \
        Right <Right / Tab / l>\n\n \
        Left <Left / Shift + Tab / h>\n\n \
        Up <Up / j>\n\n \
        Down <Down / k>\n\n \
        Find Commands <f>\n\n \
        Help <F1 / ?>",
    )
}

fn insert_options() -> String {
    String::from(
        "\n \
        Return <Esc>\n\n \
        Next Field <Tab>\n\n \
        Previous Field <Shift + Tab>\n\n \
        Create Command <Enter>\n\n \
        Help <F1>",
    )
}

fn edit_options() -> String {
    String::from(
        "\n \
        Return <Esc>\n\n \
        Next Field <Tab>\n\n \
        Previous Field <Shift + Tab>\n\n \
        Update Command <Enter>\n\n \
        Help <F1>",
    )
}

pub fn render_help<B: Backend>(frame: &mut Frame<B>, state: &State) {
    let block = Paragraph::new(match state.view_mode {
        ViewMode::Main => main_options(),
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

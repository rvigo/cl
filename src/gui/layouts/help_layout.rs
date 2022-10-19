use super::{
    layout_utils::{centered_rect, DEFAULT_SELECTED_COLOR},
    view_mode::ViewMode,
};
use crate::gui::entities::state::State;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint},
    style::Style,
    widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame,
};

pub fn render_helper_footer() -> Paragraph<'static> {
    let help_content = "Show help <F1/?>";
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
fn key_style() -> Style {
    Style::default().fg(DEFAULT_SELECTED_COLOR)
}
fn get_cell_style(text: &str, style: Option<Style>) -> Cell {
    if let Some(style) = style {
        Cell::from(text).style(style)
    } else {
        Cell::from(text)
    }
}

fn main_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            get_cell_style("<Q/Esc/Ctrl + C>", Some(key_style())),
            get_cell_style("Quit", None),
        ],
        vec![
            get_cell_style("<I/Insert>", Some(key_style())),
            get_cell_style("New command", None),
        ],
        vec![
            get_cell_style("<D/Delete>", Some(key_style())),
            get_cell_style("Delete command", None),
        ],
        vec![
            get_cell_style("<E>", Some(key_style())),
            get_cell_style("Edit command", None),
        ],
        vec![
            get_cell_style("<L/Right/Tab>", Some(key_style())),
            get_cell_style("Right", None),
        ],
        vec![
            get_cell_style("<H/Left/Shift + Tab>", Some(key_style())),
            get_cell_style("Left", None),
        ],
        vec![
            get_cell_style("<J/Up>", Some(key_style())),
            get_cell_style("Up", None),
        ],
        vec![
            get_cell_style("<K/Down>", Some(key_style())),
            get_cell_style("Down", None),
        ],
        vec![
            get_cell_style("<F>", Some(key_style())),
            get_cell_style("Find stored commands", None),
        ],
        vec![
            get_cell_style("<F1/?>", Some(key_style())),
            get_cell_style("Help", None),
        ],
    ]
}

fn insert_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            get_cell_style("<Esc/Ctrl + C>", Some(key_style())),
            get_cell_style("Return", None),
        ],
        vec![
            get_cell_style("<Tab>", Some(key_style())),
            get_cell_style("Next Field", None),
        ],
        vec![
            get_cell_style("<Shift + Tab>", Some(key_style())),
            get_cell_style("Previous Field", None),
        ],
        vec![
            get_cell_style("<Enter/ Ctrl + S>", Some(key_style())),
            get_cell_style("Create command", None),
        ],
        vec![
            get_cell_style("<F1>", Some(key_style())),
            get_cell_style("Help", None),
        ],
    ]
}

fn edit_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            get_cell_style("<Esc/Ctrl + C>", Some(key_style())),
            get_cell_style("Return", None),
        ],
        vec![
            get_cell_style("<Tab>", Some(key_style())),
            get_cell_style("Next Field", None),
        ],
        vec![
            get_cell_style("<Shift + Tab>", Some(key_style())),
            get_cell_style("Previous Field", None),
        ],
        vec![
            get_cell_style("<Enter/ Ctrl + S>", Some(key_style())),
            get_cell_style("Update command", None),
        ],
        vec![
            get_cell_style("<F1>", Some(key_style())),
            get_cell_style("Help", None),
        ],
    ]
}

pub fn render_help<B: Backend>(frame: &mut Frame<B>, state: &State) {
    let options = match state.view_mode {
        ViewMode::Main => main_options(),
        ViewMode::Edit => edit_options(),
        ViewMode::Insert => insert_options(),
    };

    let rows = options
        .clone()
        .into_iter()
        .map(|cells| Row::new(cells).bottom_margin(1));

    let table = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    let area = centered_rect(
        50,
        (100 * (options.len() as u16 * 2)) / frame.size().height, //dynamic height based on options size
        frame.size(),
    );

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

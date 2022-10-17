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

pub fn render_main_layout_helper_footer() -> Paragraph<'static> {
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
fn get_default_key_color() -> Style {
    Style::default().fg(DEFAULT_SELECTED_COLOR)
}

fn main_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            Cell::from("<Q/Esc/Ctrl + C>").style(get_default_key_color()),
            Cell::from("Quit"),
        ],
        vec![
            Cell::from("<Insert/I>").style(get_default_key_color()),
            Cell::from("New command"),
        ],
        vec![
            Cell::from("<Delete/D>").style(get_default_key_color()),
            Cell::from("Delete command"),
        ],
        vec![
            Cell::from("<E>").style(get_default_key_color()),
            Cell::from("Edit command"),
        ],
        vec![
            Cell::from("<Right/Tab/L>").style(get_default_key_color()),
            Cell::from("Right"),
        ],
        vec![
            Cell::from("<Left/Shift + Tab/H>").style(get_default_key_color()),
            Cell::from("Left"),
        ],
        vec![
            Cell::from("<Up/J>").style(get_default_key_color()),
            Cell::from("Up"),
        ],
        vec![
            Cell::from("<Down/K>").style(get_default_key_color()),
            Cell::from("Down"),
        ],
        vec![
            Cell::from("<F>").style(get_default_key_color()),
            Cell::from("Find stored commands"),
        ],
        vec![
            Cell::from("<F1/?>").style(get_default_key_color()),
            Cell::from("Help"),
        ],
    ]
}

fn insert_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            Cell::from("<Esc/Ctrl + C>").style(get_default_key_color()),
            Cell::from("Return"),
        ],
        vec![
            Cell::from("<Tab>").style(get_default_key_color()),
            Cell::from("Next Field"),
        ],
        vec![
            Cell::from("<Shift + Tab>").style(get_default_key_color()),
            Cell::from("Previous Field"),
        ],
        vec![
            Cell::from("<Enter/ Ctrl + S>").style(get_default_key_color()),
            Cell::from("Create command"),
        ],
        vec![
            Cell::from("<F1>").style(get_default_key_color()),
            Cell::from("Help"),
        ],
    ]
}

fn edit_options() -> Vec<Vec<Cell<'static>>> {
    vec![
        vec![
            Cell::from("<Esc/Ctrl + C>").style(get_default_key_color()),
            Cell::from("Return"),
        ],
        vec![
            Cell::from("<Tab>").style(get_default_key_color()),
            Cell::from("Next Field"),
        ],
        vec![
            Cell::from("<Shift + Tab>").style(get_default_key_color()),
            Cell::from("Previous Field"),
        ],
        vec![
            Cell::from("<Enter/ Ctrl + S>").style(get_default_key_color()),
            Cell::from("Update command"),
        ],
        vec![
            Cell::from("<F1>").style(get_default_key_color()),
            Cell::from("Help"),
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
        (100 * (options.len() as u16 * 2)) / frame.size().height, //dynamic size based on options size
        frame.size(),
    );

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

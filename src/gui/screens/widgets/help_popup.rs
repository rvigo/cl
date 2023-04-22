use super::WidgetExt;
use crate::gui::{
    entities::states::ui_state::ViewMode, screens::ScreenSize, DEFAULT_SELECTED_COLOR,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Cell, Clear, Row, Table, Widget},
};

pub struct HelpPopup<'a> {
    content: Vec<Vec<Cell<'a>>>,
    screen_size: ScreenSize,
}

impl<'a> HelpPopup<'a> {
    pub fn new(view_mode: &ViewMode, terminal_size: ScreenSize) -> HelpPopup<'a> {
        let content = match view_mode {
            ViewMode::Main => main_options(),
            ViewMode::Edit => edit_options(),
            ViewMode::Insert => insert_options(),
        };
        HelpPopup {
            content,
            screen_size: terminal_size,
        }
    }
}

impl<'a> Widget for HelpPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self
            .content
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

        let width = if self.screen_size.eq(&ScreenSize::Small) {
            100
        } else {
            50
        };

        let dynamic_height = (100 * (self.content.len() as u16 * 2)) / area.height;
        let height = std::cmp::max(dynamic_height, area.height);
        let centered_rect = self.centered_area(width, height, area);

        Clear::render(Clear, centered_rect, buf);
        table.render(centered_rect, buf)
    }
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

fn main_options<'a>() -> Vec<Vec<Cell<'a>>> {
    vec![
        vec![
            get_cell_style("<Q/Esc/Ctrl + C>", Some(key_style())),
            get_cell_style("Quit", None),
        ],
        vec![
            get_cell_style("<I/Insert>", Some(key_style())),
            get_cell_style("Create new command", None),
        ],
        vec![
            get_cell_style("<D/Delete>", Some(key_style())),
            get_cell_style("Delete selected command", None),
        ],
        vec![
            get_cell_style("<E>", Some(key_style())),
            get_cell_style("Edit selected command", None),
        ],
        vec![
            get_cell_style("<L/→/Tab>", Some(key_style())),
            get_cell_style("Move to next namespace", None),
        ],
        vec![
            get_cell_style("<H/←/Shift + Tab>", Some(key_style())),
            get_cell_style("Move to previous namespace", None),
        ],
        vec![
            get_cell_style("<K/↑>", Some(key_style())),
            get_cell_style("Move up", None),
        ],
        vec![
            get_cell_style("<J/↓>", Some(key_style())),
            get_cell_style("Move down", None),
        ],
        vec![
            get_cell_style("<Y>", Some(key_style())),
            get_cell_style("Copy selected command", None),
        ],
        vec![
            get_cell_style("<F//>", Some(key_style())),
            get_cell_style("Find stored commands", None),
        ],
        vec![
            get_cell_style("<F1/?>", Some(key_style())),
            get_cell_style("Show help", None),
        ],
    ]
}

fn insert_options<'a>() -> Vec<Vec<Cell<'a>>> {
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

fn edit_options<'a>() -> Vec<Vec<Cell<'a>>> {
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

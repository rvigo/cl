use crate::{
    centered_rect,
    gui::{entities::states::ui_state::ViewMode, screens::ScreenSize, DEFAULT_SELECTED_COLOR},
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
        let centered_rect = centered_rect!(width, height, area);

        Clear::render(Clear, centered_rect, buf);
        table.render(centered_rect, buf)
    }
}

fn key_style() -> Style {
    Style::default().fg(DEFAULT_SELECTED_COLOR)
}

macro_rules! styled_cell {
    ($text:expr) => {
        Cell::from($text)
    };

    ($text:expr, $style:expr) => {
        Cell::from($text).style($style)
    };
}

fn main_options<'a>() -> Vec<Vec<Cell<'a>>> {
    vec![
        vec![
            styled_cell!("<Q/Esc/Ctrl + C>", key_style()),
            styled_cell!("Quit"),
        ],
        vec![
            styled_cell!("<I/Insert>", key_style()),
            styled_cell!("Create new command"),
        ],
        vec![
            styled_cell!("<D/Delete>", key_style()),
            styled_cell!("Delete selected command"),
        ],
        vec![
            styled_cell!("<E>", key_style()),
            styled_cell!("Edit selected command"),
        ],
        vec![
            styled_cell!("<L/→/Tab>", key_style()),
            styled_cell!("Move to next namespace"),
        ],
        vec![
            styled_cell!("<H/←/Shift + Tab>", key_style()),
            styled_cell!("Move to previous namespace"),
        ],
        vec![styled_cell!("<K/↑>", key_style()), styled_cell!("Move up")],
        vec![
            styled_cell!("<J/↓>", key_style()),
            styled_cell!("Move down"),
        ],
        vec![
            styled_cell!("<Y>", key_style()),
            styled_cell!("Copy selected command"),
        ],
        vec![
            styled_cell!("<F//>", key_style()),
            styled_cell!("Find stored commands"),
        ],
        vec![
            styled_cell!("<F1/?>", key_style()),
            styled_cell!("Show help"),
        ],
    ]
}

fn edit_options<'a>() -> Vec<Vec<Cell<'a>>> {
    vec![
        vec![
            styled_cell!("<Esc/Ctrl + C>", key_style()),
            styled_cell!("Return"),
        ],
        vec![
            styled_cell!("<Tab>", key_style()),
            styled_cell!("Next Field"),
        ],
        vec![
            styled_cell!("<Shift + Tab>", key_style()),
            styled_cell!("Previous Field"),
        ],
        vec![
            styled_cell!("<Enter/ Ctrl + S>", key_style()),
            styled_cell!("Update command"),
        ],
        vec![styled_cell!("<F1>", key_style()), styled_cell!("Help")],
    ]
}

fn insert_options<'a>() -> Vec<Vec<Cell<'a>>> {
    vec![
        vec![
            styled_cell!("<Esc/Ctrl + C>", key_style()),
            styled_cell!("Return"),
        ],
        vec![
            styled_cell!("<Tab>", key_style()),
            styled_cell!("Next Field"),
        ],
        vec![
            styled_cell!("<Shift + Tab>", key_style()),
            styled_cell!("Previous Field"),
        ],
        vec![
            styled_cell!("<Enter/ Ctrl + S>", key_style()),
            styled_cell!("Create command"),
        ],
        vec![styled_cell!("<F1>", key_style()), styled_cell!("Help")],
    ]
}

mod choice;
mod help;
mod popup_trait;
mod popup_type;

pub use choice::Choice;
use comfy_table::presets;
use comfy_table::CellAlignment;
pub use popup_type::Type;
use tui::widgets::Padding;

use crate::screen::dialog_factory::Table;
use crate::{
    centered_rect,
    context::PopupContext,
    default_popup_block,
    event::PopupCallbackAction,
    theme::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR},
};
use std::rc::Rc;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

trait PopupExt {
    fn content_height(&self) -> u16;

    fn content_width(&self) -> u16;

    fn get_render_position(&self, area: Rect) -> Rect {
        let width = self.content_width();
        let height = self.content_height();

        let dynamic_height = (100 * (height * 2)) / area.height;
        let real_height = std::cmp::max(dynamic_height, area.height);
        centered_rect!(width, real_height, area)
    }
}

#[derive(Clone, Debug)]
pub struct Popup<C> {
    content: C,
    pub choices: Vec<Choice>,
    r#type: Type,
    pub callback: PopupCallbackAction,
}

impl<C> Popup<C> {
    pub fn new(
        content: C,
        choices: Vec<Choice>,
        r#type: Type,
        callback: PopupCallbackAction,
    ) -> Popup<C> {
        Self {
            content,
            choices,
            r#type,
            callback,
        }
    }

    fn choices(&self) -> Vec<Choice> {
        let choices = match self.r#type {
            Type::Error => Choice::confirm(),
            Type::Warning => Choice::dialog(),
            Type::Help => Choice::empty(),
        };

        choices
    }

    fn button_widget(&self, selected: usize) -> Tabs<'_> {
        let choices: Vec<Line<'_>> = self
            .choices()
            .iter()
            .map(|choice| Line::from(choice.to_string()))
            .collect();

        Tabs::new(choices)
            .block(Block::default().borders(Borders::NONE))
            .select(selected)
            .highlight_style(
                Style::default()
                    .fg(DEFAULT_SELECTED_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider("")
    }

    fn create_buttom_area(&self, area: Rect) -> Rect {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(self.create_buttom_layout(area)[4]);

        let constraints = if self.choices().len() == 2 {
            vec![Constraint::Min(50)]
        } else {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        };
        let buttom_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[0]);

        buttom_area[buttom_area.len() - 1]
    }

    fn create_buttom_layout(&self, area: Rect) -> Rc<[Rect]> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Length(3), //keeps the options inside the box
            ])
            .split(layout[3])
    }
}

impl PopupExt for Popup<String> {
    fn content_width(&self) -> u16 {
        self.content.width() as u16
    }

    fn content_height(&self) -> u16 {
        const MIN_HEIGHT: usize = 5;

        let lines = self.content.lines().count();
        MIN_HEIGHT.max(lines) as u16
    }
}

impl Widget for Popup<String> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self, area, buf, &mut PopupContext::default());
    }
}

impl StatefulWidget for Popup<String> {
    type State = PopupContext;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut PopupContext) {
        let block = default_popup_block!(self.r#type);

        let paragraph = Paragraph::new(self.content.to_owned())
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block.to_owned());

        let render_position = self.get_render_position(area);

        Clear::render(Clear, render_position, buf);
        paragraph.render(render_position, buf);

        let options = self.button_widget(state.selected_choice_idx());
        let buttom_area = self.create_buttom_area(render_position);
        options.render(buttom_area, buf);
    }
}

impl PopupExt for Popup<Table<'_>> {
    fn content_height(&self) -> u16 {
        self.content.len() as u16
    }

    fn content_width(&self) -> u16 {
        const FIXED_WIDTH: u16 = 75;

        FIXED_WIDTH
    }
}

impl Widget for Popup<Table<'_>> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self, area, buf, &mut PopupContext::default());
    }
}

impl StatefulWidget for Popup<Table<'_>> {
    type State = PopupContext;

    fn render(self, area: Rect, buf: &mut Buffer, _: &mut PopupContext) {
        let render_position = self.get_render_position(area);
        let mut t = comfy_table::Table::new();
        t.load_preset(presets::NOTHING);
        for row in &self.content.content {
            t.add_row(row.cells.iter().map(|cell| cell.text).collect::<Vec<_>>());
        }
        t.column_iter_mut().for_each(|col| {
            col.set_constraint(comfy_table::ColumnConstraint::Absolute(
                comfy_table::Width::Fixed(self.content.width() + 7),
            ));
            col.set_cell_alignment(CellAlignment::Left);
        });

        let p = Paragraph::new(t.to_string())
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(default_popup_block!(Type::Help).padding(Padding::vertical(2)));

        Clear::render(Clear, render_position, buf);
        p.render(render_position, buf)
    }
}

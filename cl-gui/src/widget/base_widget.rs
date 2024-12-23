use super::StatusBarItem;
use crate::terminal::TerminalSize;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct BaseWidget<'base, L, C, R>
where
    L: StatusBarItem,
    C: StatusBarItem,
    R: StatusBarItem,
{
    terminal_size: &'base TerminalSize,
    left_statusbar_item: Option<L>,
    center_statusbar_item: Option<R>,
    right_statusbar_item: Option<C>,
    base_block: Block<'base>,
}

impl<'base, L, C, R> BaseWidget<'base, L, C, R>
where
    L: StatusBarItem,
    C: StatusBarItem,
    R: StatusBarItem,
{
    pub fn new(
        terminal_size: &'base TerminalSize,
        left_statusbar_item: Option<L>,
        center_statusbar_item: Option<R>,
        right_statusbar_item: Option<C>,
    ) -> BaseWidget<'base, L, C, R> {
        let base_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(env!("CARGO_PKG_VERSION"))
            .title_alignment(Alignment::Right)
            .border_type(BorderType::Plain);

        BaseWidget {
            terminal_size,
            left_statusbar_item,
            center_statusbar_item,
            right_statusbar_item,
            base_block,
        }
    }
}

impl<'base, L, C, R> BaseWidget<'base, L, C, R>
where
    L: StatusBarItem,
    C: StatusBarItem,
    R: StatusBarItem,
{
    fn render_small_screen(&self, area: Rect, buf: &mut Buffer) {
        self.render_base_block(area, buf)
    }

    fn render_medium_screen(&self, area: Rect, buf: &mut Buffer) {
        let has_left_item = self.left_statusbar_item.is_some();
        let has_center_item = self.center_statusbar_item.is_some();
        let has_right_item = self.right_statusbar_item.is_some();

        if has_left_item || has_right_item || has_center_item {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(5),
                        Constraint::Min(10),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(area);

            let statusbar_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(chunks[3]);

            if let Some(left_statusbar_item) = &self.left_statusbar_item {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain);

                let left_inner_area = block.inner(statusbar_layout[0]);

                block.render(chunks[3], buf);
                left_statusbar_item.to_owned().render(left_inner_area, buf);
            }

            if let Some(center_statusbar_item) = &self.center_statusbar_item {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain);

                let center_inner_area = block.inner(statusbar_layout[1]);

                block.render(chunks[3], buf);
                center_statusbar_item
                    .to_owned()
                    .render(center_inner_area, buf);
            }

            if let Some(right_statusbar_item) = &self.right_statusbar_item {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain);

                let right_inner_area = block.inner(statusbar_layout[2]);
                right_statusbar_item
                    .to_owned()
                    .render(right_inner_area, buf);
            }
        }

        self.render_base_block(area, buf);
    }

    fn render_big_screen(&self, area: Rect, buf: &mut Buffer) {
        self.render_medium_screen(area, buf)
    }

    fn render_base_block(&self, area: Rect, buf: &mut Buffer) {
        self.base_block.to_owned().render(area, buf)
    }
}

impl<'base, L, C, R> Widget for BaseWidget<'base, L, C, R>
where
    L: StatusBarItem,
    C: StatusBarItem,
    R: StatusBarItem,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.terminal_size {
            TerminalSize::Small => self.render_small_screen(area, buf),
            TerminalSize::Medium => self.render_medium_screen(area, buf),
            TerminalSize::Large => self.render_big_screen(area, buf),
        }
    }
}

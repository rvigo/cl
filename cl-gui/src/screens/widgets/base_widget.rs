use super::StatusBarItem;
use crate::screens::ScreenSize;
use cl_core::resources::metadata::MAIN_PKG_METADATA;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct BaseWidget<'a, F, H>
where
    F: StatusBarItem,
    H: StatusBarItem,
{
    terminal_size: &'a ScreenSize,
    left_statusbar_item: Option<&'a F>,
    right_statusbar_item: Option<H>,
}

impl<'a, F, H> BaseWidget<'a, F, H>
where
    F: StatusBarItem,
    H: StatusBarItem,
{
    pub fn new(
        terminal_size: &'a ScreenSize,
        left_statusbar_item: Option<&'a F>,
        right_statusbar_item: Option<H>,
    ) -> BaseWidget<'a, F, H> {
        BaseWidget {
            terminal_size,
            left_statusbar_item,
            right_statusbar_item,
        }
    }

    fn render_small_screen(&self, area: Rect, buf: &mut Buffer) {
        self.render_base_block(area, buf)
    }

    fn render_medium_screen(&self, area: Rect, buf: &mut Buffer) {
        let has_left_item = self.left_statusbar_item.is_some();
        let has_right_item = self.right_statusbar_item.is_some();

        if has_left_item || has_right_item {
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

            if let Some(left_statusbar_item) = self.left_statusbar_item {
                let statusbar_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(28), Constraint::Length(18)].as_ref())
                    .split(chunks[3]);

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain);

                let right_inner_area = block.inner(statusbar_layout[1]);
                let left_inner_area = block.inner(statusbar_layout[0]);

                block.render(chunks[3], buf);
                left_statusbar_item.to_owned().render(left_inner_area, buf);

                if let Some(right_statusbar_item) = &self.right_statusbar_item {
                    right_statusbar_item
                        .to_owned()
                        .render(right_inner_area, buf);
                }
            } else {
                if let Some(right_statusbar_item) = &self.right_statusbar_item {
                    let statusbar_layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(chunks[3]);

                    right_statusbar_item
                        .to_owned()
                        .render(statusbar_layout[0], buf);
                }
            }
        }

        self.render_base_block(area, buf);
    }

    fn render_big_screen(&self, area: Rect, buf: &mut Buffer) {
        self.render_medium_screen(area, buf)
    }

    fn render_base_block(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {} ", MAIN_PKG_METADATA.to_string()))
            .title_alignment(Alignment::Right)
            .border_type(BorderType::Plain);

        block.render(area, buf)
    }
}

impl<'a, F, H> Widget for BaseWidget<'a, F, H>
where
    F: StatusBarItem,
    H: StatusBarItem,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.terminal_size {
            ScreenSize::Small => self.render_small_screen(area, buf),
            ScreenSize::Medium => self.render_medium_screen(area, buf),
            ScreenSize::Large => self.render_big_screen(area, buf),
        }
    }
}

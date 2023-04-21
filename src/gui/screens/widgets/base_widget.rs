use super::Footer;
use crate::gui::screens::ScreenSize;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct BaseWidget<'a, F, H>
where
    F: Footer,
    H: Footer,
{
    terminal_size: &'a ScreenSize,
    footer: Option<&'a F>,
    help_footer: H, //TODO improve the name of this widget
}

impl<'a, F, H> BaseWidget<'a, F, H>
where
    F: Footer,
    H: Footer,
{
    pub fn new(
        terminal_size: &'a ScreenSize,
        footer: Option<&'a F>,
        help_footer: H,
    ) -> BaseWidget<'a, F, H> {
        BaseWidget {
            terminal_size,
            footer,
            help_footer,
        }
    }

    fn render_small_screen(&self, area: Rect, buf: &mut Buffer) {
        self.render_base_block(area, buf)
    }

    fn render_medium_screen(&self, area: Rect, buf: &mut Buffer) {
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

        if let Some(footer) = self.footer {
            let footer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(28), Constraint::Length(18)].as_ref())
                .split(chunks[3]);
            footer.to_owned().render(footer_layout[0], buf);
            self.help_footer.to_owned().render(footer_layout[1], buf)
        } else {
            let footer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(chunks[3]);
            self.help_footer.to_owned().render(footer_layout[0], buf)
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
            .title(format!(" cl {} ", env!("CARGO_PKG_VERSION")))
            .title_alignment(Alignment::Right)
            .border_type(BorderType::Plain);

        block.render(area, buf)
    }
}

impl<'a, F, H> Widget for BaseWidget<'a, F, H>
where
    F: Footer,
    H: Footer,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.terminal_size {
            ScreenSize::Small => self.render_small_screen(area, buf),
            ScreenSize::Medium => self.render_medium_screen(area, buf),
            ScreenSize::Large => self.render_big_screen(area, buf),
        }
    }
}

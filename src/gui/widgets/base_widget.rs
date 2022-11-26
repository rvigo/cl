use super::{display::DisplayWidget, query_box::QueryBox};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct BaseWidget<'a> {
    query_box: Option<&'a QueryBox<'a>>,
}

impl<'a> BaseWidget<'a> {
    pub fn new(query_box: Option<&'a QueryBox<'a>>) -> BaseWidget<'a> {
        BaseWidget { query_box }
    }
}

impl<'a> Widget for BaseWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
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

        if let Some(query_box) = self.query_box.cloned() {
            let footer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(28), Constraint::Length(18)].as_ref())
                .split(chunks[3]);
            query_box.render(footer[0], buf);
            create_helper_footer().render(footer[1], buf)
        } else {
            let footer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(chunks[3]);
            create_helper_footer().render(footer[0], buf)
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" cl {} ", env!("CARGO_PKG_VERSION")))
            .title_alignment(Alignment::Right)
            .border_type(BorderType::Plain);

        block.render(area, buf)
    }
}

fn create_helper_footer<'a>() -> DisplayWidget<'a> {
    let help_content = "Show help <F1/?>";
    let block = Block::default()
        .style(Style::default())
        .borders(Borders::ALL)
        .title(" Help ")
        .title_alignment(Alignment::Right)
        .border_type(BorderType::Plain);
    DisplayWidget::new(help_content, true)
        .alignment(Alignment::Right)
        .block(block)
}

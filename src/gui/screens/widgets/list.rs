use super::WidgetExt;
use crate::{
    command::Command,
    gui::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR},
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{List, ListItem, ListState, StatefulWidget, Widget},
};

#[derive(Clone)]
pub struct ListWidget<'a> {
    items: Vec<ListItem<'a>>,
    state: ListState,
}

impl<'a> ListWidget<'a> {
    pub fn new(commands: Vec<Command>, state: ListState) -> ListWidget<'a> {
        let items: Vec<ListItem> = commands
            .into_iter()
            .map(|c| {
                ListItem::new(vec![Spans::from(c.alias)])
                    .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            })
            .collect();

        Self { items, state }
    }
}

impl<'a> Widget for ListWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.to_owned(), area, buf, &mut self.state)
    }
}

impl<'a> StatefulWidget for ListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(
            List::new(self.items.to_owned())
                .block(self.default_block("Aliases"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(DEFAULT_SELECTED_COLOR)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> "),
            area,
            buf,
            state,
        )
    }
}

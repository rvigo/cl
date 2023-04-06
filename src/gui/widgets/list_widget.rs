use crate::{
    command::Command,
    gui::layouts::{get_default_block, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR},
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
    pub fn new(commands: Vec<Command>) -> ListWidget<'a> {
        let items: Vec<ListItem> = commands
            .into_iter()
            .map(|c| {
                let lines = vec![Spans::from(c.alias)];
                ListItem::new(lines.clone().to_owned())
                    .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            })
            .collect();

        Self {
            items,
            state: ListState::default(),
        }
    }
}

impl<'a> Widget for ListWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.clone(), area, buf, &mut self.state)
    }
}

impl<'a> StatefulWidget for ListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(
            List::new(self.items.clone())
                .block(get_default_block("Aliases"))
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

use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::prelude::{Modifier, Style};
use tui::widgets::{Block, List as TuiList, ListItem, ListState};
use tui::Frame;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct List {
    items: Vec<String>,
    pub state: ListState,
}

impl List {
    pub fn new() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            ..Default::default()
        }
    }

    pub fn next(&mut self, next: usize) {
        self.state.select(Some(next));
    }

    pub fn previous(&mut self, previous: usize) {
        self.state.select(Some(previous));
    }

    pub fn update_items(&mut self, items: Vec<String>) {
        self.items = items
    }
}

impl Renderable for List {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        let block_style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());
        let tui_list = TuiList::new(
            self.items
                .iter()
                .cloned()
                .map(ListItem::new)
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .block(Block::bordered().style(block_style));

        frame.render_stateful_widget(tui_list, area, &mut self.state);
    }
}

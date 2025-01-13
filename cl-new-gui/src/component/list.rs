use crate::component::StatefulComponent;
use tui::layout::Rect;
use tui::prelude::{Modifier, Style};
use tui::style::Color;
use tui::widgets::{List as TuiList, ListItem, ListState};
use tui::Frame;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct List {
    pub items: Vec<String>,
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
}

impl StatefulComponent for List {
    fn render_stateful(&mut self, frame: &mut Frame, area: Rect) {
        let tui_list = TuiList::new(
            self.items
                .iter()
                .cloned()
                .map(ListItem::new)
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

        frame.render_stateful_widget(tui_list, area, &mut self.state);
    }
}

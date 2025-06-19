use crate::component::{Component, Renderable, Search};
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::layout::Direction::{Horizontal, Vertical};
use tui::layout::{Constraint, Layout};
use tui::Frame;

pub struct QuickSearchLayer {
    pub search: Component,
    pub listeners: BTreeMap<TypeId, Vec<Component>>,
}

impl Layer for QuickSearchLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let search = Search::default();
        let search = Component::new(search);

        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Search>(), vec![search.clone()]);

        Self { search, listeners }
    }

    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let [_, second_row] = *Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Percentage(50); 2])
            .split(frame.area())
        else {
            todo!()
        };

        let [first_col, _] = *Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Percentage(50); 2])
            .split(second_row)
        else {
            todo!()
        };

        // TODO adjust theme
        self.search.render(frame, first_col, theme)
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>> {
        self.listeners.clone()
    }
}

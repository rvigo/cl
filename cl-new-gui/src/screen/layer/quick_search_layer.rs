use crate::component::{Component, RenderableComponent, Search};
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::Direction::{Horizontal, Vertical};
use tui::layout::{Constraint, Layout};
use tui::Frame;
use crate::observer::observable::Observable;

pub struct QuickSearchLayer {
    pub search: RenderableComponent<Search>,
    pub listeners: BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>,
}

impl Layer for QuickSearchLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let search = Search::default();
        let search = RenderableComponent(Component::new(search));

        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Search>(), vec![search.get_observable()]);

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

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
      self.listeners.clone()
    }
}

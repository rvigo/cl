use crate::component::{Component, RenderableComponent, Search};
use crate::observer::observable::Observable;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Layout, Rect};
use tui::Frame;

pub struct QuickSearchLayer {
    pub search: RenderableComponent<Search>,
    pub listeners: BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>,
}

impl Default for QuickSearchLayer {
    fn default() -> Self {
        let search = Search::default();
        let search = RenderableComponent(Component::new(search));

        let mut listeners = BTreeMap::new();
        listeners.insert(TypeId::of::<Search>(), vec![search.get_observable()]);

        Self { search, listeners }
    }
}

impl Layer for QuickSearchLayer {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let area = centered_rect(50, 30, frame.area());
        self.search.render(frame, area, theme)
    }

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        &self.listeners
    }
}

/// Return a [`Rect`] centered within `r`, sized by percentages of width/height.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let margin_v = (100 - percent_y) / 2;
    let margin_h = (100 - percent_x) / 2;

    let vertical = Layout::vertical([
        Constraint::Percentage(margin_v),
        Constraint::Percentage(percent_y),
        Constraint::Percentage(margin_v),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage(margin_h),
        Constraint::Percentage(percent_x),
        Constraint::Percentage(margin_h),
    ])
    .split(vertical[1])[1]
}

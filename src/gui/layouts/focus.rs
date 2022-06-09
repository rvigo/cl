use itertools::Itertools;
use log::info;
use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct Focus {
    pub focus_state: ListState,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]

pub struct Item {
    name: String,
    focus: bool,
    pub input: String,
}

impl Item {
    pub fn new(name: String, focus: bool) -> Item {
        Item {
            name,
            focus,
            input: String::from(""),
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus = !self.focus
    }
}

impl Focus {
    pub fn new(items: Vec<(String, bool)>) -> Focus {
        let items = items
            .into_iter()
            .map(|(name, focus)| Item::new(name, focus))
            .collect_vec();
        Focus {
            items,
            focus_state: ListState::default(),
        }
    }

    pub fn is_in_focus(&self, name: &str) -> bool {
        let item = self
            .items
            .get(self.focus_state.selected().unwrap())
            .unwrap();
        item.name == name && item.focus
    }

    pub fn next(&mut self) {
        let old_i = self.focus_state.selected().unwrap();
        self.items.get_mut(old_i).unwrap().toggle_focus();
        let i = match self.focus_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.focus_state.select(Some(i));
        self.items.get_mut(i).unwrap().toggle_focus();
    }
    pub fn previous(&mut self) {
        let old_i = self.focus_state.selected().unwrap();
        self.items.get_mut(old_i).unwrap().toggle_focus();
        let i = match self.focus_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.focus_state.select(Some(i));
        self.items.get_mut(i).unwrap().toggle_focus();
    }

    pub fn get_current_in_focus(&mut self) -> &mut Item {
        let current = self
            .items
            .get_mut(self.focus_state.selected().unwrap())
            .unwrap();
        info!("current on focus: {:?}", current);
        current
    }

    pub(crate) fn get_component_input(&self, component_name: &str) -> String {
        self.items
            .clone()
            .into_iter()
            .filter(|item| item.name == component_name)
            .next()
            .unwrap()
            .input
    }
}

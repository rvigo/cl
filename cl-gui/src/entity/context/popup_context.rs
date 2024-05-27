use super::Selectable;
use crate::{
    entity::{popup_info::PopupInfo, state::State},
    widget::popup::choice::Choice,
};

#[derive(Default)]
pub struct PopupContext {
    pub info: PopupInfo,
    selected_choice_idx: usize,
    available_choices: Vec<Choice>,
}

impl PopupContext {
    pub fn new() -> PopupContext {
        Self {
            info: PopupInfo::default(),
            selected_choice_idx: 0,
            available_choices: vec![],
        }
    }

    pub fn set_available_choices(&mut self, choices: Vec<Choice>) {
        self.available_choices = choices
    }

    pub fn get_available_choices(&self) -> &Vec<Choice> {
        &self.available_choices
    }

    pub fn selected_choice(&self) -> usize {
        self.selected_choice_idx
    }

    pub fn clear(&mut self) {
        self.selected_choice_idx = 0;
    }
}

impl Selectable for PopupContext {
    fn next(&mut self) {
        let current = self.selected_choice_idx;
        let next = (current + 1) % self.available_choices.len();

        self.selected_choice_idx = next;
    }

    fn previous(&mut self) {
        let current = self.selected_choice_idx;
        let previous = (current + self.available_choices.len() - 1) % self.available_choices.len();

        self.selected_choice_idx = previous;
    }
}

impl State for PopupContext {
    type Output = usize;

    fn selected(&self) -> usize {
        self.selected_choice_idx
    }

    fn select(&mut self, index: usize) {
        self.selected_choice_idx = index;
    }
}

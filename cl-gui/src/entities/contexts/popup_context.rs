use super::Selectable;
use crate::{
    entities::states::{popup_state::PopupState, State},
    widgets::popup::option::Choice,
};

#[derive(Default)]
pub struct PopupContext {
    selected_choice: Option<Choice>,
    state: PopupState,
    available_choices: Vec<Choice>,
}

impl PopupContext {
    pub fn new() -> PopupContext {
        let mut context = Self {
            selected_choice: None,
            state: PopupState::default(),
            available_choices: vec![],
        };
        context.state.select(Some(0));

        context
    }

    pub fn set_available_choices(&mut self, choices: Vec<Choice>) {
        self.available_choices = choices
    }

    pub fn get_available_choices(&self) -> Vec<Choice> {
        self.available_choices.clone()
    }

    pub fn state(&self) -> &PopupState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut PopupState {
        &mut self.state
    }

    pub fn clear(&mut self) {
        self.selected_choice = None;

        self.state.select(Some(0));
    }
}

impl Selectable for PopupContext {
    fn next(&mut self) {
        let current = self.state.selected().unwrap_or(0);
        let next = (current + 1) % self.available_choices.len();

        self.state.select(Some(next));
    }

    fn previous(&mut self) {
        let current = self.state.selected().unwrap_or(0);
        let previous = (current + self.available_choices.len() - 1) % self.available_choices.len();

        self.state.select(Some(previous));
    }
}

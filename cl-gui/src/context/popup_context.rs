use super::Selectable;
use crate::{
    event::PopupCallbackAction,
    widget::popup::{Choice, Content, Type},
    State,
};

#[derive(Default)]
pub struct PopupContext {
    pub content: Content,
    selected_choice_idx: usize,
    available_choices: Vec<Choice>,
    show_popup: bool,
}

impl PopupContext {
    pub fn new() -> PopupContext {
        Self {
            content: Content::default(),
            selected_choice_idx: 0,
            available_choices: vec![],
            show_popup: false,
        }
    }

    pub fn set_choices(&mut self, choices: Vec<Choice>) {
        self.available_choices = choices
    }

    pub fn choices(&self) -> &Vec<Choice> {
        &self.available_choices
    }

    pub fn selected_choice(&self) -> Choice {
        self.choices()[self.selected_choice_idx].to_owned()
    }

    pub fn selected_choice_idx(&self) -> usize {
        self.selected_choice_idx
    }

    pub fn clear_choices(&mut self) {
        self.selected_choice_idx = 0;
    }

    pub fn set_content(
        &mut self,
        popup_type: Type,
        message: String,
        callback_action: PopupCallbackAction,
    ) {
        let answers = match popup_type {
            Type::Error => Choice::confirm(),
            Type::Warning => Choice::dialog(),
            Type::Help => Choice::empty(),
        };
        self.set_choices(answers);
        self.content
            .set(popup_type.to_string(), popup_type, message, callback_action);
    }

    pub fn show_popup(&self) -> bool {
        self.show_popup
    }

    pub fn set_show_popup(&mut self, show: bool) {
        self.show_popup = show
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

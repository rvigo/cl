use crate::gui::widgets::popup::{Answer, ChoicesState, Popup};

#[derive(Default)]
pub struct PopupContext {
    answer: Option<Answer>,
    choices_state: ChoicesState,
    popup: Option<Popup>,
}

impl PopupContext {
    pub fn new() -> PopupContext {
        let mut context = Self {
            answer: None,
            choices_state: ChoicesState::default(),
            popup: None,
        };
        context.choices_state.select(Some(0));

        context
    }

    pub fn get_popup(&self) -> Option<Popup> {
        self.popup.to_owned()
    }

    pub fn set_popup(&mut self, popup: Option<Popup>) {
        self.popup = popup
    }

    pub fn state(&self) -> &ChoicesState {
        &self.choices_state
    }

    pub fn state_mut(&mut self) -> &mut ChoicesState {
        &mut self.choices_state
    }

    pub fn answer(&self) -> Option<Answer> {
        self.answer.to_owned()
    }

    pub fn clear(&mut self) {
        self.answer = None;
        self.popup = None;
        self.choices_state.select(Some(0));
    }
}

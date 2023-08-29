use super::Selectable;
use crate::gui::{
    entities::states::{answer_state::AnswerState, State},
    screens::widgets::popup::{Answer, Popup},
};

#[derive(Default)]
pub struct PopupContext {
    answer: Option<Answer>,
    answer_state: AnswerState,
    popup: Option<Popup>,
}

impl PopupContext {
    pub fn new() -> PopupContext {
        let mut context = Self {
            answer: None,
            answer_state: AnswerState::default(),
            popup: None,
        };
        context.answer_state.select(Some(0));

        context
    }

    pub fn get_popup(&self) -> Option<Popup> {
        self.popup.to_owned()
    }

    pub fn set_popup(&mut self, popup: Option<Popup>) {
        self.popup = popup
    }

    pub fn state(&self) -> &AnswerState {
        &self.answer_state
    }

    pub fn state_mut(&mut self) -> &mut AnswerState {
        &mut self.answer_state
    }

    pub fn answer(&self) -> Option<Answer> {
        self.answer.to_owned()
    }

    pub fn clear(&mut self) {
        self.answer = None;
        self.popup = None;
        self.answer_state.select(Some(0));
    }
}

impl Selectable for PopupContext {
    fn next(&mut self) {
        if let Some(popup) = &self.popup {
            let current = self.answer_state.selected().unwrap_or(0);
            let next = (current + 1) % popup.choices().len();

            self.answer_state.select(Some(next));
        }
    }

    fn previous(&mut self) {
        if let Some(popup) = &self.popup {
            let current = self.answer_state.selected().unwrap_or(0);
            let previous = (current + popup.choices().len() - 1) % popup.choices().len();

            self.answer_state.select(Some(previous));
        }
    }
}

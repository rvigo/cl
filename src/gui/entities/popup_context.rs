use super::{answer_state::AnswerState, state::State};
use crate::gui::widgets::popup::{Answer, Popup};

#[derive(Default, Clone)]
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

    pub fn next(&mut self) {
        if let Some(popup) = &self.popup {
            let mut i = self.answer_state.selected().unwrap_or(0);
            i = if i >= popup.choices().len() - 1 {
                0
            } else {
                i + 1
            };

            self.answer_state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        if let Some(popup) = &self.popup {
            let mut i = self.answer_state.selected().unwrap_or(0);
            i = if i == 0 {
                popup.choices().len() - 1
            } else {
                i - 1
            };

            self.answer_state.select(Some(i));
        }
    }
}

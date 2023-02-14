use crate::gui::widgets::popup::{Answer, ChoicesState, Popup};

#[derive(Default)]
pub struct PopupContext<'a> {
    pub answer: Option<Answer>,
    pub choices_state: ChoicesState,
    pub popup: Option<Popup<'a>>,
}

impl<'a> PopupContext<'a> {
    pub fn clear(&mut self) {
        self.answer = None;
        self.popup = None;
        self.choices_state.select(Some(0));
    }
}

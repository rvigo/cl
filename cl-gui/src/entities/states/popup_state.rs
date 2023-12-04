use super::State;
use crate::{
    entities::events::app_events::PopupCallbackAction, widgets::popup::popup_type::PopupType,
};

#[derive(Default, Clone, Debug)]
pub struct PopupState {
    selected_choice: Option<usize>,
    pub title: String,
    pub message: String,
    pub popup_type: PopupType,
    pub callback: PopupCallbackAction,
}

impl PopupState {
    pub fn set<T: Into<String>>(
        &mut self,
        title: T,
        popup_type: PopupType,
        message: String,
        callback: PopupCallbackAction,
    ) {
        self.title = title.into();
        self.popup_type = popup_type;
        self.callback = callback;
        self.message = message
    }
}

impl State for PopupState {
    type Output = Option<usize>;

    fn selected(&self) -> Option<usize> {
        self.selected_choice
    }

    fn select(&mut self, index: Option<usize>) {
        self.selected_choice = index;
    }
}

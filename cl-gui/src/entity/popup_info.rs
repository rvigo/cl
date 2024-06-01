use crate::{entity::event::PopupCallbackAction, widget::popup::PopupType};

#[derive(Default, Clone, Debug)]
pub struct PopupInfo {
    pub title: String,
    pub message: String,
    pub popup_type: PopupType,
    pub callback: PopupCallbackAction,
}

impl PopupInfo {
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

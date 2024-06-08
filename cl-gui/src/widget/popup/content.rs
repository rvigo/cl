use crate::{event::PopupCallbackAction, widget::popup::Type};

#[derive(Default, Clone, Debug)]
pub struct Content {
    pub title: String,
    pub message: String,
    pub popup_type: Type,
    pub callback: PopupCallbackAction,
}

impl Content {
    pub fn set<T: Into<String>>(
        &mut self,
        title: T,
        popup_type: Type,
        message: String,
        callback: PopupCallbackAction,
    ) {
        self.title = title.into();
        self.popup_type = popup_type;
        self.callback = callback;
        self.message = message
    }
}

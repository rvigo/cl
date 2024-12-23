use crate::widget::popup::{Choice, Popup};

pub struct GenericErrorDialog(Popup<String>);

impl GenericErrorDialog {
    pub fn new<I: Into<String>>(message: I) -> Popup<String> {
        let choices = vec![Choice::Ok];
        let r#type = crate::widget::popup::Type::Error;
        let callback = crate::event::PopupCallbackAction::None;
        let popup = Popup::new(message.into(), choices, r#type, callback);

        popup
    }
}

use crate::widget::popup::{Choice, Popup};

pub struct GenericErrorDialog;

impl GenericErrorDialog {
    pub fn create<I>(message: I) -> Popup
    where
        I: Into<String>,
    {
        let choices = Choice::confirm();
        let r#type = crate::widget::popup::Type::Error;
        let callback = crate::event::PopupCallbackAction::None;

        Popup::new(message.into(), choices, r#type, callback)
    }
}

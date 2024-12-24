use crate::widget::popup::{Choice, Popup};

pub struct GenericErrorDialog;

impl GenericErrorDialog {
	pub fn create<I: Into<String>>(message: I) -> Popup {
		let choices = Choice::confirm();
		let r#type = crate::widget::popup::Type::Error;
		let callback = crate::event::PopupCallbackAction::None;

		Popup::new(message.into(), choices, r#type, callback)
	}
}

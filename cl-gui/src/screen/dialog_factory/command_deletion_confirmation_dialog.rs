use crate::{
	event::PopupCallbackAction,
	widget::popup::{Choice, Popup, Type},
};

const CONTENT: &str = "Are you sure you want to delete the command?";

pub struct CommandDeletionConfirmationDialog;

impl CommandDeletionConfirmationDialog {
	pub fn create() -> Popup {
		let choices = Choice::dialog();
		let r#type = Type::Warning;
		let callback = PopupCallbackAction::RemoveCommand;

		Popup::new(CONTENT.into(), choices, r#type, callback)
	}
}

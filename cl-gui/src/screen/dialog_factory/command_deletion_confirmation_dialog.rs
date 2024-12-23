use crate::{
    event::PopupCallbackAction,
    widget::popup::{Choice, Popup, Type},
};

const CONTENT: &str = "Are you sure you want to delete the command?";

pub struct CommandDeletionConfirmationDialog(Popup<String>);

impl CommandDeletionConfirmationDialog {
    pub fn new() -> Popup<String> {
        let choices = vec![Choice::Ok, Choice::Cancel];
        let r#type = Type::Warning;
        let callback = PopupCallbackAction::RemoveCommand;
        let popup = Popup::new(CONTENT.into(), choices, r#type, callback);

        popup
    }
}

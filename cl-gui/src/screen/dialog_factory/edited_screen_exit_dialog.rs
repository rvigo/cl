use crate::{
    event::{PopupCallbackAction, RenderEvent},
    widget::popup::{Choice, Popup, Type},
};

const CONTENT: &str = "Wait, you didn't save your changes! Are you sure you want to quit?";

pub struct EditedScreenExitDialog;

impl EditedScreenExitDialog {
    pub fn create() -> Popup {
        let choices = Choice::dialog();
        let r#type = Type::Warning;
        let callback = PopupCallbackAction::Render(RenderEvent::Main);

        Popup::new(CONTENT.to_owned(), choices, r#type, callback)
    }
}

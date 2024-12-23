use crate::{
    event::{PopupCallbackAction, RenderEvent},
    widget::popup::{Choice, Popup, Type},
};

const CONTENT: &str = "Wait, you didn't save your changes! Are you sure you want to quit?";

pub struct EditedScreenExitDialog(Popup<String>);

impl EditedScreenExitDialog {
    pub fn new() -> Popup<String> {
        let choices = vec![Choice::Ok, Choice::Cancel];
        let r#type = Type::Warning;
        let callback = PopupCallbackAction::Render(RenderEvent::Main);
        let popup = Popup::new(CONTENT.to_owned(), choices, r#type, callback);

        popup
    }
}

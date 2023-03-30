use crate::gui::widgets::popup::Answer;
use crossterm::event::KeyEvent;

#[derive(Clone, Debug)]
pub enum RenderEvents {
    Main,
    Edit,
    Insert,
}

#[derive(Clone, Debug)]
pub enum AppEvents {
    Run(CommandEvents),
    Render(RenderEvents),
    Screen(ScreenEvents),
    Popup(PopupEvent),
    QueryBox(QueryboxEvent),
    Quit,
}

#[derive(Clone, Debug)]
pub enum CommandEvents {
    Execute,
    Edit,
    Insert,
    Delete,
}

#[derive(Clone, Debug)]
pub enum PopupEvent {
    Enable(PopupType),
    Answer(Option<Answer>),
    Disable,
}
#[derive(Clone, Debug)]
pub enum PopupType {
    Help,
    Dialog {
        message: String,
        callback_action: PopupCallbackAction,
    },
    Error {
        message: String,
    },
}

#[derive(Clone, Debug)]
pub enum PopupCallbackAction {
    Delete,
    None,
}

#[derive(Clone, Debug)]
pub enum QueryboxEvent {
    Active,
    Deactive,
}
#[derive(Clone, Debug)]
pub enum ScreenEvents {
    Main(MainScreenEvent),
    Form(FormScreenEvent),
}

#[derive(Clone, Debug)]
pub enum MainScreenEvent {
    NextCommand,
    PreviousCommand,
    NextNamespace,
    PreviousNamespace,
}

#[derive(Clone, Debug)]
pub enum FormScreenEvent {
    NextField,
    PreviousField,
    Input(KeyEvent),
}

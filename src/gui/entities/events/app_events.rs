use crossterm::event::KeyEvent;

#[derive(Clone, Debug)]
pub enum RenderEvent {
    Main,
    Edit,
    Insert,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    Run(CommandEvent),
    Render(RenderEvent),
    Screen(ScreenEvent),
    Popup(PopupEvent),
    QueryBox(QueryboxEvent),
    Quit,
}

#[derive(Clone, Debug)]
pub enum CommandEvent {
    Execute,
    Edit,
    Insert,
    Copy,
}

#[derive(Clone, Debug)]
pub enum PopupEvent {
    Enable(PopupType),
    Answer,
    Disable,
    NextChoice,
    PreviousChoice,
}

#[derive(Clone, Debug)]
pub enum PopupType {
    Help,
    Dialog {
        message: String,
        callback_action: PopupCallbackAction,
    },
}

#[derive(Clone, Debug)]
pub enum PopupCallbackAction {
    DeleteCommand,
    Render(RenderEvent),
    None,
}

#[derive(Clone, Debug)]
pub enum QueryboxEvent {
    Active,
    Deactive,
    Input(KeyEvent),
}

#[derive(Clone, Debug)]
pub enum ScreenEvent {
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

use cl_core::Command;

pub enum UiEvent {
    ShowCommand(Command<'static>),
}

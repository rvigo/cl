use cl_core::Command;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum StateEvent {
    SelectNextCommand {
        respond_to: oneshot::Sender<Command<'static>>,
    },
    ExecuteCommand(Command<'static>),
}

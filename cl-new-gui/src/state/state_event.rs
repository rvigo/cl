use crate::state::state::SelectedCommand;
use cl_core::Command;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum StateEvent {
    SelectNextCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
    SelectPreviousCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
    ExecuteCommand,
    // Starter event
    GetAllItems {
        respond_to: oneshot::Sender<Vec<Command<'static>>>,
    },
    CurrentCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
}

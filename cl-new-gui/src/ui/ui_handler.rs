use crate::ui::ui_actor::UiActor;
use crate::ui::ui_event::UiEvent;
use cl_core::Command;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct UiHandler {
    sender: Sender<UiEvent>,
}

impl UiHandler {
    // pub fn new() -> Self {
    //     let (tx, rx) = tokio::sync::mpsc::channel(8);
    //     let mut actor = UiActor::new(rx);
    // 
    //     tokio::spawn(async move {
    //         actor.run().await;
    //     });
    // 
    //     Self { sender: tx }
    // }
    // 
    // pub async fn show_command(&self, command: Command<'static>) {
    //     self.sender.send(UiEvent::ShowCommand(command)).await.ok();
    // }
}

use crate::screen::Screens;
use cl_core::Command;
use log::debug;

#[derive(Default)]
pub struct Ui {
    pub selected_command: Option<Command<'static>>,
    pub screens: Screens,
}

impl Ui {
    pub fn new() -> Ui {
        Self {
            ..Default::default()
        }
    }

    pub async fn select_command(&mut self, command: Option<Command<'static>>) {
        debug!("old command: {:?}", self.selected_command);
        debug!("new command: {:?}", command);
        self.selected_command = command.clone();

        // this is a good place to update screens components
        // maybe is there a way to handle async components?

       // USE COMPONENT_PUBLISHER 
        if let Some(cmd) = command {
            self.screens.main.component.update_command(cmd).await
        }
    }
}

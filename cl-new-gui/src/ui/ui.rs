use crate::observer::{ObserverEvent, Publisher};
use crate::screen::Screens;
use crate::state::state::SelectedCommand;

pub struct Ui {
    pub selected_command: SelectedCommand,
    pub screens: Screens,
}

impl Ui {
    pub fn new() -> Ui {
        Self {
            selected_command: SelectedCommand::default(),
            screens: Screens::new(),
        }
    }

    pub async fn select_command(&mut self, selected_command: SelectedCommand) {
        self.selected_command = selected_command;

        let active_screen = self.screens.get_active_screen_mut();
        let publisher = active_screen.get_publisher();
        publisher
            .notify(ObserverEvent::new(self.selected_command.value.clone()))
            .await;
    }

    pub async fn next_command(&mut self, selected_command: SelectedCommand) {
        self.screens.main.list.next(selected_command.current_idx);
        self.select_command(selected_command).await;
    }

    pub async fn previous_command(&mut self, selected_command: SelectedCommand) {
        self.screens
            .main
            .list
            .previous(selected_command.current_idx);
        self.select_command(selected_command).await;
    }
}

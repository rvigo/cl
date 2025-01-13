use crate::component::{List, TextBox};
use crate::observer::event::{ListAction, ListEvent, TextboxEvent};
use crate::observer::listener::{Listener, ListenerId};
use crate::observer::publisher::PublisherContainer;
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

    pub async fn update_list_items(&mut self, items: Vec<String>) {
        let publisher = self.get_publisher(List::get_id());

        publisher
            .notify(ListEvent::new(ListAction::UpdateAll(items)))
            .await
    }

    pub async fn select_command(&mut self, selected_command: SelectedCommand) {
        let publisher = self.get_publisher(TextBox::get_id());

        publisher
            .notify(TextboxEvent::new(selected_command.value.clone()))
            .await;

        self.selected_command = selected_command;
    }

    pub async fn next_command(&mut self, selected_command: SelectedCommand) {
        let publisher = self.get_publisher(List::get_id());

        publisher
            .notify(ListEvent::new(ListAction::Next(
                selected_command.current_idx,
            )))
            .await;

        self.select_command(selected_command).await;
    }

    pub async fn previous_command(&mut self, selected_command: SelectedCommand) {
        let publisher = self.get_publisher(List::get_id());

        publisher
            .notify(ListEvent::new(ListAction::Previous(
                selected_command.current_idx,
            )))
            .await;

        self.select_command(selected_command).await;
    }

    fn get_publisher(&mut self, id: ListenerId) -> &mut PublisherContainer {
        let active_screen = self.screens.get_active_screen_mut();
        active_screen.get_publisher(id)
    }
}

use crate::observer::event::{Event, ListAction, ListEvent, TabsEvent, TextboxEvent};
use crate::observer::publisher::publisher_container::PublisherContainer;
use crate::screen::Screens;
use crate::state::state::{SelectedCommand, SelectedNamespace};
use cl_core::Command;
use std::any::TypeId;

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

    pub async fn update_items(&mut self, items: Vec<Command<'static>>) {
        let (list_items, tabs_items): (Vec<String>, Vec<String>) = items
            .iter()
            .map(|item| (item.alias.to_string(), item.namespace.to_string()))
            .unzip();

        self.update_list_items(list_items).await;

        self.update_tabs(tabs_items).await;

        // select the first command
        let event = TextboxEvent::UpdateCommand(items[0].clone());
        self.notify(event).await;
    }

    pub async fn update_list_items(&mut self, items: Vec<String>) {
        self.notify(ListEvent::new(ListAction::UpdateAll(items)))
            .await;
    }

    pub async fn update_tabs(&mut self, namespaces: Vec<String>) {
        self.notify(TabsEvent::UpdateItems(namespaces)).await;
    }

    pub async fn select_command(&mut self, selected_command: SelectedCommand) {
        self.notify(TextboxEvent::UpdateCommand(selected_command.value.clone()))
            .await;

        self.selected_command = selected_command;
    }

    pub async fn next_command(&mut self, selected_command: SelectedCommand) {
        self.notify(ListEvent::new(ListAction::Next(
            selected_command.current_idx,
        )))
        .await;

        self.select_command(selected_command).await;
    }

    pub async fn previous_command(&mut self, selected_command: SelectedCommand) {
        self.notify(ListEvent::new(ListAction::Previous(
            selected_command.current_idx,
        )))
        .await;

        self.select_command(selected_command).await;
    }

    pub async fn next_tab(&mut self, selected_namespace: SelectedNamespace) {
        self.notify(TabsEvent::Next(selected_namespace.idx)).await;
    }

    pub async fn previous_tab(&mut self, selected_namespace: SelectedNamespace) {
        self.notify(TabsEvent::Previous(selected_namespace.idx))
            .await;
    }

    async fn notify<E: Event>(&mut self, event: E) {
        self.get_publisher(TypeId::of::<E>()).notify(event).await;
    }

    fn get_publisher(&mut self, id: TypeId) -> &mut PublisherContainer {
        let active_screen = self.screens.get_active_screen_mut();
        active_screen.get_publisher(id)
    }
}

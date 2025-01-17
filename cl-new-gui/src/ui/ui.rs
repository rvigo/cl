use crate::component::{List, Tabs, TextBox};
use crate::observer::event::Event;
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
        let event = Event::UpdateCommand(items[0].clone());
        self.notify(TypeId::of::<TextBox>(), event).await;
    }

    pub async fn update_list_items(&mut self, items: Vec<String>) {
        self.notify(TypeId::of::<List>(), Event::UpdateAll(items))
            .await;
    }

    pub async fn update_tabs(&mut self, namespaces: Vec<String>) {
        self.notify(TypeId::of::<Tabs>(), Event::UpdateAll(namespaces))
            .await;
    }

    pub async fn select_command(&mut self, selected_command: SelectedCommand) {
        self.notify(
            TypeId::of::<TextBox>(),
            Event::UpdateCommand(selected_command.value.clone()),
        )
        .await;

        self.selected_command = selected_command;
    }

    pub async fn next_command(&mut self, selected_command: SelectedCommand) {
        self.notify(
            TypeId::of::<List>(),
            Event::Next(selected_command.current_idx),
        )
        .await;

        self.select_command(selected_command).await;
    }

    pub async fn previous_command(&mut self, selected_command: SelectedCommand) {
        self.notify(
            TypeId::of::<List>(),
            Event::Previous(selected_command.current_idx),
        )
        .await;

        self.select_command(selected_command).await;
    }

    pub async fn next_tab(&mut self, selected_namespace: SelectedNamespace) {
        self.notify(TypeId::of::<Tabs>(), Event::Next(selected_namespace.idx))
            .await;
    }

    pub async fn previous_tab(&mut self, selected_namespace: SelectedNamespace) {
        self.notify(
            TypeId::of::<Tabs>(),
            Event::Previous(selected_namespace.idx),
        )
        .await;
    }

    pub async fn modify_popup(&mut self) {
        // self.notify(PopupEvent::Action(PopupAction::Confirm)).await
    }

    async fn notify(&mut self, id: TypeId, event: Event) {
        self.screens.notify(id, event);
    }
}

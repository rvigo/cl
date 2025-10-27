use crate::component::{List, Tabs, TextBox};
use crate::observer::event::Event;
use crate::screen::Screen;
use crate::state::selected_command::SelectedCommand;
use std::any::TypeId;

pub mod ui_actor;

pub struct Ui {
    pub selected_command: SelectedCommand,
    pub screens: Screen,
}
impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui {
    pub fn new() -> Ui {
        Self {
            selected_command: SelectedCommand::default(),
            screens: Screen::new(),
        }
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

    async fn notify(&mut self, id: TypeId, event: Event) {
        self.screens.notify(id, event).await;
    }
}

use crate::component::{TextBox, TextBoxName};
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;

impl Observable for TextBox {
    fn on_listen(&mut self, event: Event) {
        let command = match event {
            Event::UpdateCommand(cmd) => cmd,
            _ => return,
        };

        let content = match self.name {
            TextBoxName::Command => command.command.to_string(),
            TextBoxName::Description => command.description(),
            TextBoxName::Tags => command.tags_as_string(),
            TextBoxName::Namespace => command.namespace.to_string(),
        };

        self.update_content(content);
    }
}

impl ObservableComponent for TextBox {}

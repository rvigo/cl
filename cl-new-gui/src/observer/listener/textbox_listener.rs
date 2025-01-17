use crate::component::{TextBox, TextBoxName};
use crate::observer::event::TextboxEvent;
use crate::observer::listener::Observable;

impl Observable for TextBox {
    type EventType = TextboxEvent;

    fn on_listen(&mut self, event: TextboxEvent) {
        let command = match event {
            TextboxEvent::UpdateCommand(cmd) => cmd,
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

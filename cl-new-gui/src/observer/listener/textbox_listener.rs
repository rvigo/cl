use crate::component::{TextBox, TextBoxName};
use crate::observer::event::TextboxEvent;
use crate::observer::listener::{Listener, ListenerId};

impl Listener for TextBox {
    type EventType = TextboxEvent;

    fn get_id() -> ListenerId {
        ListenerId("Textbox".to_string())
    }

    async fn on_event(&mut self, event: TextboxEvent) {
        let content = match self.name {
            TextBoxName::Command => event.command.command.to_string(),
            TextBoxName::Description => event.command.description(),
            TextBoxName::Tags => event.command.tags_as_string(),
            TextBoxName::Namespace => event.command.namespace.to_string(),
        };

        self.update_content(content);
    }
}

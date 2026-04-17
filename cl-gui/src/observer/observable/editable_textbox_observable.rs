use crate::component::EditableTextbox;
use crate::observer::event::{EditableTextboxEvent, Event};
use crate::observer::observable::textbox_observable::cow_some_or_none;
use crate::observer::observable::{Observable, ObservableFuture};
use crate::state::state_event::FieldName;
use crate::state::state_event::StateEvent::EditField;
use tracing::debug;

impl Observable for EditableTextbox {
    fn on_listen(&mut self, event: Event) -> Option<ObservableFuture> {
        if let Event::EditableTextbox(e) = event {
            match e {
                EditableTextboxEvent::UpdateCommand(command) => {
                    debug!("EditableTextbox({}): loading command fields", self.name);
                    let content = match self.name {
                        FieldName::Command => cow_some_or_none(command.command),
                        FieldName::Description => command.description.map(|d| d.to_string()),
                        FieldName::Tags => command
                            .tags
                            .map(|v| v.iter().map(|c| c.as_ref()).collect::<Vec<_>>().join(", ")),
                        FieldName::Namespace => cow_some_or_none(command.namespace),
                        FieldName::Alias => cow_some_or_none(command.alias),
                    };
                    self.update_content(content);
                }
                EditableTextboxEvent::KeyInput(key) => {
                    if self.is_active() {
                        debug!("EditableTextbox({}): handling key input", self.name);
                        self.handle_key_event(key);
                        self.modified = true;
                    }
                }
                EditableTextboxEvent::GetFieldContent(state_tx) => {
                    let content = self.textarea.lines().join("\n");
                    let name = self.name;
                    debug!("EditableTextbox({}): sending content '{}'", name, content);
                    return Some(Box::pin(async move {
                        if let Err(e) = state_tx.send(EditField(name, content)).await {
                            tracing::error!(
                                "EditableTextbox({}): failed to send field content: {e}",
                                name
                            );
                        }
                    }));
                }
                EditableTextboxEvent::SetField(field) => {
                    let matches = self.name == field;
                    debug!(
                        "EditableTextbox({}): SetField({:?}) → active={}",
                        self.name, field, matches
                    );
                    self.set_active(matches);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::state_event::FieldName;

    #[tokio::test]
    async fn set_field_activates_matching_textbox() {
        let mut tb = EditableTextbox {
            name: FieldName::Command,
            ..Default::default()
        };
        let event = Event::EditableTextbox(EditableTextboxEvent::SetField(FieldName::Command));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(tb.is_active());
    }

    #[tokio::test]
    async fn set_field_deactivates_non_matching_textbox() {
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            active: true,
            ..Default::default()
        };
        let event = Event::EditableTextbox(EditableTextboxEvent::SetField(FieldName::Command));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(!tb.is_active());
    }

    #[tokio::test]
    async fn wrong_event_variant_is_ignored() {
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            ..Default::default()
        };
        let event = Event::List(crate::observer::event::ListEvent::Next(0));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(!tb.is_active());
    }

    #[tokio::test]
    async fn key_input_marks_active_field_as_modified() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            active: true,
            ..Default::default()
        };
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let event = Event::EditableTextbox(EditableTextboxEvent::KeyInput(key));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(tb.modified);
    }

    #[tokio::test]
    async fn key_input_does_not_mark_inactive_field_as_modified() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            active: false,
            ..Default::default()
        };
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let event = Event::EditableTextbox(EditableTextboxEvent::KeyInput(key));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(!tb.modified);
    }

    #[tokio::test]
    async fn update_command_does_not_mark_field_as_modified() {
        use cl_core::Command;
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            active: true,
            ..Default::default()
        };
        let cmd = Command {
            alias: "my-alias".into(),
            namespace: "ns".into(),
            command: "echo hi".into(),
            description: None,
            tags: None,
        };
        let event = Event::EditableTextbox(EditableTextboxEvent::UpdateCommand(cmd));
        if let Some(fut) = tb.on_listen(event) {
            fut.await;
        }
        assert!(!tb.modified);
    }
}

use crate::component::{EditableTextbox, EditableTextboxName};
use crate::observer::event::{EditableTextboxEvent, Event};
use crate::observer::observable::textbox_observable::SomeOrNone;
use crate::observer::observable::Observable;
use crate::state::state_event::FieldName;
use crate::state::state_event::StateEvent::EditField;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for EditableTextbox {
    async fn on_listen(&mut self, event: Event) {
        if let Event::EditableTextbox(e) = event {
            match e {
                EditableTextboxEvent::UpdateCommand(command) => {
                    debug!("EditableTextbox({}): loading command fields", self.name);
                    let content = match self.name {
                        EditableTextboxName::Command => command.command.some_or_none(),
                        EditableTextboxName::Description => {
                            command.description.map(|d| d.to_string())
                        }
                        EditableTextboxName::Tags => command.tags.map(|v| {
                            v.iter().map(|c| c.as_ref()).collect::<Vec<_>>().join(", ")
                        }),
                        EditableTextboxName::Namespace => command.namespace.some_or_none(),
                        EditableTextboxName::Alias => command.alias.some_or_none(),
                    };
                    self.update_content(content);
                }
                EditableTextboxEvent::KeyInput(key) => {
                    if self.is_active() {
                        debug!("EditableTextbox({}): handling key input", self.name);
                        self.handle_key_event(key);
                    }
                }
                EditableTextboxEvent::GetFieldContent(state_tx) => {
                    let content = self.textarea.lines().join("\n");
                    debug!("EditableTextbox({}): sending content '{}'", self.name, content);
                    let field = self.name.to_field_name();
                    state_tx.send(EditField(field, content)).await.ok();
                }
                EditableTextboxEvent::SetField(field) => {
                    let matches = self.name.to_field_name() == field;
                    debug!(
                        "EditableTextbox({}): SetField({:?}) → active={}",
                        self.name, field, matches
                    );
                    self.set_active(matches);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: map EditableTextboxName → FieldName
// ---------------------------------------------------------------------------

pub trait EditableTextboxNameExt {
    fn to_field_name(&self) -> FieldName;
}

impl EditableTextboxNameExt for EditableTextboxName {
    fn to_field_name(&self) -> FieldName {
        match self {
            EditableTextboxName::Command => FieldName::Command,
            EditableTextboxName::Description => FieldName::Description,
            EditableTextboxName::Tags => FieldName::Tags,
            EditableTextboxName::Namespace => FieldName::Namespace,
            EditableTextboxName::Alias => FieldName::Alias,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::state_event::FieldName;

    #[test]
    fn set_field_activates_matching_textbox() {
        let mut tb = EditableTextbox {
            name: EditableTextboxName::Command,
            ..Default::default()
        };
        // Use the sync path via tokio runtime for the async observable.
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            tb.on_listen(Event::EditableTextbox(EditableTextboxEvent::SetField(
                FieldName::Command,
            )))
            .await;
        });
        assert!(tb.is_active());
    }

    #[test]
    fn set_field_deactivates_non_matching_textbox() {
        let mut tb = EditableTextbox {
            name: EditableTextboxName::Alias,
            active: true,
            ..Default::default()
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            tb.on_listen(Event::EditableTextbox(EditableTextboxEvent::SetField(
                FieldName::Command,
            )))
            .await;
        });
        assert!(!tb.is_active());
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut tb = EditableTextbox {
            name: EditableTextboxName::Alias,
            ..Default::default()
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            tb.on_listen(Event::List(crate::observer::event::ListEvent::Next(0)))
                .await;
        });
        // No panic, no state change.
        assert!(!tb.is_active());
    }
}

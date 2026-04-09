use crate::component::EditableTextbox;
use crate::observer::event::{EditableTextboxEvent, Event};
use crate::observer::observable::textbox_observable::SomeOrNone;
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
                        FieldName::Command => command.command.some_or_none(),
                        FieldName::Description => {
                            command.description.map(|d| d.to_string())
                        }
                        FieldName::Tags => command.tags.map(|v| {
                            v.iter().map(|c| c.as_ref()).collect::<Vec<_>>().join(", ")
                        }),
                        FieldName::Namespace => command.namespace.some_or_none(),
                        FieldName::Alias => command.alias.some_or_none(),
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
                    let name = self.name.clone();
                    debug!("EditableTextbox({}): sending content '{}'", name, content);
                    return Some(Box::pin(async move {
                        if let Err(e) = state_tx.send(EditField(name.clone(), content)).await {
                            tracing::error!("EditableTextbox({}): failed to send field content: {e}", name);
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

    #[test]
    fn set_field_activates_matching_textbox() {
        let mut tb = EditableTextbox {
            name: FieldName::Command,
            ..Default::default()
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let event = Event::EditableTextbox(EditableTextboxEvent::SetField(FieldName::Command));
            if let Some(fut) = tb.on_listen(event) {
                fut.await;
            }
        });
        assert!(tb.is_active());
    }

    #[test]
    fn set_field_deactivates_non_matching_textbox() {
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            active: true,
            ..Default::default()
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let event = Event::EditableTextbox(EditableTextboxEvent::SetField(FieldName::Command));
            if let Some(fut) = tb.on_listen(event) {
                fut.await;
            }
        });
        assert!(!tb.is_active());
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut tb = EditableTextbox {
            name: FieldName::Alias,
            ..Default::default()
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let event = Event::List(crate::observer::event::ListEvent::Next(0));
            if let Some(fut) = tb.on_listen(event) {
                fut.await;
            }
        });
        assert!(!tb.is_active());
    }
}

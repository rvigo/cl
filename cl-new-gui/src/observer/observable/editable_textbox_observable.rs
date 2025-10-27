use crate::component::{EditableTextbox, EditableTextboxName};
use crate::observer::event::{EditEvent, Event};
use crate::observer::observable::textbox_observable::SomeOrNone;
use crate::observer::observable::Observable;
use crate::state::state_event::FieldName;
use crate::state::state_event::StateEvent::EditField;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for EditableTextbox {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::UpdateCommand(command) => {
                debug!("updating fields for {}", self.name);
                let content = match self.name {
                    EditableTextboxName::Command => command.command.some_or_none(),
                    EditableTextboxName::Description => {
                        command.description.map(|desc| desc.to_string())
                    }
                    EditableTextboxName::Tags => command.tags.map(|vec| {
                        vec.iter()
                            .map(|cow| cow.as_ref())
                            .collect::<Vec<_>>()
                            .join(", ")
                    }),

                    EditableTextboxName::Namespace => command.namespace.some_or_none(),
                    EditableTextboxName::Alias => command.alias.some_or_none(),
                };

                self.update_content(content);
            }
            Event::KeyEvent(key) => {
                if self.is_active() {
                    debug!("Handling key event for {}", self.name);
                    self.handle_key_event(key)
                }
            }
            Event::GetFieldContent(state_tx) => {
                let content = self.textarea.lines().join("\n");
                debug!("sending content for {}: {}", self.name, content);

                match self.name {
                    EditableTextboxName::Command => state_tx
                        .send(EditField(FieldName::Command, content))
                        .await
                        .ok(),
                    EditableTextboxName::Description => state_tx
                        .send(EditField(FieldName::Description, content))
                        .await
                        .ok(),
                    EditableTextboxName::Tags => state_tx
                        .send(EditField(FieldName::Tags, content))
                        .await
                        .ok(),
                    EditableTextboxName::Namespace => state_tx
                        .send(EditField(FieldName::Namespace, content))
                        .await
                        .ok(),
                    EditableTextboxName::Alias => state_tx
                        .send(EditField(FieldName::Alias, content))
                        .await
                        .ok(),
                };
            }
            Event::Edit(cmd) => match cmd {
                EditEvent::SetField(field) => {
                    if self.match_name_to_field() == field {
                        debug!("field {:?} matches, activating", field);
                        self.set_active(true);
                    } else {
                        debug!("field {:?} does not match, deactivating", field);
                        self.set_active(false);
                    }
                }
            },
            _ => {}
        }
    }
}

pub trait EditableTextboxExt {
    fn match_name_to_field(&self) -> FieldName;
}

impl EditableTextboxExt for EditableTextbox {
    fn match_name_to_field(&self) -> FieldName {
        match self.name {
            EditableTextboxName::Command => FieldName::Command,
            EditableTextboxName::Description => FieldName::Description,
            EditableTextboxName::Tags => FieldName::Tags,
            EditableTextboxName::Namespace => FieldName::Namespace,
            EditableTextboxName::Alias => FieldName::Alias,
        }
    }
}

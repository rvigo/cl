use crate::observer::event::{EditEvent, Event};
use crate::observer::observable::Observable;
use crate::state::state_event::FieldType;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for FieldType {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Edit(edit) => match edit {
                EditEvent::SetField(field) => {
                    debug!("changing field {:?} to {:?}", self, field);
                    *self = field;
                }
            },
            _ => {}
        }
    }
}

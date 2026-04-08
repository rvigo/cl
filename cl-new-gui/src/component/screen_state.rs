use crate::state::state_event::FieldName;

#[derive(Debug)]
pub struct ScreenState {
    pub current_field: FieldName,
    pub has_changes: bool,
}

impl ScreenState {
    pub fn new(current_field: FieldName) -> Self {
        Self {
            current_field,
            has_changes: false,
        }
    }
}

impl crate::observer::event::NotifyTarget for ScreenState {
    type Payload = crate::observer::event::ScreenStateEvent;
    fn wrap(payload: Self::Payload) -> crate::observer::event::Event {
        crate::observer::event::Event::ScreenState(payload)
    }
}

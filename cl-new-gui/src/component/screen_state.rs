use crate::state::state_event::FieldType;

#[derive(Debug)]
pub struct ScreenState {
    pub current_field: FieldType,
    pub has_changes: bool,
}

impl ScreenState {
    pub fn new(current_field: FieldType) -> Self {
        Self {
            current_field,
            has_changes: false,
        }
    }
}

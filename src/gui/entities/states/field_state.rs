use super::state::State;
pub use crate::gui::widgets::text_field::FieldType;

#[derive(Default, Clone)]
pub struct FieldState {
    selected: Option<FieldType>,
}

impl State for FieldState {
    type Output = Option<FieldType>;

    fn selected(&self) -> Option<FieldType> {
        self.selected.to_owned()
    }

    fn select(&mut self, field_type: Option<FieldType>) {
        self.selected = field_type;
    }
}

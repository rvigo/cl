use crate::gui::widgets::text_field::FieldType;

#[derive(Default, Clone)]
pub struct FieldState {
    selected: Option<FieldType>,
}

impl FieldState {
    pub fn selected(&self) -> Option<FieldType> {
        self.selected.to_owned()
    }

    pub fn select(&mut self, field_type: Option<FieldType>) {
        self.selected = field_type;
    }
}

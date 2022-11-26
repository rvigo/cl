use crate::gui::layouts::get_style;

use super::field::{Field, FieldType};
use std::ops::{Deref, DerefMut};
use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders},
};

#[derive(Default)]
pub struct Fields<'a>(pub Vec<Field<'a>>);

impl<'a> Fields<'a> {
    pub fn build_form_fields() -> Fields<'a> {
        let alias = forms_widget_factory("Alias".to_string(), FieldType::Alias, true, false);
        let namespace =
            forms_widget_factory("Namespace".to_string(), FieldType::Namespace, false, false);
        let command = forms_widget_factory("Command".to_string(), FieldType::Command, false, true);
        let description = forms_widget_factory(
            "Description".to_string(),
            FieldType::Description,
            false,
            true,
        );

        let tags = forms_widget_factory("Tags".to_string(), FieldType::Tags, false, false);
        Fields(vec![alias, namespace, command, description, tags])
    }
}

impl<'a> Deref for Fields<'a> {
    type Target = Vec<Field<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fields<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn forms_widget_factory(
    title: String,
    field_type: FieldType,
    in_focus: bool,
    multiline: bool,
) -> Field<'static> {
    let mut field = Field::new(title.clone(), field_type, in_focus, multiline);
    field.block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {} ", title))
            .border_type(BorderType::Plain),
    );
    field.style(get_style(in_focus));

    field
}

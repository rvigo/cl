use super::field::{Field, FieldType};
use crate::gui::layouts::get_style;
use std::ops::{Deref, DerefMut};
use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders},
};

pub struct Fields<'a>(pub Vec<Field<'a>>);

impl<'a> Default for Fields<'a> {
    fn default() -> Self {
        let alias = forms_widget_factory(FieldType::Alias, true, false);
        let namespace = forms_widget_factory(FieldType::Namespace, false, false);
        let command = forms_widget_factory(FieldType::Command, false, true);
        let description = forms_widget_factory(FieldType::Description, false, true);
        let tags = forms_widget_factory(FieldType::Tags, false, false);

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

fn forms_widget_factory<'a>(field_type: FieldType, in_focus: bool, multiline: bool) -> Field<'a> {
    let title = field_type.to_string();
    let mut field = Field::new(&title, field_type, in_focus, multiline);
    field.block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {} ", &title))
            .border_type(BorderType::Plain),
    );
    field.style(get_style(in_focus));

    field
}

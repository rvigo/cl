use super::field::{Field, FieldType};
use crate::gui::layouts::{get_style, TerminalSize};
use log::debug;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders},
};

const ORDER_SMALL_SIZE: &[FieldType] = &[
    FieldType::Alias,
    FieldType::Namespace,
    FieldType::Description,
    FieldType::Tags,
    FieldType::Command,
];

const ORDER_MEDIUM_SIZE: &[FieldType] = &[
    FieldType::Alias,
    FieldType::Namespace,
    FieldType::Command,
    FieldType::Description,
    FieldType::Tags,
];

pub struct Fields<'a> {
    items: HashMap<FieldType, Field<'a>>,
    order: Vec<FieldType>,
}

impl<'a> Fields<'a> {
    pub fn get_fields(&self) -> Vec<Field<'a>> {
        let mut ordered_fields = vec![];

        self.order.iter().for_each(|i| {
            if let Some(f) = self.items.get(i) {
                ordered_fields.push(f.to_owned())
            }
        });

        ordered_fields
    }

    pub fn reorder(&mut self, size: &TerminalSize) {
        debug!("reordering fields to '{size:?}'");
        let order = match size {
            TerminalSize::Small => ORDER_SMALL_SIZE.to_owned(),
            TerminalSize::Medium | TerminalSize::Large => ORDER_MEDIUM_SIZE.to_owned(),
        };

        self.order = order
    }

    pub fn get_field_mut(&mut self, field_type: &FieldType) -> Option<&mut Field<'a>> {
        self.items.get_mut(field_type)
    }

    pub fn get_order(&self) -> Vec<FieldType> {
        self.order.to_owned()
    }
}

impl Default for Fields<'_> {
    fn default() -> Self {
        let alias = forms_widget_factory(FieldType::Alias, true, false);
        let namespace = forms_widget_factory(FieldType::Namespace, false, false);
        let command = forms_widget_factory(FieldType::Command, false, true);
        let description = forms_widget_factory(FieldType::Description, false, true);
        let tags = forms_widget_factory(FieldType::Tags, false, false);

        let map = vec![alias, namespace, command, description, tags]
            .into_iter()
            .map(|f| (f.field_type.to_owned(), f))
            .collect();

        Fields {
            items: map,
            order: ORDER_MEDIUM_SIZE.to_owned(),
        }
    }
}

impl<'a> Deref for Fields<'a> {
    type Target = HashMap<FieldType, Field<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Fields<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
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

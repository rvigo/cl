use super::text_field::{FieldType, TextField};
use crate::gui::screens::ScreenSize;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
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

#[derive(Clone)]
pub struct Fields<'a> {
    items: HashMap<FieldType, TextField<'a>>,
    order: Vec<FieldType>,
}

impl<'a> Fields<'a> {
    pub fn get_fields_iter(&self) -> impl Iterator<Item = TextField<'a>> {
        let mut ordered_fields = vec![];

        self.order.iter().for_each(|i| {
            if let Some(f) = self.items.get(i) {
                ordered_fields.push(f.to_owned())
            }
        });

        ordered_fields.into_iter()
    }

    pub fn reorder_by_screen_size(&mut self, size: &ScreenSize) {
        let order = match size {
            ScreenSize::Small => ORDER_SMALL_SIZE.to_owned(),
            ScreenSize::Medium | ScreenSize::Large => ORDER_MEDIUM_SIZE.to_owned(),
        };

        self.order = order
    }

    pub fn get_field_mut(&mut self, field_type: &FieldType) -> Option<&mut TextField<'a>> {
        self.items.get_mut(field_type)
    }

    pub fn get_order(&self) -> Vec<FieldType> {
        self.order.to_owned()
    }

    pub fn clear_inputs(&mut self) {
        self.items
            .iter_mut()
            .for_each(|(_, field)| field.clear_input());
    }
}

impl<'a> From<(HashMap<FieldType, TextField<'a>>, Vec<FieldType>)> for Fields<'a> {
    fn from((items, order): (HashMap<FieldType, TextField<'a>>, Vec<FieldType>)) -> Self {
        Fields { items, order }
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
            .map(|f| (f.field_type(), f))
            .collect();

        Fields {
            items: map,
            order: ORDER_MEDIUM_SIZE.to_owned(),
        }
    }
}

impl<'a> Deref for Fields<'a> {
    type Target = HashMap<FieldType, TextField<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Fields<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

fn forms_widget_factory<'a>(
    field_type: FieldType,
    in_focus: bool,
    multiline: bool,
) -> TextField<'a> {
    let title = field_type.to_string();
    TextField::new(title, field_type, in_focus, multiline)
}

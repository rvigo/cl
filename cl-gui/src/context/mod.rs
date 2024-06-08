mod application;
mod commands_context;
mod fields;
mod namespace_context;
mod popup_context;
mod ui;

pub use application::Application;
pub use commands_context::CommandsContext;
pub use popup_context::PopupContext;
pub use ui::UI;

use crate::widget::text_field::FieldType;
use cl_core::Command;
use std::{collections::HashMap, ops::Deref};

pub trait Selectable {
    fn next(&mut self);

    fn previous(&mut self);
}

/// Convets a type into an Option
pub trait ToOption {
    fn to_option(&self) -> Option<Self>
    where
        Self: Sized;
}

impl ToOption for String {
    fn to_option(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

impl ToOption for Vec<String> {
    fn to_option(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_empty() || self.iter().all(String::is_empty) {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

pub struct FieldMap(pub HashMap<FieldType, String>);

impl Deref for FieldMap {
    type Target = HashMap<FieldType, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FieldMap> for Command {
    fn from(value: FieldMap) -> Self {
        let namespace = value.get(&FieldType::Namespace).unwrap();
        let alias = value.get(&FieldType::Alias).unwrap();
        let description = value.get(&FieldType::Description).unwrap();
        let command = value.get(&FieldType::Command).unwrap();
        let tags = value.get(&FieldType::Tags).unwrap();
        let tags = tags
            .split(',')
            .map(|tag| String::from(tag.trim()))
            .filter(|tag| !tag.is_empty())
            .collect::<Vec<_>>()
            .to_option();

        Command {
            namespace: namespace.to_string(),
            alias: alias.to_string(),
            description: description.to_string().to_option(),
            command: command.to_string(),
            tags,
        }
    }
}

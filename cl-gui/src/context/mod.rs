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
use std::{borrow::Cow, collections::HashMap, ops::Deref};

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

impl ToOption for Cow<'_, str> {
    fn to_option(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl ToOption for Vec<Cow<'_, str>> {
    fn to_option(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_empty() || self.iter().all(|s| s.is_empty()) {
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

// FIXME check all the unwraps
impl From<FieldMap> for Command<'_> {
    fn from(value: FieldMap) -> Self {
        let namespace = value.get(&FieldType::Namespace).unwrap().to_string();
        let alias = value.get(&FieldType::Alias).unwrap().to_string();
        let description = value.get(&FieldType::Description).unwrap().to_string();
        let command = value.get(&FieldType::Command).unwrap().to_string();
        let tags = value.get(&FieldType::Tags).unwrap();
        let tags = tags
            .split(',')
            .map(|tag| Cow::Owned::<str>(tag.trim().to_owned()))
            .filter(|tag| !tag.is_empty())
            .collect::<Vec<_>>();

        Command {
            namespace: Cow::Owned(namespace),
            alias: Cow::Owned(alias),
            description: Cow::Owned::<str>(description).to_option(),
            command: Cow::Owned(command),
            tags: tags.to_option(),
        }
    }
}

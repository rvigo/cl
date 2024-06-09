mod display;
mod highlight;
pub mod list;
mod macros;

pub mod popup;
pub mod statusbar;
pub mod text_field;

pub use display::DisplayWidget;
pub use text_field::TextField;

use crossterm::event::KeyEvent;
use std::ops::Deref;

/// Handles use key input
pub trait WidgetKeyHandler {
    fn handle_input(&mut self, input: KeyEvent);
}

#[macro_export]
macro_rules! create_fields_map {
    ($($field_type:path :{ focus = $focus:expr,multiline = $multiline:expr }),+ $(,)*) => {{
        use $crate::widget::text_field::TextFieldBuilder;
        let mut fields = cl_core::hashmap!();
            $(
            fields.insert( $field_type,
                TextFieldBuilder::default()
                    .field_type($field_type)
                    .in_focus($focus)
                    .multiline($multiline)
                    .build());
            )+

            fields
    }};
}

#[derive(Default)]
pub struct Lines(pub Vec<String>);

impl Deref for Lines {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Lines {
    fn from(value: String) -> Self {
        let inner = value.lines().map(String::from).collect();
        Lines(inner)
    }
}

impl<'a> From<&'a String> for Lines {
    fn from(value: &'a String) -> Self {
        value.to_owned().into()
    }
}

impl From<Option<&String>> for Lines {
    fn from(value: Option<&String>) -> Self {
        match value {
            Some(content) => content.into(),
            None => Lines::default(),
        }
    }
}

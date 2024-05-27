pub mod alias_list;
pub mod base_widget;
pub mod display;
pub mod field_state;
pub mod highlight;
pub mod macros;
pub mod popup;
pub mod statusbar;
pub mod text_field;

use self::statusbar::StatusBarItem;
use crossterm::event::KeyEvent;

/// Handles use key input
pub trait WidgetKeyHandler {
    fn handle_input(&mut self, input: KeyEvent);
}

#[macro_export]
macro_rules! create_fields_map {
    ($($field_type:path :{ focus = $focus:expr,multiline = $multiline:expr }),+ $(,)*) => {{
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

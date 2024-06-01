mod alias_list;
mod base_widget;
mod display;
mod highlight;
mod macros;
pub mod popup;
pub mod statusbar;
pub mod text_field;

pub use alias_list::AliasListWidget;
pub use base_widget::BaseWidget;
pub use display::DisplayWidget;
pub use text_field::TextField;

use self::statusbar::StatusBarItem;
use crossterm::event::KeyEvent;

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

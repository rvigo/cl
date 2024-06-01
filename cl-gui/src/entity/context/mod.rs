mod application_context;
mod commands_context;
mod fields;
mod namespaces;
mod popup_context;
mod ui;

pub use application_context::ApplicationContext;
pub use popup_context::PopupContext;
pub use ui::UI;

pub trait Selectable {
    fn next(&mut self);

    fn previous(&mut self);
}

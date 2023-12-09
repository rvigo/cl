pub mod application_context;
pub mod commands_context;
mod fields;
pub mod namespaces;
pub mod popup_context;
pub mod ui;

pub trait Selectable {
    fn next(&mut self);

    fn previous(&mut self);
}

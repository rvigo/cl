pub mod application_context;
pub mod cache_info;
pub mod commands_context;
mod field_context;
pub mod namespaces_context;
mod popup_context;
pub mod ui_context;

trait Selectable {
    fn next(&mut self);

    fn previous(&mut self);
}

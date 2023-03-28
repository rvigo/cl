mod edit_handler;
pub mod input_handler;
mod insert_handler;
mod main_handler;
mod popup_handler;

use super::entities::application_context::ApplicationContext;
use crossterm::event::KeyEvent;
use parking_lot::{lock_api::MutexGuard, RawMutex};

fn handle_help(application_context: &mut MutexGuard<RawMutex, ApplicationContext>) {
    application_context.set_show_help(false);
}

fn handle_querybox_input(
    key_event: KeyEvent,
    application_context: &mut MutexGuard<RawMutex, ApplicationContext>,
) {
    application_context.handle_querybox_input(key_event);
    application_context.filter_commands();
    application_context.filter_namespaces();
}

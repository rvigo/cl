mod edit_handler;
mod insert_handler;
mod main_handler;
mod popup_handler;

use self::{
    edit_handler::EditHandler, insert_handler::InsertHandler, main_handler::MainHandler,
    popup_handler::PopupHandler,
};
use super::{entities::application_context::ApplicationContext, layouts::ViewMode};
use crossterm::event::KeyEvent;

pub trait KeyHandler {
    fn handle(&self, key_event: KeyEvent, application_context: &mut ApplicationContext);
}

pub fn handle(key_event: KeyEvent, application_context: &mut ApplicationContext) {
    if application_context.popup().is_some() {
        PopupHandler::default().handle(key_event, application_context);
    } else if application_context.show_help() {
        handle_help(application_context)
    } else if application_context.querybox_focus() {
        handle_querybox_input(key_event, application_context)
    } else {
        let handler = get_handler(application_context.view_mode());
        handler.handle(key_event, application_context);
    }
}

fn handle_help(application_context: &mut ApplicationContext) {
    application_context.set_show_help(false);
}

fn handle_querybox_input(key_event: KeyEvent, application_context: &mut ApplicationContext) {
    application_context.handle_querybox_input(key_event);
    application_context.filter_commands();
    application_context.filter_namespaces();
}

fn get_handler(view_mode: &ViewMode) -> Box<dyn KeyHandler> {
    match view_mode {
        ViewMode::Main => Box::new(MainHandler),
        ViewMode::Insert => Box::new(InsertHandler),
        ViewMode::Edit => Box::new(EditHandler),
    }
}

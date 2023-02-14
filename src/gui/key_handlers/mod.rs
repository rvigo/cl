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

pub fn handle(key_event: KeyEvent, application_context: &mut ApplicationContext) {
    if application_context.ui_context.popup_context.popup.is_some() {
        PopupHandler::default().handle(key_event, application_context);
    } else if application_context.show_help() {
        handle_help(application_context)
    } else if application_context.ui_context.query_box.is_on_focus() {
        application_context.ui_context.query_box.handle(key_event);
        application_context.filter_commands();
    } else {
        let handler = get_handler(application_context.ui_context.view_mode());
        handler.handle(key_event, application_context);
    }
}

fn get_handler(view_mode: &ViewMode) -> Box<dyn KeyHandler> {
    match view_mode {
        ViewMode::Main => Box::new(MainHandler),
        ViewMode::Insert => Box::new(InsertHandler),
        ViewMode::Edit => Box::new(EditHandler),
    }
}

pub trait KeyHandler {
    fn handle(&self, key_event: KeyEvent, application_context: &mut ApplicationContext);
}

fn handle_help(application_context: &mut ApplicationContext) {
    application_context.set_show_help(false);
}

mod edit_handler;
mod insert_handler;
mod main_handler;
mod popup_handler;

use self::{
    edit_handler::EditHandler, insert_handler::InsertHandler, main_handler::MainHandler,
    popup_handler::PopupHandler,
};
use super::layouts::ViewMode;
use crate::gui::entities::state::State;
use crossterm::event::KeyEvent;

pub fn handle(key_event: KeyEvent, state: &mut State) {
    if state.popup_context.popup.is_some() {
        PopupHandler::default().handle(key_event, state);
    } else if state.show_help {
        handle_help(state)
    } else if state.query_box.is_on_focus() {
        state.query_box.handle(key_event)
    } else {
        let handler = get_handler(state.view_mode.clone());
        handler.handle(key_event, state);
    }
}

fn get_handler(view_mode: ViewMode) -> Box<dyn Handler> {
    match view_mode {
        ViewMode::Main => Box::new(MainHandler),
        ViewMode::Insert => Box::new(InsertHandler),
        ViewMode::Edit => Box::new(EditHandler),
    }
}

pub trait Handler {
    fn handle(&self, key_event: KeyEvent, state: &mut State);
}

fn handle_help(state: &mut State) {
    state.show_help = false;
}

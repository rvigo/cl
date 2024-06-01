mod edit_handler;
mod help_popup_handler;
mod insert_handler;
mod main_handler;
mod popup_handler;
mod querybox_handler;

pub use edit_handler::EditScreenHandler;
pub use help_popup_handler::HelpPopupHandler;
pub use insert_handler::InsertScreenHandler;
pub use main_handler::MainScreenHandler;
pub use popup_handler::PopupHandler;
pub use querybox_handler::QueryboxHandler;

use super::entity::event::AppEvent;
use anyhow::Result;
use crossterm::event::KeyEvent;

type AppEventResult = Result<Option<AppEvent>>;

/// (Almost) Every KeyEvent triggers an app event
pub trait KeyEventHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult;
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum HandlerType {
    Main,
    Insert,
    Edit,
    Popup,
    Help,
    QueryBox,
}

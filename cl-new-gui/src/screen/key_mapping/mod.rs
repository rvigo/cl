mod form_screen_key_mapping;
mod main_screen_key_mapping;
mod popup_key_mapping;
mod search_key_mapping;

pub mod command;

use crate::screen::key_mapping::command::ScreenCommand;
use crate::state::state_event::StateEvent;
use async_trait::async_trait;
use crossterm::event::KeyEvent;
use tokio::sync::mpsc::Sender;

#[async_trait(?Send)]
pub trait KeyMapping {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>>;
}

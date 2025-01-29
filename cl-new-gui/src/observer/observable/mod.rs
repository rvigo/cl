mod list_observable;
mod popup_observable;
mod tabs_observable;
mod textbox_observable;
mod search_observable;
mod clipboard_status;

use crate::observer::event::Event;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Observable {
    async fn on_listen(&mut self, event: Event);
}

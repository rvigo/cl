use async_trait::async_trait;
use downcast_rs::{impl_downcast, Downcast};
use crate::observer::event::Event;

pub mod list_observable;
mod popup_observable;
pub mod tabs_observable;
pub mod textbox_observable;


#[async_trait(?Send)]
pub trait Observable {

    async fn on_listen(&mut self, event: Event);
}


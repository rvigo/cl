use crate::component::ClipboardStatus;
use crate::observer::event::{ClipboardAction, Event};
use crate::observer::observable::Observable;
use async_trait::async_trait;

#[async_trait(?Send)]
impl Observable for ClipboardStatus {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Clipboard(action) => match action {
                ClipboardAction::Copied => self.start_counter(),
            },
            _ => {},
        }
    }
}

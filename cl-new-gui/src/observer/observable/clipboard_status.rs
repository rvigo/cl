use crate::component::ClipboardStatus;
use crate::observer::event::{ClipboardAction, Event};
use crate::observer::observable::Observable;
use async_trait::async_trait;

#[async_trait(?Send)]
impl Observable for ClipboardStatus {
    async fn on_listen(&mut self, event: Event) {
        if let Event::Clipboard(action) = event {
            match action {
                ClipboardAction::Copied => self.start_counter(),
            }
        }
    }
}

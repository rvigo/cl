use crate::component::ScreenState;
use crate::observer::event::{EditEvent, Event};
use crate::observer::observable::Observable;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for ScreenState {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Edit(cmd) => match cmd {
                EditEvent::SetField(field) => {
                    debug!("setting field to {:?}", field);
                    self.current_field = field;
                }
            },
            Event::KeyEvent(_) => {
                if self.has_changes {
                    debug!("state already registered changes")
                } else {
                    self.has_changes = true;
                    debug!("registered changes")
                }
            }
            _ => todo!(),
        }
    }
}

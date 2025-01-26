use crate::component::Popup;
use crate::observer::event::{Event, PopupEvent, PopupType};
use crate::observer::observable::Observable;
use crate::screen::{ActiveScreen, ScreenCommandCallback};
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for Popup {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Popup(popup) => match popup {
                PopupEvent::Create(type_) => match type_ {
                    PopupType::Dialog(message) => *self = Popup::dialog(message),
                    PopupType::Help(active_screen) => match active_screen {
                        ActiveScreen::Main => *self = Popup::help_main(),
                    },
                },
                PopupEvent::NextChoice => self.next(),
                PopupEvent::PreviousChoice => self.previous(),
                PopupEvent::Run(state_tx, tx) => {
                    debug!("running code inside the button");
                    self.click(state_tx).await;

                    debug!("sending a callback response to the previous layer");
                    tx.send(ScreenCommandCallback::UpdateAll).await.ok();
                }
            },
            _ => {}
        }
    }
}

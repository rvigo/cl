use crate::async_fn_body;
use crate::component::{FutureEventType, Popup};
use crate::observer::event::{Event, PopupEvent, PopupType};
use crate::observer::observable::Observable;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::ActiveScreen;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for Popup {
    async fn on_listen(&mut self, event: Event) {
        if let Event::Popup(popup) = event {
            match popup {
                PopupEvent::Create(type_) => match type_ {
                    PopupType::Dialog(message, yes_action, yes_callback) => {
                        *self = Popup::dialog(message, yes_action, yes_callback);
                    }
                    PopupType::Help(active_screen) => match active_screen {
                        ActiveScreen::Main => *self = Popup::help_main(),
                    },
                },
                PopupEvent::NextChoice => self.next(),
                PopupEvent::PreviousChoice => self.previous(),
                PopupEvent::Run(state_tx, tx) => {
                    debug!("Popup: running button click");
                    match self.click(state_tx).await {
                        Ok(callback) => {
                            debug!("Popup: sending callback to previous layer");
                            tx.send(callback).await.ok();
                        }
                        Err(err) => {
                            *self = Popup::dialog(
                                err.to_string(),
                                FutureEventType::State(|_| async_fn_body!(Ok(()))),
                                ScreenCommandCallback::DoNothing,
                            );
                        }
                    }
                }
            }
        }
    }
}

use crate::component::{Button, Popup};
use crate::observer::event::PopupAction::{Cancel, Confirm};
use crate::observer::event::{Event, PopupEvent, PopupType};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::state::state_event::StateEvent::ExecuteCommand;
use async_trait::async_trait;
use log::debug;

#[macro_export]
macro_rules! async_fn_body {
    ($($body:stmt);*) => {
        Box::pin(async move {
            $($body)*
        })
    };
}

#[async_trait(?Send)]
impl Observable for Popup {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Popup(popup) => match popup {
                PopupEvent::Create(type_) => match type_ {
                    // TODO move this to a popup factory
                    PopupType::Dialog(message) => {
                        self.title = "Warning".to_string();
                        self.content = message;
                        self.buttons = vec![
                            Button::new("Yes", Confirm, |state| {
                                async_fn_body! {
                                    let result = state.send(ExecuteCommand).await.ok();
                                    match result {
                                      Some(_) => { debug!("worked!") }
                                      None => { debug!("something went wrong") }
                                    }
                                }
                            }),
                            Button::new("No", Cancel, |_| {
                                async_fn_body! {
                                    debug!("asdasd")
                                }
                            }),
                        ]
                    }
                },
                PopupEvent::NextChoice => self.next(),
                PopupEvent::PreviousChoice => self.previous(),
                PopupEvent::Action(_) => {}
                PopupEvent::Run(state_tx) => self.run_button_callback(state_tx).await,
            },
            _ => {}
        }
    }
}

impl ObservableComponent for Popup {}

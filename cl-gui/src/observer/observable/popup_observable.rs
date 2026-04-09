use crate::component::Popup;
use crate::observer::event::{Event, PopupEvent, PopupType};
use crate::observer::observable::{Observable, ObservableFuture};
use crate::screen::ActiveScreen;
use tracing::debug;

impl Observable for Popup {
    fn on_listen(&mut self, event: Event) -> Option<ObservableFuture> {
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
                    // Extract button data while we hold &mut self; return owned future
                    if self.buttons.is_empty() {
                        debug!("No buttons to click");
                        return None;
                    }
                    let selected_idx = self.state.selected();
                    let callback = self.buttons[selected_idx].callback.clone();
                    let on_click = self.buttons[selected_idx].on_click.clone();

                    return Some(Box::pin(async move {
                        match on_click.call(Some(state_tx), None).await {
                            Ok(()) => {
                                debug!("Popup: sending callback to previous layer");
                                if let Err(e) = tx.send(callback).await {
                                    tracing::error!("Popup: failed to send callback: {e}");
                                }
                            }
                            Err(err) => {
                                // Previous code did `*self = Popup::dialog(...)` but self was
                                // already being popped by PopLastLayer, so error was never shown.
                                // Log it instead for better observability.
                                tracing::error!("Popup: button click failed: {err}");
                            }
                        }
                    }));
                }
            }
        }
        None
    }
}

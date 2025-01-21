use crate::component::{Button, Popup};
use crate::observer::event::Event;
use crate::observer::event::PopupAction::Confirm;
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use log::debug;

impl Observable for Popup {
    fn on_listen(&mut self, event: Event) {
        
        // FIXME adjust the proper events
        match event {
            Event::Next(_) => {
                self.title = "Warning".to_string();
                self.content = "Are you sure you want to continue?".to_string();
                self.buttons = vec![Button::new("Yest", Confirm, || {
                    debug!("Yes clicked");
                })];
            }
            _ => {}
        }
        // match event {
        //     PopupEvent::ShowPopup => {}
        //     PopupEvent::HidePopup => {}
        //     PopupEvent::Action(action) => match action {
        //         PopupAction::Confirm => {
        //             for button in &self.buttons {
        //                 if button.action == PopupAction::Confirm {
        //                     (button.on_click)();
        //                 }
        //             }
        //         }
        //         PopupAction::Cancel => {
        //             debug!("Canceling")
        //         }
        //     },
        // }
    }
}

impl ObservableComponent for Popup {}

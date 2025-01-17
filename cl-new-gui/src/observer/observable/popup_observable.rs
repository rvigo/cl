use crate::component::Popup;
use crate::observer::event::{Event, PopupAction, PopupEvent};
use crate::observer::observable::Observable;

impl Observable for Popup {

    fn on_listen(&mut self, event: Event) {
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

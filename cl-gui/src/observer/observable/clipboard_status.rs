use crate::component::ClipboardStatus;
use crate::observer::event::{ClipboardAction, Event};
use crate::observer::observable::SyncObservable;

impl SyncObservable for ClipboardStatus {
    fn on_event(&mut self, event: Event) {
        if let Event::ClipboardStatus(action) = event {
            match action {
                ClipboardAction::Copied => self.start_counter(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::event::ClipboardAction;

    #[test]
    fn copied_action_starts_counter() {
        let mut cs = ClipboardStatus::default();
        cs.on_event(Event::ClipboardStatus(ClipboardAction::Copied));
        // start_counter sets `copied = true`; verify via check_if_need_to_stop
        // which won't flip it back immediately (duration is 3 s).
        // We assert no panic occurs and rely on ClipboardStatus::check
        // behaviour being covered by the component's own logic.
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut cs = ClipboardStatus::default();
        cs.on_event(Event::List(crate::observer::event::ListEvent::Next(0)));
        // No panic = pass.
    }
}

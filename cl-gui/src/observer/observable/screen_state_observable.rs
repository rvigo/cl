use crate::component::ScreenState;
use crate::observer::event::{Event, ScreenStateEvent};
use crate::observer::observable::SyncObservable;
use tracing::debug;

impl SyncObservable for ScreenState {
    fn on_event(&mut self, event: Event) {
        if let Event::ScreenState(e) = event {
            match e {
                ScreenStateEvent::SetField(field) => {
                    debug!("ScreenState: setting current field to {:?}", field);
                    self.current_field = field;
                }
                ScreenStateEvent::KeyInput(_) => {
                    if self.has_changes {
                        debug!("ScreenState: changes already registered");
                    } else {
                        self.has_changes = true;
                        debug!("ScreenState: registered first change");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::event::ScreenStateEvent;
    use crate::state::state_event::FieldName;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn dummy_key() -> crossterm::event::KeyEvent {
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)
    }

    #[test]
    fn set_field_updates_current_field() {
        let mut state = ScreenState::new(FieldName::Alias);
        state.on_event(Event::ScreenState(ScreenStateEvent::SetField(
            FieldName::Command,
        )));
        assert_eq!(state.current_field, FieldName::Command);
    }

    #[test]
    fn key_input_marks_has_changes() {
        let mut state = ScreenState::new(FieldName::Alias);
        assert!(!state.has_changes);
        state.on_event(Event::ScreenState(ScreenStateEvent::KeyInput(dummy_key())));
        assert!(state.has_changes);
    }

    #[test]
    fn key_input_is_idempotent_once_changed() {
        let mut state = ScreenState::new(FieldName::Alias);
        state.on_event(Event::ScreenState(ScreenStateEvent::KeyInput(dummy_key())));
        state.on_event(Event::ScreenState(ScreenStateEvent::KeyInput(dummy_key())));
        assert!(state.has_changes);
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut state = ScreenState::new(FieldName::Alias);
        state.on_event(Event::List(crate::observer::event::ListEvent::Next(0)));
        assert_eq!(state.current_field, FieldName::Alias);
        assert!(!state.has_changes);
    }
}

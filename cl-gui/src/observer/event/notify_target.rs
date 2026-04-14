use crate::observer::event::Event;

/// Maps a component type to the `Event` variant that targets it.
///
/// Implement this on each component to explicitly declare which event payload
/// type it accepts and how that payload is wrapped into `Event`.
///
/// ```ignore
/// impl NotifyTarget for List {
///     type Payload = ListEvent;
///     fn wrap(payload: ListEvent) -> Event { Event::List(payload) }
/// }
/// ```
pub trait NotifyTarget: 'static {
    type Payload;
    fn wrap(payload: Self::Payload) -> Event;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{
        ClipboardStatus, EditableTextbox, List, Popup, ScreenState, Search, Tabs, TextBox,
    };
    use crate::observer::event::{
        ClipboardAction, EditableTextboxEvent, ListEvent, PopupEvent, ScreenStateEvent,
        SearchEvent, TabsEvent, TextBoxEvent,
    };
    use std::any::TypeId;

    #[test]
    fn list_wraps_to_event_list() {
        let event = List::wrap(ListEvent::Next(3));
        assert!(matches!(event, Event::List(ListEvent::Next(3))));
    }

    #[test]
    fn tabs_wraps_to_event_tabs() {
        let event = Tabs::wrap(TabsEvent::Next(1));
        assert!(matches!(event, Event::Tabs(TabsEvent::Next(1))));
    }

    #[test]
    fn textbox_wraps_to_event_textbox() {
        let event = TextBox::wrap(TextBoxEvent::UpdateContent("x".to_string()));
        assert!(matches!(
            event,
            Event::TextBox(TextBoxEvent::UpdateContent(_))
        ));
    }

    #[test]
    fn search_wraps_to_event_search() {
        let event = Search::wrap(SearchEvent::UpdateQuery("q".to_string()));
        assert!(matches!(event, Event::Search(SearchEvent::UpdateQuery(_))));
    }

    #[test]
    fn screen_state_wraps_to_event_screen_state() {
        use crate::state::state_event::FieldName;
        let event = ScreenState::wrap(ScreenStateEvent::SetField(FieldName::Command));
        assert!(matches!(
            event,
            Event::ScreenState(ScreenStateEvent::SetField(_))
        ));
    }

    #[test]
    fn clipboard_status_wraps_to_event_clipboard_status() {
        let event = ClipboardStatus::wrap(ClipboardAction::Copied);
        assert!(matches!(
            event,
            Event::ClipboardStatus(ClipboardAction::Copied)
        ));
    }

    #[test]
    fn popup_type_id_is_distinct_from_list_type_id() {
        assert_ne!(TypeId::of::<Popup>(), TypeId::of::<List>());
    }

    #[test]
    fn editable_textbox_wraps_to_event_editable_textbox() {
        use crate::state::state_event::FieldName;
        let event = EditableTextbox::wrap(EditableTextboxEvent::SetField(FieldName::Command));
        assert!(matches!(
            event,
            Event::EditableTextbox(EditableTextboxEvent::SetField(_))
        ));
    }
}

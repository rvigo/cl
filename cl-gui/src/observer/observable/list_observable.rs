use crate::component::List;
use crate::observer::event::{Event, ListEvent};
use crate::observer::observable::SyncObservable;

impl SyncObservable for List {
    fn on_event(&mut self, event: Event) {
        if let Event::List(e) = event {
            match e {
                ListEvent::Next(idx) => self.select(idx),
                ListEvent::Previous(idx) => self.select(idx),
                ListEvent::UpdateAll(items) => {
                    self.update_items(items);
                    self.state.select(Some(0));
                }
                ListEvent::UpdateListIdx(idx) => {
                    self.state.select(Some(idx));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::event::ListEvent;

    #[test]
    fn next_selects_given_index() {
        let mut list = List::new();
        list.update_items(vec!["a".into(), "b".into(), "c".into()]);
        list.on_event(Event::List(ListEvent::Next(2)));
        assert_eq!(list.state.selected(), Some(2));
    }

    #[test]
    fn previous_selects_given_index() {
        let mut list = List::new();
        list.update_items(vec!["a".into(), "b".into(), "c".into()]);
        list.on_event(Event::List(ListEvent::Previous(0)));
        assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn update_all_resets_to_first() {
        let mut list = List::new();
        list.on_event(Event::List(ListEvent::Next(5)));
        list.on_event(Event::List(ListEvent::UpdateAll(vec![
            "x".into(),
            "y".into(),
        ])));
        assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn update_list_idx_sets_selection() {
        let mut list = List::new();
        list.update_items(vec!["a".into(), "b".into(), "c".into()]);
        list.on_event(Event::List(ListEvent::UpdateListIdx(1)));
        assert_eq!(list.state.selected(), Some(1));
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut list = List::new();
        list.update_items(vec!["a".into()]);
        list.state.select(Some(0));
        // Sending a non-List event must not change state.
        list.on_event(Event::Tabs(crate::observer::event::TabsEvent::Next(99)));
        assert_eq!(list.state.selected(), Some(0));
    }
}

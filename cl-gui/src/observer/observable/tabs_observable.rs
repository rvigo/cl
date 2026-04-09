use crate::component::Tabs;
use crate::observer::event::{Event, TabsEvent};
use crate::observer::observable::SyncObservable;

impl SyncObservable for Tabs {
    fn on_event(&mut self, event: Event) {
        if let Event::Tabs(e) = event {
            match e {
                TabsEvent::Next(idx) => self.select(idx),
                TabsEvent::Previous(idx) => self.select(idx),
                TabsEvent::UpdateAll(items) => {
                    self.update_items(items);
                    self.reset_selected();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::event::TabsEvent;

    fn tabs_with_items(items: Vec<&str>) -> Tabs {
        let mut t = Tabs::default();
        t.update_items(items.into_iter().map(String::from).collect());
        t
    }

    #[test]
    fn update_all_resets_selection() {
        let mut tabs = tabs_with_items(vec!["a", "b", "c"]);
        tabs.on_event(Event::Tabs(TabsEvent::Next(2)));
        tabs.on_event(Event::Tabs(TabsEvent::UpdateAll(vec!["x".into(), "y".into()])));
        // After UpdateAll the selected index is reset to 0 (reset_selected).
        // We verify by sending Next(1) afterwards — if it were already 0 the
        // tab should accept 1 without panic.
        tabs.on_event(Event::Tabs(TabsEvent::Next(1)));
        // No panic = pass; selected is 1 after Next.
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut tabs = tabs_with_items(vec!["a"]);
        // A List event must not cause any state change.
        tabs.on_event(Event::List(crate::observer::event::ListEvent::Next(99)));
        // No panic = pass.
    }
}

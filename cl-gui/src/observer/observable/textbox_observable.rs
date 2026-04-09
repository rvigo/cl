use crate::component::TextBox;
use crate::observer::event::{Event, TextBoxEvent};
use crate::observer::observable::SyncObservable;
use crate::state::state_event::FieldName;
use tracing::debug;
use std::borrow::Cow;

impl SyncObservable for TextBox {
    fn on_event(&mut self, event: Event) {
        match event {
            Event::TextBox(e) => match e {
                TextBoxEvent::UpdateCommand(command) => {
                    let content = match self.name {
                        FieldName::Command => command.command.some_or_none(),
                        FieldName::Description => {
                            command.description.map(|desc| desc.to_string())
                        }
                        FieldName::Tags => command.tags.map(|vec| {
                            vec.iter()
                                .map(|cow| cow.as_ref())
                                .collect::<Vec<_>>()
                                .join(", ")
                        }),
                        FieldName::Namespace => command.namespace.some_or_none(),
                        FieldName::Alias => command.alias.some_or_none(),
                    };
                    self.update_content(content);
                }
                TextBoxEvent::UpdateContent(content) => {
                    debug!("updating textbox content");
                    self.update_content(Some(content));
                }
            },
            // The quick_search TextBox in MainScreenLayer is registered under
            // the Search TypeId so it can also receive search-query updates.
            Event::Search(crate::observer::event::SearchEvent::UpdateQuery(query)) => {
                debug!("updating quick_search display from search query");
                self.update_content(Some(query));
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Helper trait: converts Cow<str> to Option<String> (None when empty).
// Used by TextBox and EditableTextbox observables.
// ---------------------------------------------------------------------------

pub trait SomeOrNone {
    fn some_or_none(&self) -> Option<String>
    where
        Self: Sized;
}

impl SomeOrNone for Cow<'static, str> {
    fn some_or_none(&self) -> Option<String> {
        let s = self.to_string();
        if s.is_empty() { None } else { Some(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::event::{SearchEvent, TextBoxEvent};
    use std::borrow::Cow;
    #[test]
    fn update_content_sets_text() {
        let mut tb = TextBox::default();
        tb.on_event(Event::TextBox(TextBoxEvent::UpdateContent("hello".into())));
        assert_eq!(tb.content.as_deref(), Some("hello"));
    }

    #[test]
    fn search_update_query_sets_content_on_quick_search_textbox() {
        let mut tb = TextBox::default();
        tb.on_event(Event::Search(SearchEvent::UpdateQuery("foo".into())));
        assert_eq!(tb.content.as_deref(), Some("foo"));
    }

    #[test]
    fn some_or_none_returns_none_for_empty() {
        let cow: Cow<'static, str> = Cow::Borrowed("");
        assert!(cow.some_or_none().is_none());
    }

    #[test]
    fn some_or_none_returns_some_for_non_empty() {
        let cow: Cow<'static, str> = Cow::Borrowed("hello");
        assert_eq!(cow.some_or_none(), Some("hello".to_owned()));
    }

    #[test]
    fn wrong_event_variant_is_ignored() {
        let mut tb = TextBox {
            content: Some("original".into()),
            ..Default::default()
        };
        tb.on_event(Event::List(crate::observer::event::ListEvent::Next(0)));
        assert_eq!(tb.content.as_deref(), Some("original"));
    }
}

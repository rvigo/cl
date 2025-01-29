use crate::component::{TextBox, TextBoxName};
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use async_trait::async_trait;
use std::borrow::Cow;
use log::debug;

#[async_trait(?Send)]
impl Observable for TextBox {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::UpdateCommand(command) => {
                let content = match self.name {
                    TextBoxName::Command => command.command.some_or_none(),
                    TextBoxName::Description => command.description.map(|desc| desc.to_string()),
                    TextBoxName::Tags => command.tags.map(|vec| {
                        vec.iter()
                            .map(|cow| cow.as_ref())
                            .collect::<Vec<_>>()
                            .join(", ")
                    }),

                    TextBoxName::Namespace => command.namespace.some_or_none(),
                };

                self.update_content(content);
            }
            Event::UpdateContent(content) => {
                debug!("updating query @ main screen");
                self.update_content(Some(content));
            }
            _ => {}
        }
    }
}

trait SomeOrNone {
    fn some_or_none(&self) -> Option<String>
    where
        Self: Sized;
}

impl SomeOrNone for Cow<'static, str> {
    fn some_or_none(&self) -> Option<String>
    where
        Self: Sized,
    {
        let string = self.to_string();
        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }
}

use crate::component::{List, Tabs, TextBox};
use crate::observer::event::{Event, ListEvent, TabsEvent, TextboxEvent};
use crate::observer::publisher::Publisher;

pub enum PublisherContainer {
    TextBox(Publisher<TextBox>),
    List(Publisher<List>),
    Tabs(Publisher<Tabs>),
}

macro_rules! parse_event {
    ($event:expr, $type:tt) => {
        $event.as_any().downcast_ref::<$type>().map(|r| r.clone())
    };
}

impl PublisherContainer {
    pub async fn notify<E: Event>(&mut self, event: E) {
        match self {
            PublisherContainer::TextBox(p) => {
                if let Some(event) = parse_event!(event, TextboxEvent) {
                    p.notify(event.clone()).await;
                }
            }
            PublisherContainer::Tabs(p) => {
                if let Some(event) = parse_event!(event, TabsEvent) {
                    p.notify(event.clone()).await;
                }
            }
            PublisherContainer::List(p) => {
                if let Some(event) = parse_event!(event, ListEvent) {
                    p.notify(event.clone()).await;
                }
            }
        }
    }
}

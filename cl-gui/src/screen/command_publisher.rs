use crate::screen::listener::{ Event, Listener, Publisher};
use crate::widget::DisplayWidget;
use std::cell::RefCell;
use std::rc::Rc;

type Observers<'m> = Vec<Rc<RefCell<DisplayWidget<'m>>>>;

#[derive(Clone)]
pub struct CommandPublisher<'m> {
    observers: Observers<'m>,
}

impl CommandPublisher<'_> {
    pub fn new() -> Self {
        Self {
            observers: Observers::default(),
        }
    }
}

impl<'m> Publisher<DisplayWidget<'m>> for CommandPublisher<'m> {
    fn get_listeners<'s>(&self) -> &Vec<Rc<RefCell<DisplayWidget<'m>>>> {
        &self.observers
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<Rc<RefCell<DisplayWidget<'m>>>> {
        self.observers.as_mut()
    }

    fn register(&mut self, observer: Rc<RefCell<DisplayWidget<'m>>>) {
        self.get_listeners_mut().push(observer);
    }

    fn notify(&mut self, event: Event) {
        for observer in self.get_listeners() {
            observer.borrow_mut().update(event.to_owned());
        }
    }
}

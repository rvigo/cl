use crate::widget::{text_field::FieldType, DisplayWidget};
use cl_core::Command;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait Observer {
    fn update(&mut self, content: String);
}

impl<'d> Observer for DisplayWidget<'d> {
    fn update(&mut self, content: String) {
        self.content = content;
    }
}

#[derive(Clone)]
pub struct Subject<O> {
    observers: HashMap<FieldType, Rc<RefCell<O>>>,
}

impl<O> Default for Subject<O> {
    fn default() -> Self {
        Subject {
            observers: HashMap::new(),
        }
    }
}

impl<O> Subject<O>
where
    O: Observer,
{
    pub fn register(&mut self, field_type: FieldType, observer: Rc<RefCell<O>>) {
        self.observers.insert(field_type, observer);
    }

    pub fn notify(&mut self, command: &Command) {
        for (field_type, observer) in &mut self.observers {
            match field_type {
                FieldType::Command => {
                    let mut o = observer.borrow_mut();
                    o.update(command.command.clone());
                }
                FieldType::Tags => {
                    let mut o = observer.borrow_mut();
                    o.update(command.tags_as_string());
                }
                FieldType::Namespace => {
                    let mut o = observer.borrow_mut();
                    o.update(command.namespace.clone());
                }
                FieldType::Description => {
                    let mut o = observer.borrow_mut();
                    o.update(command.description());
                }
                _ => {}
            }
        }
    }
}

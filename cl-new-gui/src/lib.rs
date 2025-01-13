use std::cell::RefCell;
use std::rc::Rc;

mod component;
pub mod crossterm;
mod macros;
mod observer;
pub mod screen;
pub mod state;
pub mod termination;
pub mod ui;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedCell<T>(Rc<RefCell<T>>);

impl<T> SharedCell<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }
}

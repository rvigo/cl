use crate::observer::ObservableComponent;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct SharedComponent(pub Rc<RefCell<dyn ObservableComponent + 'static>>);

impl SharedComponent {
    pub fn new(component: impl ObservableComponent + 'static) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }

    pub fn borrow(&self) -> Ref<dyn ObservableComponent> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<dyn ObservableComponent> {
        self.0.borrow_mut()
    }
}

impl Clone for SharedComponent {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self))
    }
}

impl Deref for SharedComponent {
    type Target = Rc<RefCell<dyn ObservableComponent + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SharedComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

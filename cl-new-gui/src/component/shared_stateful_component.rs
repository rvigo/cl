use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;
use crate::observer::ObservableStatefulComponent;

pub struct SharedStatefulComponent(pub Rc<RefCell<dyn ObservableStatefulComponent + 'static>>);

impl SharedStatefulComponent {
    pub fn new(component: impl ObservableStatefulComponent + 'static) -> Self {
        Self(Rc::new(RefCell::new(component)))
    }

    pub fn borrow(&self) -> Ref<dyn ObservableStatefulComponent> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<dyn ObservableStatefulComponent> {
        self.0.borrow_mut()
    }
}

impl Deref for SharedStatefulComponent {
    type Target = Rc<RefCell<dyn ObservableStatefulComponent>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for SharedStatefulComponent {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self))
    }
}

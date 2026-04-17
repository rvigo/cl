use crate::observer::observable::Observable;
use std::any::Any;
use std::fmt::Debug;

pub mod event;
pub mod observable;
pub mod subscription;

/// Marker trait for structs that are Observables & Components
pub trait ObservableComponent: Observable + Debug + Any {
    /// Returns a reference to the component as a trait object
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the component as a trait object
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Default impl
impl<T> ObservableComponent for T
where
    T: Observable + Debug + Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

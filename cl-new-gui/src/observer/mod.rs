use crate::component::Renderable;
use crate::observer::observable::Observable;
use std::fmt::Debug;

pub mod event;
pub mod observable;
pub mod subscription;
mod clipboard_status;

/// Marker trait for structs that are Observables & Components
pub trait ObservableComponent: Observable + Renderable + Debug {}

// Default impl 
impl<T> ObservableComponent for T where T: Observable + Renderable + Debug {}

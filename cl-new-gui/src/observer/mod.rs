use crate::component::Component;
use crate::observer::observable::Observable;
use std::fmt::Debug;

pub mod event;
pub mod observable;
pub mod subscription;

pub trait ObservableComponent: Observable + Component + Debug {}

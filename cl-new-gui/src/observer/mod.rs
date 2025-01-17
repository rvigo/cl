use std::fmt::Debug;
use crate::component::{Component, StatefulComponent};
use crate::observer::observable::Observable;

pub mod event;
pub mod observable;
pub mod subscription;

pub trait ObservableComponent: Observable + Component + Debug {}

pub trait ObservableStatefulComponent: Observable + StatefulComponent {}
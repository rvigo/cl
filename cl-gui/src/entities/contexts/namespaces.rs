use cl_core::Namespace;
use itertools::Itertools;

use crate::entities::states::namespaces_state::NamespacesState;

pub const DEFAULT_NAMESPACE: &str = "All";

pub struct Namespaces {
    pub items: Vec<Namespace>,
    pub state: NamespacesState,
}

impl Namespaces {
    pub fn new(mut namespaces: Vec<Namespace>) -> Self {
        let mut namespaces = namespaces.include_default();
        namespaces.sort();

        Self {
            items: namespaces,
            state: NamespacesState::default(),
        }
    }

    pub fn update_namespaces(&mut self, namespaces: &[&Namespace]) {
        let mut namespaces = namespaces
            .iter()
            .copied()
            .map(|n| n.to_owned())
            .collect_vec()
            .include_default();
        namespaces.sort();

        self.items = namespaces
    }
}

pub trait NamespacesExt {
    fn include_default(&mut self) -> Vec<Namespace>;
}

impl NamespacesExt for Vec<Namespace> {
    fn include_default(&mut self) -> Vec<Namespace> {
        self.insert(0, DEFAULT_NAMESPACE.to_owned());
        self.to_owned()
    }
}

impl NamespacesExt for Vec<&Namespace> {
    fn include_default(&mut self) -> Vec<Namespace> {
        let mut namespaces = self.iter_mut().map(|n| n.to_owned()).collect::<Vec<_>>();
        namespaces.insert(0, DEFAULT_NAMESPACE.to_owned());
        namespaces
    }
}

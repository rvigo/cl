use std::collections::HashSet;

use crate::entities::states::namespaces_state::NamespacesState;
use cl_core::Namespace;
use itertools::Itertools;

pub const DEFAULT_NAMESPACE: &str = "All";

pub struct Namespaces {
    pub items: Vec<Namespace>,
    pub state: NamespacesState,
}

impl Namespaces {
    pub fn new(namespaces: Vec<Namespace>) -> Self {
        let sorted_namespaces = namespaces.fold_default();

        log::debug!("Namespaces: {:?}", sorted_namespaces);

        Self {
            items: sorted_namespaces,
            state: NamespacesState::default(),
        }
    }

    pub fn update_namespaces(&mut self, namespaces: &[&Namespace]) {
        let namespaces = namespaces
            .iter()
            .copied()
            .map(|n| n.to_owned())
            .collect_vec();

        self.items = namespaces.fold_default()
    }
}

pub trait NamespacesExt {
    fn include_default(&mut self) -> Vec<Namespace>;

    fn fold_default(&self) -> Vec<Namespace>;
}

impl NamespacesExt for Vec<Namespace> {
    fn include_default(&mut self) -> Vec<Namespace> {
        self.insert(0, DEFAULT_NAMESPACE.to_owned());
        self.to_owned()
    }

    fn fold_default(&self) -> Vec<Namespace> {
        let namespaces_set = self.iter().fold(
            Vec::<Namespace>::new()
                .include_default()
                .into_iter()
                .collect::<HashSet<String>>(),
            |mut set, ns| {
                set.insert(ns.to_owned());
                set
            },
        );

        let mut namespaces = namespaces_set.into_iter().collect::<Vec<Namespace>>();
        namespaces.sort();

        namespaces
    }
}

impl NamespacesExt for Vec<&Namespace> {
    fn include_default(&mut self) -> Vec<Namespace> {
        let mut namespaces = self.iter_mut().map(|n| n.to_owned()).collect::<Vec<_>>();
        namespaces.include_default();
        namespaces
    }

    fn fold_default(&self) -> Vec<Namespace> {
        self.iter()
            .map(|n| n.to_string())
            .collect::<Vec<Namespace>>()
            .fold_default()
    }
}

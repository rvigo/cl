use cl_core::Namespace;

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

    pub fn update_namespaces(&mut self, mut namespaces: Vec<Namespace>) {
        let mut namespaces = namespaces.include_default();
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

use crate::state::{NamespacesState, State};
use cl_core::Namespace;
use itertools::Itertools;
use std::collections::HashSet;

pub const DEFAULT_NAMESPACE: &str = "All";

pub struct NamespaceContext {
    pub items: Vec<Namespace>,
    pub state: NamespacesState,
}

impl NamespaceContext {
    pub fn new(namespaces: Vec<Namespace>) -> Self {
        let sorted_namespaces = namespaces.fold_default();

        log::debug!("Namespaces: {:?}", sorted_namespaces);

        Self {
            items: sorted_namespaces,
            state: NamespacesState::default(),
        }
    }

    pub fn current(&self) -> Namespace {
        self.items[self.state.selected()].to_owned()
    }

    pub fn update(&mut self, namespaces: &[&Namespace]) {
        let namespaces = namespaces.iter().map(|n| n.to_owned()).collect_vec();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Selectable;

    macro_rules! create_command {
        ($alias:expr, $command:expr, $namespace:expr, $description:expr, $tags:expr) => {{
            use cl_core::Command;
            Command {
                alias: $alias.to_owned(),
                namespace: $namespace.to_owned(),
                command: $command.to_owned(),
                description: $description,
                tags: $tags,
            }
        }};
    }

    #[test]
    fn should_go_to_next_namespace() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace1", None, None);
        let command3 = create_command!("alias3", "command", "namespace2", None, None);
        let n = vec![command1, command2, command3]
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect::<Vec<_>>();
        let mut namespaces = NamespaceContext::new(n);
        assert_eq!(namespaces.current(), DEFAULT_NAMESPACE);

        namespaces.next();
        assert_eq!(namespaces.current(), "namespace1");

        namespaces.next();
        assert_eq!(namespaces.current(), "namespace2");

        namespaces.next();
        assert_eq!(namespaces.current(), DEFAULT_NAMESPACE);
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace1", None, None);
        let command3 = create_command!("alias3", "command", "namespace2", None, None);
        let n = vec![command1, command2, command3]
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect::<Vec<_>>();
        let mut namespaces = NamespaceContext::new(n);

        assert_eq!(namespaces.current(), DEFAULT_NAMESPACE);

        namespaces.previous();
        assert_eq!(namespaces.current(), "namespace2");

        namespaces.previous();
        assert_eq!(namespaces.current(), "namespace1");

        namespaces.previous();
        assert_eq!(namespaces.current(), DEFAULT_NAMESPACE);
    }
}

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
    pub fn new(namespaces: Vec<impl Into<Namespace> + Clone>) -> Self {
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

impl<N> NamespacesExt for Vec<N>
where
    N: Into<Namespace> + Clone,
{
    fn include_default(&mut self) -> Vec<Namespace> {
        let mut namespaces = vec![DEFAULT_NAMESPACE.to_string()];
        namespaces.extend(self.iter().cloned().map(|ns| ns.into()));
        namespaces
    }

    fn fold_default(&self) -> Vec<Namespace> {
        let mut namespaces_set: HashSet<String> =
            self.iter().cloned().map(|ns| ns.into()).collect();

        namespaces_set.insert(DEFAULT_NAMESPACE.to_string());

        let mut namespaces = namespaces_set.into_iter().collect::<Vec<Namespace>>();
        namespaces.sort();
        namespaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Selectable;
    use std::borrow::Cow;

    macro_rules! create_command {
        ($alias:expr, $command:expr, $namespace:expr, $description:expr, $tags:expr) => {{
            use cl_core::Command;
            Command {
                alias: Cow::Borrowed($alias),
                namespace: Cow::Borrowed($namespace),
                command: Cow::Borrowed($command),
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
            .map(|c| c.namespace.to_string())
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
            .map(|c| c.namespace.to_string())
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

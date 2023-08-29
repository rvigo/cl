use super::Selectable;
use crate::gui::entities::states::{namespace_state::NamespaceState, State};
use std::collections::HashSet;

pub struct NamespacesContext {
    namespaces: Vec<String>,
    state: NamespaceState,
    current_namespace: String,
}

pub const DEFAULT_NAMESPACE: &str = "All";

impl NamespacesContext {
    pub fn new(namespaces: Vec<String>) -> NamespacesContext {
        let namespaces = Self::filter_namespaces(namespaces);
        let mut context = Self {
            namespaces: namespaces.to_owned(),
            state: NamespaceState::default(),
            current_namespace: namespaces[0].to_owned(),
        };

        context.state.select(0);

        context
    }

    pub fn namespaces(&self) -> &Vec<String> {
        &self.namespaces
    }

    pub fn current_namespace(&self) -> String {
        self.current_namespace.to_owned()
    }

    pub fn reset_context(&mut self) {
        self.select_namespace(0);
        let namespace = self.namespaces()[0].to_owned();
        self.set_current_namespace(namespace);
    }

    pub fn update_namespaces(&mut self, new_namespaces: Vec<String>) {
        let namespaces = Self::filter_namespaces(new_namespaces);
        self.namespaces = namespaces
    }

    pub fn get_selected_namespace_idx(&self) -> usize {
        self.state.selected()
    }

    fn select_namespace(&mut self, idx: usize) {
        self.state.select(idx)
    }

    fn set_current_namespace(&mut self, new_namespace: String) {
        self.current_namespace = new_namespace
    }

    fn filter_namespaces(namespaces: Vec<String>) -> Vec<String> {
        let mut namespaces = namespaces.iter().fold(
            vec![DEFAULT_NAMESPACE.to_owned()]
                .into_iter()
                .collect::<HashSet<String>>(),
            |mut set, ns| {
                set.insert(ns.to_owned());
                set
            },
        );
        let mut namespaces: Vec<String> = namespaces.drain().collect();
        namespaces.sort();

        namespaces
    }
}

impl Selectable for NamespacesContext {
    fn next(&mut self) {
        let current = self.get_selected_namespace_idx();
        let next = (current + 1) % self.namespaces.len();
        self.state.select(next);

        self.current_namespace = self
            .namespaces
            .get(next)
            .unwrap_or(&String::from(DEFAULT_NAMESPACE))
            .to_owned();
    }

    fn previous(&mut self) {
        let current = self.get_selected_namespace_idx();
        let previous = (current + self.namespaces.len() - 1) % self.namespaces.len();

        self.state.select(previous);
        self.current_namespace = self
            .namespaces
            .get(previous)
            .unwrap_or(&String::from(DEFAULT_NAMESPACE))
            .to_owned();
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn namespace_builder(n_of_namespaces: usize) -> Vec<String> {
        let mut namespaces = vec![];
        for i in 0..n_of_namespaces {
            namespaces.push(format!("namespace{}", (i + 1)))
        }

        namespaces
    }
    fn context_builder(n_of_commands: usize) -> NamespacesContext {
        let namespaces = namespace_builder(n_of_commands);
        NamespacesContext::new(namespaces)
    }

    #[test]
    fn should_filter_namespaces() {
        let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];
        let context = NamespacesContext::new(vec!["namespace1".to_string()]);
        assert_eq!(context.namespaces(), &expected);

        let namespaces = vec![
            "namespace1".to_string(),
            "namespace1".to_string(),
            "namespace1".to_string(),
        ];

        let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];

        let context = NamespacesContext::new(namespaces);
        assert_eq!(context.namespaces(), &expected);
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);

        context.previous();
        assert_eq!(context.state.selected(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.previous();
        assert_eq!(context.state.selected(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.previous();
        assert_eq!(context.state.selected(), 0);
        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);

        context.next();
        assert_eq!(context.state.selected(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.next();
        assert_eq!(context.state.selected(), 2);
        assert_eq!(context.state.selected(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.next();
        assert_eq!(context.state.selected(), 0);
        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);
    }
}

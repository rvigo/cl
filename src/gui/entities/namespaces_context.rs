use super::namespace_state::NamespaceState;
use std::collections::HashSet;

pub struct NamespacesContext {
    namespaces: Vec<String>,
    namespace_state: NamespaceState,
    current_namespace: String,
}

const DEFAULT_NAMESPACE: &str = "All";

impl NamespacesContext {
    pub fn new(namespaces: Vec<String>) -> NamespacesContext {
        let namespaces = Self::filter_namespaces(namespaces);
        let mut context = Self {
            namespaces: namespaces.clone(),
            namespace_state: NamespaceState::default(),
            current_namespace: namespaces[0].to_owned(),
        };

        context.namespace_state.select(0);

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
        self.namespace_state.selected()
    }

    pub fn next_namespace(&mut self) {
        let i = self.get_selected_namespace_idx();
        let i = if i >= self.namespaces.len() - 1 {
            0
        } else {
            i + 1
        };
        self.namespace_state.select(i);
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from(DEFAULT_NAMESPACE))
            .to_owned();
    }

    pub fn previous_namespace(&mut self) {
        let i = self.get_selected_namespace_idx();
        let i = if i == 0 {
            self.namespaces.len() - 1
        } else {
            i - 1
        };

        self.namespace_state.select(i);
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from(DEFAULT_NAMESPACE))
            .to_owned();
    }

    fn select_namespace(&mut self, idx: usize) {
        self.namespace_state.select(idx)
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

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected(), 0);
        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);

        context.next_namespace();
        assert_eq!(context.namespace_state.selected(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.next_namespace();
        assert_eq!(context.namespace_state.selected(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.next_namespace();
        assert_eq!(context.namespace_state.selected(), 0);
        assert_eq!(context.current_namespace, DEFAULT_NAMESPACE);
    }
}

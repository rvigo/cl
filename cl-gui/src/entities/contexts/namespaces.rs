use cl_core::commands::Namespace;

use super::Selectable;
use crate::entities::states::State;
use std::collections::HashSet;

pub const DEFAULT_NAMESPACE: &str = "All";

pub struct Namespaces {
    namespaces: Vec<Namespace>,
    selected_idx: usize,
}

impl Namespaces {
    pub fn new(namespaces: Vec<String>) -> Namespaces {
        Self {
            namespaces: Self::filter(namespaces),
            selected_idx: 0,
        }
    }

    pub fn namespaces(&self) -> &Vec<String> {
        &self.namespaces
    }

    pub fn current(&self) -> String {
        self.namespaces[self.selected_idx].to_owned()
    }

    pub fn reset_context(&mut self) {
        self.selected_idx = 0;
    }

    pub fn update(&mut self, new_namespaces: Vec<String>) {
        let namespaces = Self::filter(new_namespaces);
        self.namespaces = namespaces
    }

    pub fn get_selected_idx(&self) -> usize {
        self.selected_idx
    }

    fn filter(namespaces: Vec<String>) -> Vec<String> {
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

impl Selectable for Namespaces {
    fn next(&mut self) {
        let current = self.get_selected_idx();
        let next = (current + 1) % self.namespaces.len();

        self.selected_idx = next;
    }

    fn previous(&mut self) {
        let current = self.get_selected_idx();
        let previous = (current + self.namespaces.len() - 1) % self.namespaces.len();

        self.selected_idx = previous;
    }
}

impl State for Namespaces {
    type Output = usize;

    fn selected(&self) -> usize {
        self.selected_idx
    }

    fn select(&mut self, index: usize) {
        self.selected_idx = index;
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
    fn context_builder(n_of_commands: usize) -> Namespaces {
        let namespaces = namespace_builder(n_of_commands);
        Namespaces::new(namespaces)
    }

    #[test]
    fn should_filter_namespaces() {
        let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];
        let context = Namespaces::new(vec!["namespace1".to_string()]);
        assert_eq!(context.namespaces(), &expected);

        let namespaces = vec![
            "namespace1".to_string(),
            "namespace1".to_string(),
            "namespace1".to_string(),
        ];

        let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];

        let context = Namespaces::new(namespaces);
        assert_eq!(context.namespaces(), &expected);
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.namespaces[context.selected_idx], DEFAULT_NAMESPACE);

        context.previous();
        assert_eq!(context.selected_idx, 2);
        assert_eq!(context.namespaces[context.selected_idx], "namespace2");

        context.previous();
        assert_eq!(context.selected_idx, 1);
        assert_eq!(context.namespaces[context.selected_idx], "namespace1");

        context.previous();
        assert_eq!(context.selected_idx, 0);
        assert_eq!(context.namespaces[context.selected_idx], DEFAULT_NAMESPACE);
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.namespaces[context.selected_idx], DEFAULT_NAMESPACE);

        context.next();
        assert_eq!(context.selected_idx, 1);
        assert_eq!(context.namespaces[context.selected_idx], "namespace1");

        context.next();
        assert_eq!(context.selected_idx, 2);
        assert_eq!(context.selected_idx, 2);
        assert_eq!(context.namespaces[context.selected_idx], "namespace2");

        context.next();
        assert_eq!(context.selected_idx, 0);
        assert_eq!(context.namespaces[context.selected_idx], DEFAULT_NAMESPACE);
    }
}

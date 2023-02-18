use std::collections::HashSet;
use tui::widgets::ListState;

pub struct NamespacesContext {
    namespaces: Vec<String>,
    namespace_state: ListState,
    current_namespace: String,
}

impl NamespacesContext {
    pub fn new(namespaces: Vec<String>) -> NamespacesContext {
        let namespaces = Self::filter_namespaces(namespaces);
        let mut context = Self {
            namespaces: namespaces.clone(),
            namespace_state: ListState::default(),
            current_namespace: namespaces[0].to_owned(),
        };

        context.namespace_state.select(Some(0));

        context
    }

    pub fn namespaces(&self) -> &Vec<String> {
        &self.namespaces
    }

    pub fn current_namespace(&self) -> String {
        self.current_namespace.to_owned()
    }

    pub fn reset_namespaces_state(&mut self) {
        self.select_namespace(Some(0));
        self.set_current_namespace(self.namespaces()[0].to_owned());
    }

    pub fn update_namespaces(&mut self, new_namespaces: Vec<String>) {
        let namespaces = Self::filter_namespaces(new_namespaces);
        self.namespaces = namespaces
    }

    pub fn get_selected_namespace_idx(&self) -> usize {
        self.namespace_state.selected().unwrap_or(0)
    }

    pub fn next_namespace(&mut self) {
        let i = self.get_selected_namespace_idx();
        let i = if i >= self.namespaces.len() - 1 {
            0
        } else {
            i + 1
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from("All"))
            .to_owned();
    }

    pub fn previous_namespace(&mut self) {
        let i = self.get_selected_namespace_idx();
        let i = if i == 0 {
            self.namespaces.len() - 1
        } else {
            i - 1
        };

        self.namespace_state.select(Some(i));
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from("All"))
            .to_owned();
    }

    fn select_namespace(&mut self, idx: Option<usize>) {
        self.namespace_state.select(idx)
    }

    fn set_current_namespace(&mut self, new_namespace: String) {
        self.current_namespace = new_namespace
    }

    fn filter_namespaces(namespaces: Vec<String>) -> Vec<String> {
        let mut namespaces = namespaces.iter().fold(
            vec!["All".to_string()]
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
    use crate::command::Command;

    use super::*;
    fn commands_builder(n_of_commands: usize) -> Vec<Command> {
        let mut commands = vec![];
        for i in 0..n_of_commands {
            commands.push(Command {
                namespace: format!("namespace{}", (i + 1)),
                command: "command".to_string(),
                description: None,
                alias: "alias".to_string(),
                tags: None,
            })
        }

        commands
    }
    fn context_builder(n_of_commands: usize) -> NamespacesContext {
        let commands = commands_builder(n_of_commands);
        NamespacesContext::new(commands.iter().cloned().map(|c| c.namespace).collect())
    }

    #[test]
    fn should_filter_namespaces() {
        let context = context_builder(1);

        let expected = vec!["All".to_string(), "namespace1".to_string()];
        assert_eq!(context.namespaces, expected);
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.current_namespace, "All");

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.previous_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 0);
        assert_eq!(context.current_namespace, "All");
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut context = context_builder(2);

        assert_eq!(context.current_namespace, "All");

        context.next_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 1);
        assert_eq!(context.current_namespace, "namespace1");

        context.next_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 2);
        assert_eq!(context.current_namespace, "namespace2");

        context.next_namespace();
        assert_eq!(context.namespace_state.selected().unwrap(), 0);
        assert_eq!(context.current_namespace, "All");
    }
}

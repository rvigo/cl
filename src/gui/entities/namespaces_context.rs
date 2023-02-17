use itertools::Itertools;
use tui::widgets::ListState;
use crate::command::Command;

pub struct NamespacesContext {
    pub namespaces: Vec<String>,
    pub namespace_state: ListState,
    pub current_namespace: String,
}

impl NamespacesContext {
    pub fn new(mut namespaces: Vec<String>) -> NamespacesContext {
        namespaces.insert(0, "All".to_string());
        let mut context = Self {
            namespaces: namespaces.clone(),
            namespace_state: ListState::default(),
            current_namespace: namespaces[0].to_owned(),
        };

        context.namespace_state.select(Some(0));

        context
    }

    //TODO improve this function
    pub fn filter_namespaces(&mut self, filtered_commands: Vec<Command>) {
        let unique_namespaces = filtered_commands
            .iter()
            .map(|command| &command.namespace)
            .unique()
            .cloned()
            .collect();
        self.namespaces = unique_namespaces;
        self.namespaces.sort();
        self.namespaces.insert(0, String::from("All"));
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
}

#[cfg(test)]
mod test {
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

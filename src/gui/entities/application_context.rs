use super::{commands_context::CommandsContext, ui_context::UIContext};
use crate::{command::Command, gui::layouts::TerminalSize};
use itertools::Itertools;
use tui::widgets::ListState;

pub struct ApplicationContext<'a> {
    should_quit: bool,
    show_help: bool,

    pub namespaces: Vec<String>,
    pub namespace_state: ListState,
    pub current_namespace: String,

    pub commands_context: CommandsContext,
    pub ui_context: UIContext<'a>,
}

impl<'a> ApplicationContext<'a> {
    pub fn init(commands: Vec<Command>, terminal_size: TerminalSize) -> ApplicationContext<'a> {
        let mut state = ApplicationContext {
            should_quit: false,
            show_help: false,
            namespaces: Default::default(),
            namespace_state: ListState::default(),
            current_namespace: String::from("All"),
            commands_context: CommandsContext::new(commands),
            ui_context: UIContext::new(terminal_size),
        };

        state.load_namespaces();
        state.namespace_state.select(Some(0));
        state
            .ui_context
            .form_fields_context
            .focus_state
            .select(Some(0));
        state
            .ui_context
            .form_fields_context
            .select_command(state.commands_context.get_command_from_idx(0));
        state.ui_context.popup_context.choices_state.select(Some(0));

        state
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn set_show_help(&mut self, show_help: bool) {
        self.show_help = show_help
    }

    pub fn load_namespaces(&mut self) {
        self.namespace_state.select(Some(0));
        self.namespaces = self.commands_context.get_namespaces();
        self.current_namespace = self.namespaces[0].to_owned();
        self.filter_namespaces()
    }

    pub fn reload_state(&mut self) {
        self.load_namespaces();
        self.commands_context.select_command(0);
    }

    pub fn next_command(&mut self) {
        let query_string = self.ui_context.query_box.get_input();
        self.commands_context
            .next_command(&self.current_namespace, &query_string);
    }

    pub fn previous_command(&mut self) {
        let query_string = self.ui_context.query_box.get_input();
        self.commands_context
            .previous_command(&self.current_namespace, &query_string);
    }

    pub fn next_namespace(&mut self) {
        let i = self.namespace_state.selected().unwrap_or(0);
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
        self.commands_context.select_command(0);
    }

    pub fn previous_namespace(&mut self) {
        let i = self.namespace_state.selected().unwrap_or(0);
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
        self.commands_context.select_command(0);
    }

    pub fn filter_commands(&mut self) -> Vec<Command> {
        let query_string = self.ui_context.query_box.get_input();
        if let Ok(commands) = self
            .commands_context
            .commands
            .filter_commands(&self.current_namespace, &query_string)
        {
            commands
        } else {
            Vec::default()
        }
    }

    pub fn filter_namespaces(&mut self) {
        let filtered_commands = self.filter_commands();
        let unique_namespaces = filtered_commands
            .iter()
            .map(|command: &Command| &command.namespace)
            .unique()
            .cloned()
            .collect();
        self.namespaces = unique_namespaces;
        self.namespaces.sort();
        self.namespaces.insert(0, String::from("All"));
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
    fn state_builder(n_of_commands: usize) -> ApplicationContext<'static> {
        let commands = commands_builder(n_of_commands);
        ApplicationContext::init(commands, TerminalSize::Medium)
    }

    #[test]
    fn should_filter_namespaces() {
        let mut state = state_builder(1);

        state.filter_namespaces();

        let expected = vec!["All".to_string(), "namespace1".to_string()];
        assert_eq!(state.namespaces, expected);
    }

    #[test]
    fn should_load_namespaces() {
        let mut state = state_builder(1);

        state.load_namespaces();

        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(
            state.current_namespace,
            state.commands_context.commands.namespaces()[0].clone()
        );
        assert_eq!(
            state.namespaces,
            state.commands_context.commands.namespaces()
        );
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut state = state_builder(2);

        assert_eq!(state.current_namespace, "All");

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 2);
        assert_eq!(state.current_namespace, "namespace2");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 1);
        assert_eq!(state.current_namespace, "namespace1");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(state.current_namespace, "All");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut state = state_builder(2);

        assert_eq!(state.current_namespace, "All");

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 1);
        assert_eq!(state.current_namespace, "namespace1");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 2);
        assert_eq!(state.current_namespace, "namespace2");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(state.current_namespace, "All");
        assert_eq!(
            state
                .commands_context
                .commands_list_state()
                .selected()
                .unwrap(),
            0
        );
    }
}

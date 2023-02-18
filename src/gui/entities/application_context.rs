use super::{
    commands_context::CommandsContext, namespaces_context::NamespacesContext, ui_context::UIContext,
};
use crate::{command::Command, gui::layouts::TerminalSize};

pub struct ApplicationContext<'a> {
    should_quit: bool,
    show_help: bool,

    pub namespaces_context: NamespacesContext,

    pub commands_context: CommandsContext,
    pub ui_context: UIContext<'a>,
}

impl<'a> ApplicationContext<'a> {
    pub fn init(commands: Vec<Command>, terminal_size: TerminalSize) -> ApplicationContext<'a> {
        let mut state = ApplicationContext {
            should_quit: false,
            show_help: false,
            namespaces_context: NamespacesContext::new(
                commands.iter().map(|c| c.namespace.to_owned()).collect(),
            ),
            commands_context: CommandsContext::new(commands),
            ui_context: UIContext::new(terminal_size),
        };

        state.load_namespaces();
        state.ui_context.select_form(Some(0));
        state
            .ui_context
            .select_command(state.commands_context.get_command_from_idx(0));

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
        self.namespaces_context.namespace_state.select(Some(0));
        self.namespaces_context.namespaces = self.commands_context.get_namespaces();
        self.namespaces_context.current_namespace =
            self.namespaces_context.namespaces[0].to_owned();
        self.filter_namespaces()
    }

    pub fn reload_state(&mut self) {
        self.load_namespaces();
        self.commands_context.select_command(0);
    }

    pub fn next_command(&mut self) {
        let query_string = self.ui_context.get_querybox_input();
        self.commands_context
            .next_command(&self.namespaces_context.current_namespace, &query_string);
    }

    pub fn previous_command(&mut self) {
        let query_string = self.ui_context.get_querybox_input();
        self.commands_context
            .previous_command(&self.namespaces_context.current_namespace, &query_string);
    }

    pub fn next_namespace(&mut self) {
        self.namespaces_context.next_namespace();
        self.commands_context.select_command(0);
    }

    pub fn previous_namespace(&mut self) {
        self.namespaces_context.previous_namespace();
        self.commands_context.select_command(0);
    }

    pub fn filter_commands(&mut self) -> Vec<Command> {
        let query_string = self.ui_context.get_querybox_input();
        if let Ok(commands) = self
            .commands_context
            .commands
            .filter_commands(&self.namespaces_context.current_namespace, &query_string)
        {
            commands
        } else {
            Vec::default()
        }
    }

    pub fn filter_namespaces(&mut self) {
        let filtered_commands = self.filter_commands();
        self.namespaces_context.filter_namespaces(filtered_commands)
    }
}

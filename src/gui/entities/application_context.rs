use super::{
    commands_context::CommandsContext, namespaces_context::NamespacesContext, ui_context::UIContext,
};
use crate::{command::Command, gui::layouts::TerminalSize};

pub struct ApplicationContext<'a> {
    should_quit: bool,
    show_help: bool,
    namespaces_context: NamespacesContext,
    pub commands_context: CommandsContext,
    pub ui_context: UIContext<'a>,
}

impl<'a> ApplicationContext<'a> {
    pub fn init(commands: Vec<Command>, terminal_size: TerminalSize) -> ApplicationContext<'a> {
        let initial_command = Some(commands[0].to_owned());
        let namespaces = commands.iter().map(|c| c.namespace.to_owned()).collect();
        ApplicationContext {
            should_quit: false,
            show_help: false,
            namespaces_context: NamespacesContext::new(namespaces),
            commands_context: CommandsContext::new(commands),
            ui_context: UIContext::new(terminal_size, initial_command),
        }
    }

    pub fn namespaces_context(&self) -> &NamespacesContext {
        &self.namespaces_context
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

    pub fn reload_state(&mut self) {
        self.namespaces_context.reset_namespaces_state();
        self.filter_namespaces();
    }

    pub fn next_command(&mut self) {
        let query_string = self.ui_context.get_querybox_input();
        self.commands_context
            .next_command(&self.namespaces_context.current_namespace(), &query_string);
    }

    pub fn previous_command(&mut self) {
        let query_string = self.ui_context.get_querybox_input();
        self.commands_context
            .previous_command(&self.namespaces_context.current_namespace(), &query_string);
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
        let current_namespace = self.namespaces_context.current_namespace();
        self.commands_context
            .filter_commands(&current_namespace, &query_string)
    }

    pub fn filter_namespaces(&mut self) {
        let filtered_namespaces: Vec<String> = self
            .filter_commands()
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect();
        self.namespaces_context
            .update_namespaces(filtered_namespaces);
    }
}

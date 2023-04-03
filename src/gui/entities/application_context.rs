use super::{commands_context::CommandsContext, namespaces_context::NamespacesContext};
use crate::{
    command::Command,
    resources::{config::Options, file_service::FileService},
};
use anyhow::Result;
use tui::widgets::ListState;

pub struct ApplicationContext {
    namespaces_context: NamespacesContext,
    commands_context: CommandsContext,
    config_options: Options,
}

impl ApplicationContext {
    pub fn init(
        commands: Vec<Command>,
        file_service: FileService,
        config_options: Options,
    ) -> ApplicationContext {
        let namespaces = commands.iter().map(|c| c.namespace.to_owned()).collect();
        ApplicationContext {
            namespaces_context: NamespacesContext::new(namespaces),
            commands_context: CommandsContext::new(commands, file_service),
            config_options,
        }
    }

    // namespaces context
    pub fn namespaces_context(&self) -> &NamespacesContext {
        &self.namespaces_context
    }

    pub fn reload_namespaces_state(&mut self) {
        self.namespaces_context.reset_namespaces_state();
        self.commands_context.reset_command_idx();
        self.filter_namespaces();
    }

    pub fn next_namespace(&mut self) {
        self.namespaces_context.next_namespace();
        self.commands_context.reset_command_idx();
    }

    pub fn previous_namespace(&mut self) {
        self.namespaces_context.previous_namespace();
        self.commands_context.reset_command_idx();
    }

    // commands context
    pub fn next_command(&mut self, query_string: String) {
        self.commands_context
            .next_command(&self.namespaces_context.current_namespace(), &query_string);
    }

    pub fn previous_command(&mut self, query_string: String) {
        self.commands_context
            .previous_command(&self.namespaces_context.current_namespace(), &query_string);
    }

    pub fn add_command(&mut self, command: Command) -> Result<()> {
        self.commands_context.add_command(&command)
    }

    pub fn delete_selected_command(&mut self, command: &Command) -> Result<()> {
        self.commands_context.remove_command(command)
    }

    pub fn add_edited_command(
        &mut self,
        edited_command: Command,
        old_command: &Command,
    ) -> Result<()> {
        self.commands_context
            .add_edited_command(&edited_command, old_command)
    }

    /// Sets the current selected command to be executed at the end of the app execution and then tells the app to quit
    pub fn set_current_command_as_callback(&mut self, command: &Command) {
        self.commands_context
            .set_command_to_be_executed(Some(command.to_owned()));
    }

    /// Executes the callback command
    pub fn execute_callback_command(&self) -> Result<()> {
        self.commands_context
            .execute_command(self.config_options.get_default_quiet_mode())
    }

    pub fn get_commands_state(&self) -> ListState {
        self.commands_context.state()
    }

    pub fn get_selected_command_idx(&self) -> usize {
        self.commands_context.get_selected_command_idx()
    }

    // other
    pub fn should_highligh(&mut self) -> bool {
        self.config_options.get_highlight()
    }

    /// Filters the command list using the querybox input as query
    pub fn filter_commands(&mut self, query_string: String) -> Vec<Command> {
        let current_namespace = self.namespaces_context.current_namespace();
        self.commands_context
            .filter_commands(&current_namespace, &query_string)
    }

    /// Filters the namespaces based on a filtered command list
    pub fn filter_namespaces(&mut self) {
        let filtered_namespaces: Vec<String> = self
            .commands_context
            .filtered_commands()
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect();
        self.namespaces_context
            .update_namespaces(filtered_namespaces);
    }
}

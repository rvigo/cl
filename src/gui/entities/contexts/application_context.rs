use super::{
    commands_context::CommandsContext,
    namespaces_context::{NamespacesContext, DEFAULT_NAMESPACE},
    Selectable,
};
use crate::{
    command::Command,
    gui::entities::clipboard::Clipboard,
    resources::{
        commands_file_service::CommandsFileService, config::Options,
        logger::interceptor::ErrorInterceptor,
    },
};
use anyhow::Result;
use tui::widgets::ListState;

pub struct ApplicationContext {
    namespaces_context: NamespacesContext,
    commands_context: CommandsContext,
    config_options: Options,
    clipboard: Option<Clipboard>,
}

impl ApplicationContext {
    pub fn init(
        commands: Vec<Command>,
        commands_file_service: CommandsFileService,
        config_options: Options,
    ) -> ApplicationContext {
        let namespaces = commands.iter().map(|c| c.namespace.to_owned()).collect();
        let clipboard = Clipboard::new().log_error().ok();

        ApplicationContext {
            namespaces_context: NamespacesContext::new(namespaces),
            commands_context: CommandsContext::new(commands, commands_file_service),
            config_options,
            clipboard,
        }
    }

    pub fn copy_text_to_clipboard<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>,
    {
        if let Some(ref mut clipboard) = &mut self.clipboard {
            clipboard.set_content(content.into())?;
        }
        Ok(())
    }

    // namespaces context
    pub fn namespaces_context(&self) -> &NamespacesContext {
        &self.namespaces_context
    }

    pub fn reload_contexts(&mut self) {
        self.reload_commands_context();
        self.reload_namespaces_context();
    }

    /// Reloads the command context, filtering all commands and reseting the select command idx
    fn reload_commands_context(&mut self) {
        self.commands_context.filter_commands(DEFAULT_NAMESPACE, "");
        self.commands_context.reset_selected_command_idx();
    }

    /// Reloads the namespace context, updating the availble namespaces
    fn reload_namespaces_context(&mut self) {
        self.filter_namespaces();
        self.namespaces_context.reset_context();
    }

    pub fn next_namespace(&mut self) {
        self.namespaces_context.next();
        self.commands_context.reset_selected_command_idx();
    }

    pub fn previous_namespace(&mut self) {
        self.namespaces_context.previous();
        self.commands_context.reset_selected_command_idx();
    }

    // commands context
    pub fn next_command(&mut self) {
        self.commands_context.next();
    }

    pub fn previous_command(&mut self) {
        self.commands_context.previous();
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
            .execute_command(self.config_options.get_quiet_mode())
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
            .filter_commands(&current_namespace, &query_string);
        self.commands_context.filtered_commands()
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

use super::{commands_context::CommandsContext, namespaces::DEFAULT_NAMESPACE};
use crate::entity::clipboard::Clipboard;
use anyhow::Result;
use cl_core::{
    command::Command, commands::Commands, preferences::Preferences,
    resource::commands_file_handler::CommandsFileHandler, CommandVec,
};
use tui::widgets::ListState;

pub struct ApplicationContext {
    pub commands_context: CommandsContext,
    preferences: Preferences,
    clipboard: Option<Clipboard>,
}

impl ApplicationContext {
    pub fn init(
        commands: Commands,
        commands_file_handler: CommandsFileHandler,
        preferences: Preferences,
    ) -> ApplicationContext {
        let clipboard = Clipboard::new().ok();

        ApplicationContext {
            commands_context: CommandsContext::new(commands, commands_file_handler),
            preferences,
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

    /// Reloads the command context, filtering all commands and reseting the select command idx
    pub fn reload(&mut self) {
        self.commands_context.filter_commands(DEFAULT_NAMESPACE, "");
        self.commands_context.reset_selected_command_idx();
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
            .execute_command(self.preferences.quiet_mode())
    }

    pub fn get_commands_state(&self) -> ListState {
        self.commands_context.state()
    }

    // other
    pub fn should_highlight(&mut self) -> bool {
        self.preferences.highlight()
    }

    /// Filters the command list using the querybox input as query
    pub fn filter_commands(&mut self, query_string: &str) -> CommandVec {
        let current_namespace = self.commands_context.current_namespace();
        self.commands_context
            .filter_commands(&current_namespace, query_string)
    }
}

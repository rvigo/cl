use std::{borrow::Borrow, path::PathBuf};

use super::{
    namespace_context::{NamespaceContext, DEFAULT_NAMESPACE},
    CommandsContext, FieldMap,
};
use crate::Clipboard;
use anyhow::Result;
use cl_core::{Command, CommandVec, Commands, Preferences};

pub struct Application<'app> {
    pub commands: CommandsContext<'app>,
    pub namespaces: NamespaceContext,
    preferences: Preferences,
    clipboard: Option<Clipboard>,
}

impl<'app> Application<'app> {
    pub fn init(
        commands: Commands<'app>,
        command_file_path: PathBuf,
        preferences: Preferences,
    ) -> Self {
        let clipboard = Clipboard::new().ok();
        let command_list = commands.as_list();
        let namespaces = command_list
            .iter()
            .map(|c| c.namespace.borrow())
            .collect::<Vec<&str>>();

        Application {
            commands: CommandsContext::new(commands, command_file_path),
            namespaces: NamespaceContext::new(namespaces),
            preferences,
            clipboard,
        }
    }

    pub fn copy_to_clipboard<T>(&mut self, content: T) -> Result<()>
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
        self.commands.filter_commands(DEFAULT_NAMESPACE, "");
        self.commands.reset_selected_command_idx();
    }

    // other
    pub fn should_highlight(&self) -> bool {
        self.preferences.highlight()
    }

    /// Filters the command list using the querybox input as query
    pub fn filter_commands(&mut self, query_string: &str) -> CommandVec<'app> {
        let current_namespace = self.namespaces.current();
        self.commands
            .filter_commands(&current_namespace, query_string)
    }
}

// Callback
impl Application<'_> {
    /// Executes the callback command
    pub fn execute_callback(&self) -> Result<()> {
        self.commands.execute(self.preferences.quiet_mode())
    }
}

impl<'app> Application<'app> {
    pub fn add_command(&mut self, command_value_map: FieldMap) -> Result<()> {
        let commands = self.commands.add(&command_value_map.into())?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }

    pub fn edit_command(
        &mut self,
        edited_command: FieldMap,
        current_command: &Command<'app>,
    ) -> Result<()> {
        let edited_command: &Command<'app> = &edited_command.into();
        let commands = self.commands.edit(edited_command, current_command)?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }

    pub fn remove_command(&mut self, command: &Command<'app>) -> Result<()> {
        let commands = self.commands.remove(command)?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }
}

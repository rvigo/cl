use super::{
    namespace_context::{NamespaceContext, DEFAULT_NAMESPACE},
    CommandsContext, FieldMap,
};
use crate::Clipboard;
use anyhow::Result;
use cl_core::{resource::FileService, Command, CommandVec, Commands, Preferences};

pub struct Application {
    pub commands: CommandsContext,
    pub namespaces: NamespaceContext,
    preferences: Preferences,
    clipboard: Option<Clipboard>,
}

impl Application {
    pub fn init(
        commands: Commands,
        file_service: FileService,
        preferences: Preferences,
    ) -> Application {
        let clipboard = Clipboard::new().ok();
        let namespaces = commands
            .to_list()
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect::<Vec<_>>();

        Application {
            commands: CommandsContext::new(commands, file_service),
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
    pub fn should_highlight(&mut self) -> bool {
        self.preferences.highlight()
    }

    /// Filters the command list using the querybox input as query
    pub fn filter_commands(&mut self, query_string: &str) -> CommandVec {
        let current_namespace = self.namespaces.current();
        self.commands
            .filter_commands(&current_namespace, query_string)
    }
}

// Callback
impl Application {
    /// Executes the callback command
    pub fn execute_callback(&self) -> Result<()> {
        self.commands.execute(self.preferences.quiet_mode())
    }
}

impl Application {
    pub fn add_command(&mut self, command_value_map: FieldMap) -> Result<()> {
        let commands = self.commands.add(&command_value_map.into())?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }

    pub fn edit_command(
        &mut self,
        edited_command: FieldMap,
        current_command: &Command,
    ) -> Result<()> {
        let commands = self
            .commands
            .edit(&edited_command.into(), current_command)?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }

    pub fn remove_command(&mut self, command: &Command) -> Result<()> {
        let commands = self.commands.remove(command)?;
        self.namespaces.update(&commands.keys().collect::<Vec<_>>());
        Ok(())
    }
}

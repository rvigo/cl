use std::{thread, time::Duration};

use crate::{command::Command, commands::Commands, resources::file_service};
use anyhow::{bail, Result};
use tui::widgets::ListState;
pub struct CommandsContext {
    pub commands: Commands,
    commands_list_state: ListState,
    to_be_executed: Option<Command>,
}

impl CommandsContext {
    pub fn new(commands: Vec<Command>) -> Self {
        let mut context = Self {
            commands: Commands::init(commands),
            commands_list_state: ListState::default(),
            to_be_executed: None,
        };
        context.commands_list_state.select(Some(0));

        context
    }

    pub fn commands_list_state(&self) -> ListState {
        self.commands_list_state.to_owned()
    }

    pub fn command_to_be_executed(&self) -> Option<Command> {
        self.to_be_executed.to_owned()
    }

    pub fn set_command_to_be_executed(&mut self, command: Option<Command>) {
        self.to_be_executed = command
    }

    pub fn select_command(&mut self, idx: usize) {
        self.commands_list_state.select(Some(idx))
    }

    pub fn get_selected_command_idx(&self) -> usize {
        self.commands_list_state.selected().unwrap_or(0)
    }

    pub fn get_namespaces(&self) -> Vec<String> {
        self.commands.namespaces()
    }

    pub fn get_command_from_idx(&mut self, idx: usize) -> Option<Command> {
        self.commands
            .get_command_item_ref(idx)
            .map(|c| c.to_owned())
    }

    pub fn filter_commands(&mut self, current_namespace: &str, query_string: &str) -> Vec<Command> {
        if let Ok(commands) = self
            .commands
            .filter_commands(current_namespace, query_string)
        {
            commands
        } else {
            Vec::default()
        }
    }

    pub fn next_command(&mut self, current_namespace: &str, query_string: &str) {
        let mut i = self.get_selected_command_idx();
        let filtered_commands = self.filter_commands(current_namespace, query_string);
        if !filtered_commands.is_empty() {
            i = if i >= filtered_commands.len() - 1 {
                0
            } else {
                i + 1
            };
        }
        self.select_command(i);
    }

    pub fn previous_command(&mut self, current_namespace: &str, query_string: &str) {
        let mut i = self.get_selected_command_idx();
        let filtered_commands = self.filter_commands(current_namespace, query_string);
        if !filtered_commands.is_empty() {
            i = if i == 0 {
                filtered_commands.len() - 1
            } else {
                i - 1
            };
        };

        self.select_command(i);
    }

    pub fn add_command(&mut self, new_command: &Command) -> Result<()> {
        new_command.validate()?;
        if let Ok(commands) = self.commands.add_command(new_command) {
            file_service::write_to_command_file(&commands)
        } else {
            bail!("Cannot save the new command")
        }
    }

    pub fn add_edited_command(
        &mut self,
        edited_command: &Command,
        current_command: &Command,
    ) -> Result<()> {
        edited_command.validate()?;
        if let Ok(commands) = self
            .commands
            .add_edited_command(edited_command, current_command)
        {
            file_service::write_to_command_file(&commands)
        } else {
            bail!("Cannot save the edited command")
        }
    }

    pub fn execute_command(&self) -> Result<()> {
        if let Some(command) = &self.command_to_be_executed() {
            if command.has_named_parameter() {
                eprintln!(
                    "Warning: This command appears to contains one or more named parameters placeholders. \
                    It may not run correctly using the interface.\n\
                If you want to use these parameters, please use the CLI option (cl exec --help)"
                );

                let seconds_to_sleep = Duration::from_secs(3);
                thread::sleep(seconds_to_sleep);

                eprintln!();
            }

            self.commands.exec_command(command, false, false)?;
        }

        Ok(())
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
    fn commands_context_builder(n_of_commands: usize) -> CommandsContext {
        let commands = commands_builder(n_of_commands);
        CommandsContext::new(commands)
    }
    #[test]
    fn should_go_to_next_command() {
        let mut context = commands_context_builder(3);
        let current_namespace = "All";
        let query_string = "";

        assert_eq!(context.commands_list_state.selected(), Some(0));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.commands_list_state.selected(), Some(1));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.commands_list_state.selected(), Some(2));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.commands_list_state.selected(), Some(0));
    }
    #[test]
    fn should_go_to_previous_command() {
        let mut context = commands_context_builder(3);
        let current_namespace = "All";
        let query_string = "";

        assert_eq!(context.commands_list_state.selected(), Some(0));

        context.previous_command(current_namespace, query_string);
        assert_eq!(context.commands_list_state.selected(), Some(2));

        context.previous_command(current_namespace, query_string);
        assert_eq!(context.commands_list_state.selected(), Some(1));
    }
}

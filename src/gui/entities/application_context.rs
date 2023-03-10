use super::{
    commands_context::CommandsContext, namespaces_context::NamespacesContext, ui_context::UIContext,
};
use crate::{
    command::Command,
    gui::{
        layouts::{TerminalSize, ViewMode},
        widgets::{
            fields::Fields,
            popup::{Answer, ChoicesState, Popup},
            query_box::QueryBox,
        },
    },
    resources::{config::Options, file_service::FileService},
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use tui::widgets::ListState;

pub struct ApplicationContext<'a> {
    should_quit: bool,
    show_help: bool,
    namespaces_context: NamespacesContext,
    commands_context: CommandsContext,
    ui_context: UIContext<'a>,
    config_options: Options,
}

impl<'a> ApplicationContext<'a> {
    pub fn init(
        commands: Vec<Command>,
        terminal_size: TerminalSize,
        file_service: FileService,
        config_options: Options,
    ) -> ApplicationContext<'a> {
        let initial_command = Some(commands[0].to_owned());
        let namespaces = commands.iter().map(|c| c.namespace.to_owned()).collect();
        ApplicationContext {
            should_quit: false,
            show_help: false,
            namespaces_context: NamespacesContext::new(namespaces),
            commands_context: CommandsContext::new(commands, file_service),
            ui_context: UIContext::new(terminal_size, initial_command),
            config_options,
        }
    }

    // namespaces context
    pub fn namespaces_context(&self) -> &NamespacesContext {
        &self.namespaces_context
    }

    pub fn reload_namespaces_state(&mut self) {
        self.namespaces_context.reset_namespaces_state();
        self.filter_namespaces();
    }

    pub fn next_namespace(&mut self) {
        self.namespaces_context.next_namespace();
        self.commands_context.select_command_idx(0);
    }

    pub fn previous_namespace(&mut self) {
        self.namespaces_context.previous_namespace();
        self.commands_context.select_command_idx(0);
    }

    // UI context
    pub fn select_command(&mut self, command: Option<Command>) {
        self.ui_context.select_command(command)
    }

    ////forms
    pub fn next_form_field(&mut self) {
        self.ui_context.next_form_field()
    }

    pub fn previous_form_field(&mut self) {
        self.ui_context.previous_form_field()
    }

    pub fn get_form_fields(&self) -> &Fields {
        self.ui_context.get_form_fields()
    }

    pub fn handle_form_input(&mut self, input: KeyEvent) {
        if let Some(selected_field) = self.ui_context.get_selected_form_field_mut() {
            selected_field.on_input(input)
        }
    }

    fn build_form_fields(&mut self) {
        self.ui_context.build_form_fields()
    }

    //// querybox
    pub fn toogle_querybox_focus(&mut self) {
        self.ui_context.toogle_querybox_focus()
    }

    pub fn query_box(&self) -> QueryBox {
        self.ui_context.querybox()
    }

    pub fn querybox_focus(&self) -> bool {
        self.ui_context.querybox_focus()
    }

    pub fn handle_querybox_input(&mut self, key_event: KeyEvent) {
        self.ui_context.handle_querybox_input(key_event)
    }

    //// viewmode
    pub fn view_mode(&self) -> &ViewMode {
        self.ui_context.view_mode()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.ui_context.set_view_mode(view_mode)
    }

    //// popup
    pub fn next_popup_choice(&mut self) {
        if let Some(popup) = self.ui_context.popup() {
            self.ui_context.next_choice(popup.choices())
        }
    }

    pub fn previous_popup_choice(&mut self) {
        if let Some(popup) = self.ui_context.popup() {
            self.ui_context.previous_choice(popup.choices())
        }
    }

    pub fn get_popup_answer(&self) -> Option<Answer> {
        self.ui_context.get_popup_answer()
    }

    pub fn get_choices_state_mut(&mut self) -> &mut ChoicesState {
        self.ui_context.get_choices_state_mut()
    }

    pub fn get_selected_choice(&self) -> Option<usize> {
        self.ui_context.get_selected_choice()
    }

    pub fn clear_popup_context(&mut self) {
        self.ui_context.clear_popup_context()
    }

    pub fn popup(&self) -> Option<Popup<'a>> {
        self.ui_context.popup()
    }

    pub fn show_delete_popup(&mut self) {
        if let Some(selected_command) = self.ui_context.get_selected_command() {
            if !selected_command.is_incomplete() {
                let popup = Popup::from_warning("Are you sure you want to delete the command?");
                self.ui_context.set_popup(Some(popup));
            }
        }
    }

    pub fn handle_warning_popup(&mut self) {
        if let Some(popup) = self.popup() {
            if let Some(selected_choice_idx) = self.get_selected_choice() {
                if let Some(answer) = popup.choices().get(selected_choice_idx) {
                    match answer {
                        Answer::Ok => {
                            if let Some(command) = self.ui_context.get_selected_command() {
                                if self.commands_context.remove_command(command).is_ok() {
                                    self.clear_popup_context();

                                    self.reload_namespaces_state();
                                }
                            }
                        }

                        Answer::Cancel => {
                            self.clear_popup_context();
                        }
                    }
                }
            }
        }
    }

    //// terminal size
    pub fn terminal_size(&self) -> TerminalSize {
        self.ui_context.terminal_size()
    }

    pub fn update_terminal_size(&mut self, terminal_size: TerminalSize) {
        self.ui_context.update_terminal_size(terminal_size)
    }

    // commands context
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

    pub fn add_command(&mut self) {
        let command = self.ui_context.build_new_command();
        match self.commands_context.add_command(&command) {
            Ok(()) => self.enter_main_mode(),
            Err(error) => {
                let popup = Popup::from_error(error.to_string());
                self.ui_context.set_popup(Some(popup));
            }
        }
    }

    pub fn add_edited_command(&mut self) {
        let edited_command = self.ui_context.edit_command();
        let current_command = match self.ui_context.get_selected_command() {
            Some(command) => command,
            None => {
                let popup = Popup::from_error("No selected command to edit");
                self.ui_context.set_popup(Some(popup));
                return;
            }
        };

        if let Ok(()) = self
            .commands_context
            .add_edited_command(&edited_command, current_command)
        {
            self.enter_main_mode()
        } else {
            let popup = Popup::from_error("Failed to add the edited command");
            self.ui_context.set_popup(Some(popup));
        }
    }

    /// Sets the current selected command to be executed at the end of the app execution and then tells the app to quit
    pub fn set_callback_command(&mut self) {
        if let Some(selected_command) = self.ui_context.get_selected_command() {
            if !selected_command.is_incomplete() {
                self.commands_context
                    .set_command_to_be_executed(Some(selected_command.to_owned()));
                self.quit()
            }
        }
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
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn should_highligh(&mut self) -> bool {
        match self.config_options.get_highlight() {
            Ok(Some(value)) => value,
            _ => true,
        }
    }

    /// Tells the app to quit its execution
    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn set_show_help(&mut self, show_help: bool) {
        self.show_help = show_help
    }

    /// Filters the command list using the querybox input as query
    pub fn filter_commands(&mut self) -> Vec<Command> {
        let query_string = self.ui_context.get_querybox_input();
        let current_namespace = self.namespaces_context.current_namespace();
        self.commands_context
            .filter_commands(&current_namespace, &query_string)
    }

    /// Filters the namespaces based on a filtered command list
    pub fn filter_namespaces(&mut self) {
        let filtered_namespaces: Vec<String> = self
            .filter_commands()
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect();
        self.namespaces_context
            .update_namespaces(filtered_namespaces);
    }

    /// Changes the app main state to load the main screen in the next render tick
    pub fn enter_main_mode(&mut self) {
        self.reload_namespaces_state();
        self.ui_context.enter_main_mode();
    }

    /// Changes the app main state to load the edit screen in the next render tick
    pub fn enter_edit_mode(&mut self) {
        if self.ui_context.get_selected_command().is_some() {
            self.build_form_fields();
            self.ui_context.set_selected_command_input();
            self.ui_context.set_view_mode(ViewMode::Edit);
        }
    }

    /// Changes the app main state to load the insert screen in the next render tick
    pub fn enter_insert_mode(&mut self) {
        self.build_form_fields();
        self.set_view_mode(ViewMode::Insert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::CommandBuilder;
    use anyhow::Result;
    use std::env::temp_dir;

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

    fn application_context_builder(n_of_commands: usize) -> ApplicationContext<'static> {
        let commands = commands_builder(n_of_commands);
        ApplicationContext::init(
            commands,
            TerminalSize::Medium,
            FileService::new(temp_dir().join("commands.toml")),
            Options::default(),
        )
    }

    #[test]
    fn should_add_a_new_command() -> Result<()> {
        let mut context = application_context_builder(3);
        let expected_namespaces = vec![
            String::from("All"),
            String::from("namespace1"),
            String::from("namespace2"),
            String::from("namespace3"),
        ];

        assert_eq!(
            context.namespaces_context.namespaces(),
            &expected_namespaces
        );

        let mut builder = CommandBuilder::default();
        builder
            .alias("new_alias")
            .command("new_command")
            .namespace("new_namespace");

        let new_command = builder.build();

        context.ui_context.select_command(Some(new_command.clone()));

        assert!(context.ui_context.get_selected_command().is_some());
        assert_eq!(
            context.ui_context.get_selected_command().unwrap(),
            &new_command
        );

        context.ui_context.build_form_fields();
        context.ui_context.set_selected_command_input();

        context.add_command();

        let namespaces = vec![
            String::from("All"),
            String::from("namespace1"),
            String::from("namespace2"),
            String::from("namespace3"),
            new_command.namespace,
        ];

        assert!(context.ui_context.popup().is_none());
        assert_eq!(context.namespaces_context.namespaces(), &namespaces);

        Ok(())
    }
}

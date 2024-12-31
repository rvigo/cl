use super::{fields::Fields, PopupContext};
use crate::{state::ClipboardState, terminal::TerminalSize, widget::statusbar::QueryBox, ViewMode};
use cl_core::Command;

pub struct UI<'ui> {
    pub fields: Fields<'ui>,
    selected_command: Option<Command<'ui>>,
    pub popup: PopupContext,
    pub querybox: QueryBox<'ui>,
    pub clipboard_state: ClipboardState,
    view_mode: ViewMode,
}

impl<'ui> UI<'ui> {
    pub fn new(size: TerminalSize) -> UI<'ui> {
        UI {
            fields: Fields::new(&size),
            selected_command: None,
            popup: PopupContext::new(),
            querybox: QueryBox::default(),
            clipboard_state: ClipboardState::default(),
            view_mode: ViewMode::Main,
        }
    }
}

impl<'ui> UI<'ui> {
    pub fn set_selected_command_input(&mut self) {
        if let Some(command) = self.selected_command.as_ref() {
            self.fields.popuplate(command);
        }
    }

    pub fn selected_command(&self) -> Option<&Command<'ui>> {
        self.selected_command.as_ref()
    }

    pub fn select_command(&mut self, command: Option<&Command<'ui>>) {
        self.selected_command = command.map(ToOwned::to_owned)
    }

    ///
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode.to_owned()
    }

    pub fn set_view(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode;
        self.fields.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_clear_input_when_enter_insert_screen() {
        let mut ui = UI::new(TerminalSize::Medium);
        let command = Command::default();

        // enters edit mode
        ui.select_command(Some(&command));
        ui.fields.reset();
        ui.fields.clear_inputs();
        ui.set_selected_command_input();

        // enters insert mode
        ui.fields.reset();
        ui.fields.clear_inputs();

        let fields = ui.fields.inner();
        assert!(fields.iter().all(|c| c.text().is_empty()));
    }
}

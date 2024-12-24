use super::Selectable;
use crate::{
	event::{DialogType, PopupCallbackAction},
	screen::dialog_factory::{
		CommandDeletionConfirmationDialog, EditedScreenExitDialog, GenericErrorDialog, HelpPopup,
	},
	widget::popup::{Choice, Popup},
	State,
};

#[derive(Default, Clone)]
pub struct PopupContext {
	selected_choice_idx: usize,
	show_popup: bool,
	pub callback: PopupCallbackAction,
	pub dialog_type: Option<DialogType>,
}

impl PopupContext {
	pub fn new() -> PopupContext {
		Self {
			selected_choice_idx: 0,
			show_popup: false,
			callback: PopupCallbackAction::None,
			dialog_type: None,
		}
	}

	pub fn active_popup(&self) -> Option<Popup> {
		self.factory()
	}

	pub fn selected_choice(&self) -> Option<Choice> {
		if let Some(pop) = &self.active_popup() {
			if pop.choices.is_empty() {
				return None;
			}
			Some(pop.choices[self.selected_choice_idx].to_owned())
		} else {
			None
		}
	}

	pub fn selected_choice_idx(&self) -> usize {
		self.selected_choice_idx
	}

	pub fn clear_choices(&mut self) {
		self.selected_choice_idx = 0;
	}

	pub fn set_dialog_type(&mut self, r#type: DialogType) {
		self.dialog_type = Some(r#type);
	}

	pub fn show_popup(&self) -> bool {
		self.show_popup
	}

	pub fn set_show_popup(&mut self, show: bool) {
		self.show_popup = show
	}

	pub fn deactivate_popup(&mut self) {
		self.callback = PopupCallbackAction::None;
		self.dialog_type = None;
	}

	fn factory(&self) -> Option<Popup> {
		if let Some(dialog) = &self.dialog_type {
			let pop = match dialog {
				DialogType::CommandDeletionConfimation => CommandDeletionConfirmationDialog::create(),
				DialogType::EditedScreenExit => EditedScreenExitDialog::create(),
				DialogType::GenericError(message) => GenericErrorDialog::create(message),
				DialogType::HelpPopup(view_mode) => HelpPopup::create(view_mode),
			};

			Some(pop)
		} else {
			None
		}
	}
}

impl Selectable for PopupContext {
	fn next(&mut self) {
		if let Some(pop) = self.active_popup() {
			if pop.choices.is_empty() {
				return;
			}
			let current = self.selected_choice_idx;
			let next = (current + 1) % pop.choices.len();

			self.selected_choice_idx = next;
		}
	}

	fn previous(&mut self) {
		if let Some(pop) = self.active_popup().as_mut() {
			if pop.choices.is_empty() {
				return;
			}
			let current = self.selected_choice_idx;
			let previous = (current + pop.choices.len() - 1) % pop.choices.len();

			self.selected_choice_idx = previous;
		}
	}
}

impl State for PopupContext {
	type Output = usize;

	fn selected(&self) -> usize {
		self.selected_choice_idx
	}

	fn select(&mut self, index: usize) {
		self.selected_choice_idx = index;
	}
}

#[cfg(test)]
mod tests {
	use crate::context::Selectable;

	#[test]
	fn should_handle_empty_choices() {
		let mut popup = super::PopupContext::new();
		assert_eq!(popup.selected_choice_idx(), 0);

		popup.next();
		assert_eq!(popup.selected_choice_idx(), 0);

		popup.next();
		assert_eq!(popup.selected_choice_idx(), 0);

		popup.previous();
		assert_eq!(popup.selected_choice_idx(), 0);

		popup.previous();
		assert_eq!(popup.selected_choice_idx(), 0);
	}
}

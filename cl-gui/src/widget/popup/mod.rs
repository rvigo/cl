mod choice;
mod popup_trait;
mod popup_type;

use crate::{
	centered_rect,
	context::PopupContext,
	default_popup_block,
	event::PopupCallbackAction,
	theme::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR},
};
pub use choice::Choice;
pub use popup_type::Type;
use std::rc::Rc;
use tui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Modifier, Style},
	text::Line,
	widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct Popup {
	content: String,
	pub choices: Vec<Choice>,
	pub r#type: Type,
	pub callback: PopupCallbackAction,
}

impl Popup {
	pub fn new(
		content: String,
		choices: Vec<Choice>,
		r#type: Type,
		callback: PopupCallbackAction,
	) -> Popup {
		Self { content, choices, r#type, callback }
	}

	fn choices(&self) -> Vec<Choice> {
		match self.r#type {
			Type::Error => Choice::confirm(),
			Type::Warning => Choice::dialog(),
			Type::Help => Choice::empty(),
		}
	}

	fn button_widget(&self, selected: usize) -> Tabs<'_> {
		let choices: Vec<Line<'_>> =
			self.choices().iter().map(|choice| Line::from(choice.to_string())).collect();

		Tabs::new(choices)
			.block(Block::default().borders(Borders::NONE))
			.select(selected)
			.highlight_style(
				Style::default().fg(DEFAULT_SELECTED_COLOR).add_modifier(Modifier::UNDERLINED),
			)
			.divider("")
	}

	fn create_buttom_area(&self, area: Rect) -> Rect {
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(100)])
			.split(self.create_buttom_layout(area)[4]);

		let constraints = if self.choices().len() == 2 {
			vec![Constraint::Min(50)]
		} else {
			vec![Constraint::Percentage(50), Constraint::Percentage(50)]
		};
		let buttom_area =
			Layout::default().direction(Direction::Horizontal).constraints(constraints).split(layout[0]);

		buttom_area[buttom_area.len() - 1]
	}

	fn create_buttom_layout(&self, area: Rect) -> Rc<[Rect]> {
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(25),
				Constraint::Percentage(25),
				Constraint::Percentage(25),
				Constraint::Percentage(25),
			])
			.split(area);

		Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Percentage(20),
				Constraint::Percentage(20),
				Constraint::Percentage(20),
				Constraint::Percentage(20),
				Constraint::Percentage(20),
				Constraint::Length(3), //keeps the options inside the box
			])
			.split(layout[3])
	}

	fn content_width(&self) -> u16 {
		self.content.width() as u16
	}

	fn content_height(&self) -> u16 {
		const MIN_HEIGHT: usize = 5;

		let lines = self.content.lines().count();
		MIN_HEIGHT.max(lines) as u16
	}

	fn get_render_position(&self, area: Rect) -> Rect {
		let width = self.content_width();
		let height = self.content_height();

		let dynamic_height = (100 * (height * 2)) / area.height;
		let real_height = std::cmp::max(dynamic_height, area.height);
		centered_rect!(width, real_height, area)
	}
}

impl Widget for Popup {
	fn render(self, area: Rect, buf: &mut Buffer) {
		StatefulWidget::render(self, area, buf, &mut PopupContext::default());
	}
}

impl StatefulWidget for Popup {
	type State = PopupContext;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut PopupContext) {
		let block = default_popup_block!(self.r#type);

		let paragraph = Paragraph::new(self.content.to_owned())
			.style(Style::default().fg(DEFAULT_TEXT_COLOR))
			.alignment(Alignment::Left)
			.wrap(Wrap { trim: true })
			.block(block.to_owned());

		let render_position = self.get_render_position(area);

		Clear::render(Clear, render_position, buf);
		paragraph.render(render_position, buf);

		let options = self.button_widget(state.selected_choice_idx());
		let buttom_area = self.create_buttom_area(render_position);
		options.render(buttom_area, buf);
	}
}

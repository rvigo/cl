use crate::widget::{text_field::FieldType, DisplayWidget};
use cl_core::Command;
use std::{cell::RefCell, rc::Rc};

pub trait Observer {
	type ContentType;

	fn update(&mut self, event: Event<Self::ContentType>);
}

pub trait Sub<O>
where
	O: Observer,
{
	fn register(&mut self, observer: Rc<RefCell<O>>);

	fn notify(&mut self, event: Event<O::ContentType>);
}

pub trait Subject<O>
where
	O: Observer,
{
	fn get_observers(&self) -> &Vec<Rc<RefCell<O>>>;

	fn get_observers_mut(&mut self) -> &mut Vec<Rc<RefCell<O>>>;

	fn register(&mut self, observer: Rc<RefCell<O>>);

	fn notify(&mut self, event: Event<O::ContentType>);
}

#[derive(Debug, Clone)]
pub struct Event<T> {
	pub content: T,
}

impl<T> Event<T> {
	pub fn new(content: T) -> Self {
		Event { content }
	}
}

#[derive(Clone)]
pub struct CommandEvent {
	pub command: Command,
	pub highlight: bool,
	pub query: String,
}

impl CommandEvent {
	pub fn new(command: Command, query: String, highlight: bool) -> Self {
		CommandEvent { command, highlight, query }
	}
}

// pub struct PopupEvent<'p> {
//     content: &'p Content,
//     choices: &'p Vec<Choice>,
//     frame: &'p mut Frame<'p>,
//     popup_ctx: &'p mut Option<&'p mut PopupContext>,
// }

// impl<'p> PopupEvent<'p> {
//     pub fn new(
//         content: &'p Content,
//         choices: &'p Vec<Choice>,
//         frame: &'p mut Frame<'p>,
//         popup_ctx: &'p mut Option<&'p mut PopupContext>,
//     ) -> Self {
//         PopupEvent {
//             content,
//             choices,
//             frame,
//             popup_ctx,
//         }
//     }
// }

/// IMPLS
impl<'d> Observer for DisplayWidget<'d> {
	type ContentType = CommandEvent;

	fn update(&mut self, event: Event<Self::ContentType>) {
		let command = event.content.command;
		let content = match self.r#type {
			FieldType::Command => command.command.to_owned(),
			FieldType::Namespace => command.namespace.to_owned(),
			FieldType::Tags => command.tags_as_string(),
			FieldType::Description => command.description(),
			_ => "".to_owned(),
		};

		self.highlight(event.content.query);
		self.content = content;
		self.should_highlight = event.content.highlight;
	}
}

// impl Observer for dyn Popup {
//     type ContentType = PopupEvent<'static>;

//     fn update(&mut self, event: Event<Self::ContentType>) {
//         if let Some(ctx) = event.content.popup_ctx {
//             let content = &ctx.content;
//             let choices = &ctx.choices().to_owned();
//             let popup = popup!(content, Some(choices.to_vec()));
//             let frame = event.content.frame;
//             frame.render_stateful_widget(popup, frame.size(), &mut Some(ctx));
//         }
//     }
// }

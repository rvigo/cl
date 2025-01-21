use crate::component::Component;
use crate::observer::event::PopupAction;
use std::fmt;
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Frame;

#[derive(Clone)]
pub struct Button {
    content: String,
    pub action: PopupAction,
    pub on_click: fn() -> (),
    is_selected: bool,
}

impl Button {
    pub fn new(content: impl Into<String>, action: PopupAction, on_click: fn() -> ()) -> Self {
        Self {
            content: content.into(),
            action,
            is_selected: false,
            on_click,
        }
    }
}
impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Button")
            .field("content", &self.content)
            .field("action", &self.action)
            .field("is_selected", &self.is_selected)
            .finish()
    }
}

impl Component for Button {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.content.to_owned()).alignment(Center);
        if self.is_selected {
            match self.action {
                PopupAction::Confirm => &self.on_click,
                PopupAction::Cancel => &self.on_click,
            };
        }

        frame.render_widget(paragraph, area)
    }
}

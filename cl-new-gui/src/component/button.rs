use crate::component::Component;
use crate::observer::event::PopupAction;
use crate::state::state_event::StateEvent;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::style::Color::Yellow;
use tui::style::Style;
use tui::widgets::Paragraph;
use tui::Frame;

type FutureFn = fn(Sender<StateEvent>) -> Pin<Box<dyn Future<Output = ()> + Send>>;

#[derive(Clone)]
pub struct Button {
    content: String,
    pub action: PopupAction,
    pub on_click: FutureFn,
    pub is_selected: bool,
}

impl Button {
    pub fn new(content: impl Into<String>, action: PopupAction, on_click: FutureFn) -> Self {
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
        let style = if self.is_selected {
            Style::default()
        } else {
            Style::default().fg(Yellow)
        };
        let paragraph = Paragraph::new(self.content.to_owned())
            .alignment(Center)
            .style(style);

        frame.render_widget(paragraph, area)
    }
}

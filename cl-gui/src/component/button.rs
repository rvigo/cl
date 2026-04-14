use crate::component::Renderable;
use crate::screen::command::{ScreenCommand, ScreenCommandCallback};
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Paragraph;
use tui::Frame;

pub type StateEventFutureFn =
    fn(Sender<StateEvent>) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

pub type EventFutureFn =
    fn(Sender<Vec<ScreenCommand>>) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

#[derive(Clone, Debug)]
pub enum FutureEventType {
    State(StateEventFutureFn),
    Event(Sender<Vec<ScreenCommand>>, EventFutureFn),
}

impl FutureEventType {
    pub fn call(
        &self,
        state_sender: Option<Sender<StateEvent>>,
        screen_command_sender: Option<Sender<Vec<ScreenCommand>>>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        match self {
            FutureEventType::Event(_sx, event_fn) => {
                event_fn(screen_command_sender.expect("no screen command sender"))
            }
            FutureEventType::State(event_fn) => {
                event_fn(state_sender.expect("no state command sender"))
            }
        }
    }
}

#[derive(Clone)]
pub struct Button {
    content: String,
    pub on_click: FutureEventType,
    pub is_active: bool,
    pub callback: ScreenCommandCallback,
}

impl Button {
    pub fn new(
        content: impl Into<String>,
        active: bool,
        on_click: FutureEventType,
        callback: ScreenCommandCallback,
    ) -> Self {
        Self {
            content: content.into(),
            is_active: active,
            on_click,
            callback,
        }
    }
}

impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Button")
            .field("content", &self.content)
            .field("is_selected", &self.is_active)
            .finish()
    }
}

impl Renderable for Button {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let style = if self.is_active {
            Style::default().fg(theme.selected_color.into())
        } else {
            Style::default().fg(theme.text_color.into())
        };
        let paragraph = Paragraph::new(self.content.to_owned())
            .alignment(Center)
            .style(style.bg(theme.background_color.into()));

        frame.render_widget(paragraph, area)
    }
}

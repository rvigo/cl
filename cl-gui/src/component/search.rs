use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Clear};
use tui::Frame;
use tui_textarea::TextArea;

#[derive(Default, Debug, Clone)]
pub struct Search {
    pub textarea: TextArea<'static>,
}

impl Renderable for Search {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let block = Block::bordered()
            .style(
                Style::default()
                    .fg(theme.text_color.into())
                    .bg(theme.background_color.into()),
            )
            .title("Search");
        self.textarea.set_block(block);

        frame.render_widget(Clear, area);
        frame.render_widget(&self.textarea, area)
    }
}

impl crate::observer::event::NotifyTarget for Search {
    type Payload = crate::observer::event::SearchEvent;
    fn wrap(payload: Self::Payload) -> crate::observer::event::Event {
        crate::observer::event::Event::Search(payload)
    }
}

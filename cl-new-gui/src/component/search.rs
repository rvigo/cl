use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Clear, Paragraph};
use tui::Frame;
use tui_textarea::TextArea;

#[derive(Default, Debug)]
// TODO maybe change to quick search?
pub struct Search {
    pub textarea: TextArea<'static>,
}

impl Renderable for Search {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        let content = self.textarea.lines().join("\n");
        let paragraph = Paragraph::new(content).block(
            Block::bordered()
                .style(
                    Style::default()
                        .fg(theme.text_color.into())
                        .bg(theme.background_color.into()),
                )
                .title("Search"),
        );
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area)
    }
}

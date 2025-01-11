use crate::screen::Component;
use cl_core::Command;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

#[derive(Default, Clone)]
pub struct TextBox {
    pub command: Option<Command<'static>>,
}

impl TextBox {
    pub async fn update_command(&mut self, command: Command<'static>) {
        self.command = Some(command)
    }
}

impl Component for TextBox {
    fn render(&self, frame: &mut Frame) {
        let paragraph = Paragraph::new(format!("{:#?}", self.command))
            .block(Block::default().borders(Borders::all()));

        frame.render_widget(paragraph, frame.size())
    }
}

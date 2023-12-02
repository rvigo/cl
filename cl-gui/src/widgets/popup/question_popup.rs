use super::{option::Choice, popup_type::PopupType, Popup, WithOptions};
use crate::{
    entities::states::{popup_state::PopupState, State},
    widgets::WidgetExt,
    DEFAULT_TEXT_COLOR,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Paragraph, Widget, Wrap},
};

#[derive(Clone, Debug)]
pub struct QuestionPopup {
    pub content: String,
    popup_type: PopupType,
    choices: Vec<Choice>,
}

impl QuestionPopup {
    pub fn new<T>(message: T, answers: Vec<Choice>, popup_type: PopupType) -> QuestionPopup
    where
        T: Into<String>,
    {
        Self {
            content: message.into(),
            popup_type,
            choices: answers,
        }
    }
}

impl Popup for QuestionPopup {
    fn choices(&self) -> Vec<Choice> {
        self.choices.to_owned()
    }

    fn content_width(&self) -> u16 {
        self.content.len() as u16
    }

    fn content_height(&self) -> u16 {
        const MIN_HEIGHT: usize = 5;

        let lines = self.content.lines().count();
        MIN_HEIGHT.max(lines) as u16
    }

    fn render(self, area: Rect, buf: &mut Buffer, state: Option<&mut PopupState>) {
        if let Some(state) = state {
            let block = self.default_block(self.popup_type.to_string());

            let paragraph = Paragraph::new(self.content.to_owned())
                .style(Style::default().fg(DEFAULT_TEXT_COLOR))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .block(block.to_owned());

            let render_position = self.get_render_position(area);

            self.clear_area(render_position, buf);
            paragraph.render(render_position, buf);

            let options = self.button_widget(state.selected().unwrap_or(0));
            let buttom_area = self.create_buttom_area(render_position);
            options.render(buttom_area, buf);
        }
    }
}

impl WithOptions for QuestionPopup {}

impl WidgetExt for QuestionPopup {}

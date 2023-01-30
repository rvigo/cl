use crate::gui::layouts::{get_default_block, get_style, DEFAULT_TEXT_COLOR};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::hash::Hash;
use tui::{
    layout::Alignment,
    style::{Modifier, Style},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub enum FieldType {
    #[default]
    Alias,
    Tags,
    Command,
    Description,
    Namespace,
}

#[derive(Clone)]
pub struct Field<'a> {
    title: String,
    pub field_type: FieldType,
    in_focus: bool,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
    pub multiline: bool,
    pub text_area: TextArea<'a>,
}

impl<'a> Field<'a> {
    pub fn new(title: String, field_type: FieldType, in_focus: bool, multiline: bool) -> Field<'a> {
        Field {
            title,
            field_type,
            in_focus,
            block: None,
            style: Style::default(),
            alignment: Alignment::Left,
            multiline,
            text_area: TextArea::default(),
        }
    }

    pub fn toggle_focus(&mut self) {
        self.in_focus = !self.in_focus
    }

    pub fn block(&mut self, block: Block<'a>) {
        self.block = Some(block);
    }

    pub fn style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn input_as_string(&mut self) -> String {
        self.text_area.clone().into_lines().join("\n")
    }

    pub fn in_focus(&self) -> bool {
        self.in_focus
    }

    pub fn on_input(&mut self, input: KeyEvent) {
        if self.multiline {
            self.text_area.input(input);
        } else {
            // should avoid new lines
            match input {
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {}
                input => {
                    self.text_area.input(input);
                }
            }
        }
    }

    pub fn clear_input(&mut self) {
        self.text_area = self.default_text_area()
    }

    fn default_text_area(&self) -> TextArea<'a> {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_cursor_style(
            Style::default()
                .fg(DEFAULT_TEXT_COLOR)
                .add_modifier(Modifier::REVERSED),
        );

        text_area
    }
}

impl<'a> Widget for Field<'a> {
    fn render(mut self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        if self.in_focus() {
            self.text_area.set_cursor_style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .add_modifier(Modifier::REVERSED),
            );
        } else {
            self.text_area.set_cursor_style(Style::default());
        };

        self.text_area.set_block(if let Some(block) = self.block {
            block
        } else {
            get_default_block(self.title)
        });
        self.text_area.set_cursor_line_style(Style::default());
        self.text_area.set_alignment(self.alignment);
        self.text_area.set_style(get_style(self.in_focus));
        self.text_area.widget().render(area, buf)
    }
}

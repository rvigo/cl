use std::cmp::Ordering;
use unicode_width::UnicodeWidthStr;

pub enum FieldType {
    Alias,
    Tags,
    Command,
    Description,
    Namespace,
}

pub struct Field {
    name: String,
    title: String,
    field_type: FieldType,
    in_focus: bool,
    pub input: String,
    cursor_offset: u16,
}

impl Field {
    pub fn new(name: String, title: String, field_type: FieldType, in_focus: bool) -> Field {
        Field {
            name,
            title,
            field_type,
            in_focus,
            input: String::from(""),
            cursor_offset: 0,
        }
    }
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn in_focus(&self) -> bool {
        self.in_focus
    }
    pub fn cursor_offset(&self) -> u16 {
        self.cursor_offset
    }

    pub fn toggle_focus(&mut self) {
        self.clear_offset();
        self.in_focus = !self.in_focus
    }

    pub fn on_char(&mut self, c: char) {
        if self.cursor_offset == self.input_width() {
            self.input.push(c);
        } else {
            let idx: usize = self.cursor_offset as usize;
            self.input.insert(idx, c);
        }
        self.increase_cursor_offset();
    }

    pub fn on_backspace(&mut self) {
        if self.cursor_offset == self.input_width() {
            self.input.pop();
        } else if self.cursor_offset > 0 {
            let idx: usize = self.cursor_offset as usize;
            self.input.remove(idx);
        }
        self.decrease_cursor_offset()
    }

    pub fn on_delete_key(&mut self) {
        if self.input_width() > 0 {
            match self.cursor_offset.cmp(&(self.input_width() - 1)) {
                Ordering::Equal => {
                    self.input.pop();
                }
                Ordering::Less => {
                    let idx: usize = self.cursor_offset as usize;
                    self.input.remove(idx);
                }
                _ => {}
            }
        }
    }

    pub fn input_width(&self) -> u16 {
        self.input.width() as u16
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn increase_cursor_offset(&mut self) {
        if self.cursor_offset != self.input_width() {
            self.cursor_offset += 1;
        }
    }

    pub fn decrease_cursor_offset(&mut self) {
        if self.cursor_offset != 0 {
            self.cursor_offset -= 1
        }
    }

    fn clear_offset(&mut self) {
        self.cursor_offset = self.input_width()
    }
}

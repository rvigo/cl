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
}

impl Field {
    pub fn new(name: String, title: String, field_type: FieldType, in_focus: bool) -> Field {
        Field {
            name,
            title,
            field_type,
            in_focus,
            input: String::from(""),
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
    pub fn toggle_focus(&mut self) {
        self.in_focus = !self.in_focus
    }
    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }
    pub fn pop(&mut self) {
        self.input.pop();
    }
    pub fn clear_input(&mut self) {
        self.input.clear();
    }
}

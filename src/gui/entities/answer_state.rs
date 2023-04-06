#[derive(Default, Clone)]
pub struct AnswerState {
    selected: Option<usize>,
}

impl AnswerState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

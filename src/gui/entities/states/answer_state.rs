use super::State;

#[derive(Default, Clone)]
pub struct AnswerState {
    selected: Option<usize>,
}

impl State for AnswerState {
    type Output = Option<usize>;
    fn selected(&self) -> Option<usize> {
        self.selected
    }

    fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

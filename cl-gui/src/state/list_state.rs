use super::State;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct CommandListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl State for CommandListState {
    type Output = Option<usize>;

    fn select(&mut self, index: Self::Output) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    fn selected(&self) -> Self::Output {
        self.selected
    }
}

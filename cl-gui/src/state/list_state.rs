use super::State;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl State for ListState {
    type Output = Option<usize>;

    fn selected(&self) -> Self::Output {
        self.selected
    }

    fn select(&mut self, index: Self::Output) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

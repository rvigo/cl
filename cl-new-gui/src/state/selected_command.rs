use cl_core::Command;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct SelectedCommand {
    pub value: Command<'static>,
    pub current_idx: usize,
}

impl SelectedCommand {
    pub fn new(value: Command<'static>, current_idx: usize) -> Self {
        Self { value, current_idx }
    }

    pub fn from_vec(vec: &[Command<'static>]) -> Option<Self> {
        if !vec.is_empty() {
            Some(Self {
                value: vec.first().unwrap().to_owned(),
                current_idx: 0,
            })
        } else {
            None
        }
    }
}

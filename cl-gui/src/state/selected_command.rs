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

    pub fn first_from_vec(vec: &[Command<'static>]) -> Option<Self> {
        vec.first().map(|cmd| Self {
            value: cmd.to_owned(),
            current_idx: 0,
        })
    }
}

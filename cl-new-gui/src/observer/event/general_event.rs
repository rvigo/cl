use cl_core::Command;

#[derive(Clone, Debug)]
pub enum Event {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
    UpdateCommand(Command<'static>),
}

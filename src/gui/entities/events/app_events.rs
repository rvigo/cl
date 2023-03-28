use crate::command::Command;

#[derive(Clone, Debug)]
pub enum RenderEvents {
    Main,
    Edit,
    Insert,
}

#[derive(Clone, Debug)]
pub enum AppEvents {
    Run(CommandEvents),
    Render(RenderEvents),
    Quit,
}

#[derive(Clone, Debug)]
pub enum CommandEvents {
    Execute(Command),
    Edit {
        old_command: Command,
        edited_command: Command,
    },
    Insert(Command),
    Delete(Command),
}

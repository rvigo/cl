use crossterm::event::KeyEvent;

#[derive(Clone, Debug)]
pub enum InputMessages {
    KeyPress(KeyEvent),
}

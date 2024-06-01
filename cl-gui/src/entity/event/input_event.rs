use crossterm::event::KeyEvent;

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyPress(KeyEvent),
}

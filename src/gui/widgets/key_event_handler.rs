use crossterm::event::KeyEvent;

pub trait KeyEventHandler {
    fn on_input(&mut self, input: KeyEvent);
}

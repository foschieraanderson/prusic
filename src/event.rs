use crossterm::event::KeyCode;

pub enum Event {
    Quit,
    Key(KeyCode),
}

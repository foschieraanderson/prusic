pub struct App {
    pub running: bool,
    pub counter: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            counter: 0,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn increment(&mut self) {
        self.counter += 1;
    }
}

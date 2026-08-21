use crate::song::Song;

pub struct App {
    pub running: bool,
    pub songs: Vec<Song>,
    pub selected: usize,
}

impl App {
    pub fn new(songs: Vec<Song>) -> Self {
        Self {
            running: true,
            songs,
            selected: 0,
        }
    }
    pub fn select_next(&mut self) {
        if self.songs.is_empty() {
            return;
        }
        if self.selected + 1 < self.songs.len() {
            self.selected += 1;
        }
    }
    pub fn select_previous(&mut self) {
        if self.songs.is_empty() {
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}

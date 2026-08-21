use crate::song::Song;

pub struct App {
    pub running: bool,
    pub songs: Vec<Song>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        let songs = vec![
            Song {
                title: "Get Lucky".to_string(),
                artist: "Daft Punk".to_string(),
                album: "Random Access Memories".to_string(),
            },
            Song {
                title: "Time".to_string(),
                artist: "Pink Floyd".to_string(),
                album: "The Dark Side of the Moon".to_string(),
            },
            Song {
                title: "Don't Stop Me Now".to_string(),
                artist: "Queen".to_string(),
                album: "Jazz".to_string(),
            },
            Song {
                title: "One".to_string(),
                artist: "Metallica".to_string(),
                album: "...And Justice for All".to_string(),
            },
        ];
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

use crate::helpers::is_audio_file;
use crate::track::Track;
use rand::seq::SliceRandom;
use std::{
    fs::{self},
    io::{self},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    One,
    All,
}

pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current: usize,
    pub shuffle: bool,
    pub repeat: RepeatMode,
    shuffle_queue: Vec<usize>,
    shuffle_history: Vec<usize>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            current: 0,
            shuffle: false,
            repeat: RepeatMode::Off,
            shuffle_queue: Vec::new(),
            shuffle_history: Vec::new(),
        }
    }

    pub fn from_directory(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Diretório não encontrado: {}", path.display()),
            )
            .into());
        }

        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("O caminho não é um diretório: {}", path.display()),
            )
            .into());
        }

        let mut tracks = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if is_audio_file(&path) {
                match Track::new(path) {
                    Ok(track) => tracks.push(track),
                    Err(e) => eprintln!("Erro ao carregar música: {e}"),
                }
            }
        }

        tracks.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

        Ok(Self {
            tracks,
            current: 0,
            shuffle: false,
            repeat: RepeatMode::Off,
            shuffle_queue: Vec::new(),
            shuffle_history: Vec::new(),
        })
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);

        if self.shuffle {
            self.create_shuffle_queue();
        }
    }

    // pub fn add_tracks(&mut self, tracks: I)
    // where
    //     I: IntoIterator<Item = Track>,
    // {
    //     self.tracks.extend(tracks);
    //
    //     if self.shuffle {
    //         self.create_shuffle_queue();
    //     }
    // }

    pub fn current(&self) -> Option<&Track> {
        self.tracks.get(self.current)
    }

    pub fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        // Repetir a mesma música.
        if self.repeat == RepeatMode::One {
            return self.current();
        }

        if self.shuffle {
            return self.next_shuffle();
        }

        // Reprodução normal.
        if self.current + 1 < self.tracks.len() {
            self.current += 1;
            return self.current();
        }

        // Chegou ao final.
        match self.repeat {
            RepeatMode::All => {
                self.current = 0;
                self.current()
            }

            RepeatMode::Off => None,

            RepeatMode::One => unreachable!(),
        }
    }

    pub fn previous(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        if self.shuffle {
            return self.previous_shuffle();
        }

        if self.current == 0 {
            if self.repeat == RepeatMode::All {
                self.current = self.tracks.len() - 1;
            } else {
                return None;
            }
        } else {
            self.current -= 1;
        }

        self.current()
    }

    fn next_shuffle(&mut self) -> Option<&Track> {
        if self.shuffle_queue.is_empty() {
            match self.repeat {
                RepeatMode::All => {
                    self.create_shuffle_queue();
                }

                RepeatMode::Off => {
                    return None;
                }

                RepeatMode::One => {
                    return self.current();
                }
            }
        }

        let next = self.shuffle_queue.pop()?;

        // Guarda a música atual no histórico.
        self.shuffle_history.push(self.current);

        self.current = next;

        self.current()
    }

    fn previous_shuffle(&mut self) -> Option<&Track> {
        let previous = self.shuffle_history.pop()?;

        // A música atual volta para a fila.
        self.shuffle_queue.push(self.current);

        self.current = previous;

        self.current()
    }

    fn create_shuffle_queue(&mut self) {
        self.shuffle_queue = (0..self.tracks.len())
            .filter(|&index| index != self.current)
            .collect();

        let mut rng = rand::rng();

        self.shuffle_queue.shuffle(&mut rng);
    }

    pub fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;

        if self.shuffle {
            self.create_shuffle_queue();
            self.shuffle_history.clear();
        } else {
            self.shuffle_queue.clear();
            self.shuffle_history.clear();
        }
    }

    pub fn toggle_repeat(&mut self) {
        self.repeat = match self.repeat {
            RepeatMode::Off => RepeatMode::One,
            RepeatMode::One => RepeatMode::All,
            RepeatMode::All => RepeatMode::Off,
        };
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        if self.tracks.len() > 0 {
            return false;
        }
        return true;
    }
}

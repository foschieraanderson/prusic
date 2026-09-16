use crate::track::Track;
use std::{
    fs::{self},
    io::{self},
    path::Path,
};

pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current: usize,
}

impl Playlist {
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

        Ok(Self { tracks, current: 0 })
    }

    pub fn current(&self) -> Option<&Track> {
        self.tracks.get(self.current)
    }

    pub fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        self.current = (self.current + 1) % self.tracks.len();

        self.current()
    }

    pub fn previous(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        if self.current == 0 {
            self.current = self.tracks.len() - 1;
        } else {
            self.current -= 1;
        }

        self.current()
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }
}

fn is_audio_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };

    matches!(
        extension.to_lowercase().as_str(),
        "mp3" | "wav" | "flac" | "ogg" | "oga" | "m4a" | "aac"
    )
}

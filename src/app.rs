use crate::{
    library::Library,
    player::AudioPlayer,
    playlist::{Playlist, RepeatMode},
    track::Track,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    PlayerMode,
    LibraryMode,
    PlaylistMode,
    SearchMode,
    HelpMode,
}

pub struct App {
    pub mode: AppMode,
    pub duration: Duration,
    pub elapsed: Duration,
    pub playing: bool,
    pub repeat: RepeatMode,
    pub shuffle: bool,
    pub current_track: Option<Track>,
    pub cover_changed: bool,
    pub library: Library,
}

impl App {
    pub fn new(library: Library) -> Self {
        Self {
            mode: AppMode::PlayerMode,
            duration: Duration::from_secs(0),
            elapsed: Duration::from_secs(0),
            playing: false,
            repeat: RepeatMode::Off,
            shuffle: false,
            current_track: None,
            cover_changed: false,
            library,
        }
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn is_mode(&mut self, mode: AppMode) -> bool {
        self.mode == mode
    }

    pub fn progress(&self) -> f64 {
        if self.duration.is_zero() {
            return 0.0;
        }

        (self.elapsed.as_secs_f64() / self.duration.as_secs_f64()).clamp(0.0, 1.0)
    }

    pub fn tick(&mut self, delta: Duration) {
        if !self.playing {
            return;
        }

        self.elapsed += delta;

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.playing = false;
        }
    }

    pub fn seek(&mut self, seconds: i64) {
        let current = self.elapsed.as_secs() as i64;

        let next = (current + seconds)
            .max(0)
            .min(self.duration.as_secs() as i64);

        self.elapsed = Duration::from_secs(next as u64);
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::from_secs(0);
        self.duration = Duration::from_secs(0);
        self.playing = false;
        self.current_track = None;
    }

    pub fn play_current_track(
        &mut self,
        playlist: &Playlist,
        player: &mut AudioPlayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(track) = playlist.current() else {
            return Ok(());
        };

        let path = track.path.clone();

        player.play_file(&path)?;

        self.reset();

        if let Some(duration) = player.duration() {
            self.duration = duration;
        }

        self.playing = true;
        self.current_track = Some(track.clone());

        Ok(())
    }
}

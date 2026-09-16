use crate::track::Track;
use std::time::Duration;

pub struct App {
    pub duration: Duration,
    pub elapsed: Duration,
    pub playing: bool,
    pub repeat: bool,
    pub shuffle: bool,
    pub current_track: Option<Track>,
}

impl App {
    pub fn new() -> Self {
        Self {
            duration: Duration::from_secs(0),
            elapsed: Duration::from_secs(0),
            playing: false,
            repeat: false,
            shuffle: false,
            current_track: None,
        }
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
}

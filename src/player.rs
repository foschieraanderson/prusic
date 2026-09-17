use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

use std::{fs::File, path::Path, time::Duration};

pub struct AudioPlayer {
    stream: MixerDeviceSink,
    player: Player,

    volume: f32,
    pub current_duration: Option<Duration>,
    playing_track: bool,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = DeviceSinkBuilder::open_default_sink()?;

        let player = Player::connect_new(stream.mixer());

        Ok(Self {
            stream,
            player,

            volume: 0.5,
            current_duration: None,
            playing_track: false,
        })
    }

    pub fn play_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;

        let source = Decoder::try_from(file)?;

        self.current_duration = source.total_duration();

        self.player.stop();

        self.player = Player::connect_new(self.stream.mixer());

        self.player.set_volume(self.volume);

        self.player.append(source);
        self.player.play();
        self.playing_track = true;

        Ok(())
    }

    pub fn toggle_pause(&self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.current_duration = None;
        self.playing_track = false;
    }

    pub fn has_finished(&self) -> bool {
        self.playing_track && !self.player.is_paused() && self.player.empty()
    }

    pub fn increase_volume(&mut self) {
        self.set_volume(self.volume + 0.05);
    }

    pub fn decrease_volume(&mut self) {
        self.set_volume(self.volume - 0.05);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);

        self.player.set_volume(self.volume);
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn duration(&self) -> Option<Duration> {
        self.current_duration
    }
}

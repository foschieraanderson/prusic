use id3::{Tag, TagLike};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    time::Duration,
};

// ============================================================
// TRACK
// ============================================================

#[derive(Debug, Clone)]
struct Track {
    path: PathBuf,
    title: String,
    artist: String,
    album: String,
    year: Option<i32>,
}

impl Track {
    fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let tag = Tag::read_from_path(&path)?;

        let title = tag
            .title()
            .filter(|title| !title.trim().is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown")
                    .to_string()
            });

        let artist = tag.artist().unwrap_or("Unknown").to_string();
        let album = tag.album().unwrap_or("Unknown").to_string();
        let year = tag.year();

        Ok(Self {
            path,
            title,
            artist,
            album,
            year,
        })
    }
}

// ============================================================
// PLAYLIST
// ============================================================

struct Playlist {
    tracks: Vec<Track>,
    current: usize,
}

impl Playlist {
    fn from_directory(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
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

    fn current(&self) -> Option<&Track> {
        self.tracks.get(self.current)
    }

    fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        self.current = (self.current + 1) % self.tracks.len();

        self.current()
    }

    fn previous(&mut self) -> Option<&Track> {
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

    fn len(&self) -> usize {
        self.tracks.len()
    }
}

// ============================================================
// AUDIO FILE DETECTION
// ============================================================

fn is_audio_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };

    matches!(
        extension.to_lowercase().as_str(),
        "mp3" | "wav" | "flac" | "ogg" | "oga" | "m4a" | "aac"
    )
}

// ============================================================
// AUDIO PLAYER
// ============================================================

struct AudioPlayer {
    stream: MixerDeviceSink,
    player: Player,

    volume: f32,
    current_duration: Option<Duration>,
}

impl AudioPlayer {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = DeviceSinkBuilder::open_default_sink()?;

        let player = Player::connect_new(stream.mixer());

        Ok(Self {
            stream,
            player,

            volume: 0.5,
            current_duration: None,
        })
    }

    fn play_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Abrindo: {}", path.display());

        let file = File::open(path)?;

        let source = Decoder::try_from(file)?;

        self.current_duration = source.total_duration();

        self.player.stop();

        self.player = Player::connect_new(self.stream.mixer());

        self.player.set_volume(self.volume);

        self.player.append(source);

        self.player.play();

        Ok(())
    }

    fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    fn pause(&self) {
        self.player.pause();
    }

    fn resume(&self) {
        self.player.play();
    }

    fn stop(&mut self) {
        self.player.stop();
        self.current_duration = None;
    }

    fn increase_volume(&mut self) {
        self.set_volume(self.volume + 0.05);
    }

    fn decrease_volume(&mut self) {
        self.set_volume(self.volume - 0.05);
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);

        self.player.set_volume(self.volume);
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    fn is_empty(&self) -> bool {
        self.player.empty()
    }

    fn position(&self) -> Duration {
        self.player.get_pos()
    }

    fn duration(&self) -> Option<Duration> {
        self.current_duration
    }
}

// ============================================================
// HELPERS
// ============================================================

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{minutes:02}:{seconds:02}")
}

fn draw_progress(player: &AudioPlayer) {
    let position = player.position();

    let Some(duration) = player.duration() else {
        println!("Tempo: {}", format_duration(position));
        return;
    };

    let total = duration.as_secs_f64();
    let current = position.as_secs_f64();

    let percentage = if total > 0.0 {
        (current / total).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let width = 40usize;
    let filled = (percentage * width as f64) as usize;
    let empty = width - filled;

    println!(
        "[{}{}] {} / {}",
        "=".repeat(filled),
        "-".repeat(empty),
        format_duration(position),
        format_duration(duration),
    );
}

fn print_help() {
    println!();
    println!("Controles:");
    println!("  p       Play / Pause");
    println!("  s       Stop");
    println!("  n       Próxima");
    println!("  b       Anterior");
    println!("  +       Aumentar volume");
    println!("  -       Diminuir volume");
    println!("  h       Ajuda");
    println!("  q       Sair");
    println!();
}

// ============================================================
// MAIN
// ============================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let program = args.next().unwrap_or_else(|| "audio-player".to_string());

    let Some(music_directory) = args.next() else {
        eprintln!("Uso:");
        eprintln!("  {program} <diretorio-de-musicas>");
        eprintln!();
        eprintln!("Exemplo:");
        eprintln!("  {program} /home/user/Music");

        return Ok(());
    };

    let music_directory = PathBuf::from(music_directory);

    println!("Diretório: {}", music_directory.display());

    // --------------------------------------------------------
    // Playlist
    // --------------------------------------------------------

    let mut playlist = Playlist::from_directory(&music_directory)?;

    if playlist.len() == 0 {
        println!(
            "Nenhum arquivo de áudio encontrado em {}",
            music_directory.display()
        );

        return Ok(());
    }

    println!("{} arquivo(s) encontrado(s).", playlist.len());

    // --------------------------------------------------------
    // Player
    // --------------------------------------------------------

    let mut player = AudioPlayer::new()?;

    // --------------------------------------------------------
    // Primeira música
    // --------------------------------------------------------

    if let Some(track) = playlist.current() {
        let path = track.path.clone();

        println!("Reproduzindo: {}", track.title);

        player.play_file(&path)?;
    }

    print_help();

    // --------------------------------------------------------
    // Event loop
    // --------------------------------------------------------

    loop {
        println!();

        if let Some(track) = playlist.current() {
            println!("Faixa: {}", track.title);
            // if let Some(year) = track.year {
            //     println!(
            //         "Artista: {} | Álbum: {} ({})",
            //         track.artist, track.album, year
            //     );
            // } else {
            //     println!("Artista: {} | Álbum: {}", track.artist, track.album);
            // }

            let year = track
                .year
                .map(|year| format!(" ({year})"))
                .unwrap_or_default();

            println!("Artista: {} | Álbum: {}{}", track.artist, track.album, year);
        }

        let status = if player.is_paused() {
            "Pausado"
        } else if player.is_empty() {
            "Parado"
        } else {
            "Reproduzindo"
        };

        println!("Estado: {status}");

        println!("Volume: {:.0}%", player.volume() * 100.0);

        draw_progress(&player);

        print!("Comando: ");
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "p" => {
                player.toggle_pause();
            }

            "s" => {
                player.stop();
            }

            "n" => {
                if let Some(track) = playlist.next() {
                    let path = track.path.clone();

                    println!("Próxima: {}", track.title);

                    player.play_file(&path)?;
                }
            }

            "b" => {
                if let Some(track) = playlist.previous() {
                    let path = track.path.clone();

                    println!("Anterior: {}", track.title);

                    player.play_file(&path)?;
                }
            }

            "+" => {
                player.increase_volume();
            }

            "-" => {
                player.decrease_volume();
            }

            "h" => {
                print_help();
            }

            "q" => {
                player.stop();
                break;
            }

            "" => {}

            command => {
                println!("Comando desconhecido: {command}");
            }
        }

        // ----------------------------------------------------
        // Auto next
        // ----------------------------------------------------

        if player.is_empty() && !player.is_paused() {
            if let Some(track) = playlist.next() {
                let path = track.path.clone();

                println!("Próxima automaticamente: {}", track.title);

                player.play_file(&path)?;
            }
        }
    }

    println!("Player encerrado.");

    Ok(())
}

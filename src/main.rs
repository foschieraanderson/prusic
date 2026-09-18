mod app;
mod helpers;
mod keybindings;
mod library;
mod player;
mod playlist;
mod track;
mod ui;

use crate::app::AppMode;
use crate::helpers::{clear_image, show_image};
use crate::keybindings::handle_key;
use crate::library::Library;
use crate::player::AudioPlayer;
use crate::playlist::Playlist;
use crate::ui::{footer::render_footer, library::render_library, player::render_player};
use app::App;

use std::{
    env, io,
    path::PathBuf,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut args = env::args();

    let program = args.next().unwrap_or_else(|| "audio-player".to_string());

    let Some(music_directory) = args.next() else {
        eprintln!("Uso:");
        eprintln!("  {program} <diretorio-de-musicas>");
        eprintln!();
        eprintln!("Exemplo:");
        eprintln!("  {program} Music/");

        disable_raw_mode()?;

        return Ok(());
    };

    let music_directory = PathBuf::from(music_directory);

    let mut relative_path = dirs::home_dir().expect("Não foi possível encontrar a home");

    relative_path.push(&music_directory);

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, relative_path);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    music_directory: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    let library = Library::from_directory(&music_directory)?;

    let mut app = App::new(library);

    let mut playlist = Playlist::new();

    // if playlist.is_empty() {
    //     println!(
    //         "Nenhum arquivo de áudio encontrado em {}",
    //         music_directory.display()
    //     );
    //
    //     return Ok(());
    // }

    let mut player = AudioPlayer::new()?;

    // app.play_current_track(&playlist, &mut player)?;
    //
    // app.cover_changed = true;

    let mut cover_area: Option<Rect> = None;
    let mut previous_mode = app.mode;

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if handle_key(&mut app, &mut player, &mut playlist, key)? {
                    break;
                }
            }
        }

        if previous_mode != app.mode {
            clear_image()?;
            if app.is_mode(AppMode::LibraryMode) || app.is_mode(AppMode::PlayerMode) {
                app.cover_changed = true;
            }

            previous_mode = app.mode;
        }

        terminal.draw(|frame| {
            cover_area = render(frame, &mut app);
        })?;

        if last_tick.elapsed() >= tick_rate {
            let delta = last_tick.elapsed();

            app.tick(delta);

            last_tick = Instant::now();
        }

        if player.has_finished() {
            if playlist.next().is_some() {
                app.play_current_track(&playlist, &mut player)?;
            } else {
                app.playing = false;
            }
        }

        if app.cover_changed {
            if let Some(cover_area) = cover_area {
                if let Some(track) = &app.current_track {
                    if let Some(cover) = &track.cover {
                        show_image(cover, cover_area)?;
                    } else {
                        clear_image()?;
                    }
                }
            }

            app.cover_changed = false;
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &mut App) -> Option<Rect> {
    let area = frame.area();

    let [content, footer] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    let cover = match app.mode {
        AppMode::PlayerMode => render_player(frame, content, app),
        AppMode::LibraryMode => render_library(frame, content, app),
        _ => None,
    };

    render_footer(frame, footer);

    cover
}

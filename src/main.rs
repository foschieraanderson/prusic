mod app;
mod helpers;
mod player;
mod playlist;
mod track;
mod ui;

use crate::helpers::{print_help, show_image};
use crate::player::AudioPlayer;
use crate::playlist::Playlist;
use crate::ui::footer::render_footer;
use crate::ui::player::render_player;
use app::App;
use std::{
    env, io,
    path::PathBuf,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
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

        return Ok(());
    };

    let music_directory = PathBuf::from(music_directory);
    let mut relative_path: PathBuf = dirs::home_dir().expect("Não foi possível encontrar a home");
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
    let mut app = App::new();

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    let mut playlist = Playlist::from_directory(&music_directory)?;

    if playlist.len() == 0 {
        println!(
            "Nenhum arquivo de áudio encontrado em {}",
            music_directory.display()
        );

        return Ok(());
    }

    let mut player = AudioPlayer::new()?;

    app.play_current_track(&playlist, &mut player, &mut last_tick)?;

    let mut cover_changed = true;
    let mut cover_area = Rect::default();

    loop {
        terminal.draw(|frame| {
            cover_area = render(frame, &app);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        player.stop();
                        break;
                    }

                    KeyCode::Char('p') => {
                        app.playing = !app.playing;
                        player.toggle_pause();
                    }

                    // ------------------------------------------------
                    // Stop
                    // ------------------------------------------------
                    // KeyCode::Char('s') => {
                    //     player.stop();
                    //     app.playing = false;
                    // }
                    KeyCode::Char('n') => {
                        if playlist.next().is_some() {
                            app.play_current_track(&playlist, &mut player, &mut last_tick)?;

                            cover_changed = true;
                        }
                    }

                    KeyCode::Char('b') => {
                        if playlist.previous().is_some() {
                            app.play_current_track(&playlist, &mut player, &mut last_tick)?;

                            cover_changed = true;
                        }
                    }

                    KeyCode::Char('z') => {
                        playlist.toggle_shuffle();
                        app.shuffle = playlist.shuffle;
                    }

                    KeyCode::Char('r') => {
                        playlist.toggle_repeat();
                        app.repeat = playlist.repeat;
                    }

                    KeyCode::Char('+') => {
                        player.increase_volume();
                    }

                    KeyCode::Char('-') => {
                        player.decrease_volume();
                    }

                    KeyCode::Char('h') => {
                        print_help();
                    }

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let delta = last_tick.elapsed();

            app.tick(delta);

            last_tick = Instant::now();
        }

        if player.has_finished() {
            if playlist.next().is_some() {
                app.play_current_track(&playlist, &mut player, &mut last_tick)?;

                cover_changed = true;
            } else {
                app.playing = false;
            }
        }

        if cover_changed {
            if let Some(track) = &app.current_track {
                if let Some(cover) = &track.cover {
                    show_image(cover, cover_area)?;
                }
            }

            cover_changed = false;
        }
    }

    Ok(())
}
// fn run(
//     terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
//     music_directory: PathBuf,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut app = App::new();
//
//     let tick_rate = Duration::from_millis(50);
//     let mut last_tick = Instant::now();
//
//     let mut playlist = Playlist::from_directory(&music_directory)?;
//
//     if playlist.len() == 0 {
//         println!(
//             "Nenhum arquivo de áudio encontrado em {}",
//             music_directory.display()
//         );
//
//         return Ok(());
//     }
//
//     app.current_track = playlist.current().cloned();
//
//     // println!("{} arquivo(s) encontrado(s).", playlist.len());
//
//     let mut player = AudioPlayer::new()?;
//
//     // --------------------------------------------------------
//     // Primeira música
//     // --------------------------------------------------------
//
//     if let Some(track) = playlist.current() {
//         let path = track.path.clone();
//
//         player.play_file(&path)?;
//         app.playing = true;
//
//         if let Some(duration) = player.current_duration {
//             app.duration = duration;
//         }
//     }
//
//     let mut cover_changed = true;
//     let mut cover_area = Rect::default();
//
//     loop {
//         terminal.draw(|frame| {
//             cover_area = render(frame, &app);
//         })?;
//
//         let timeout = tick_rate.saturating_sub(last_tick.elapsed());
//
//         if event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 match key.code {
//                     KeyCode::Char('q') => {
//                         player.stop();
//                         break;
//                     }
//
//                     KeyCode::Char('p') => {
//                         app.playing = !app.playing;
//                         player.toggle_pause();
//                     }
//
//                     // KeyCode::Left => {
//                     //     app.seek(-5);
//                     // }
//                     //
//                     // KeyCode::Right => {
//                     //     app.seek(5);
//                     // }
//                     KeyCode::Char('s') => {
//                         player.stop();
//                     }
//                     KeyCode::Char('n') => {
//                         if let Some(track) = playlist.next() {
//                             app.reset();
//                             last_tick = Instant::now();
//                             let path = track.path.clone();
//
//                             player.play_file(&path)?;
//
//                             if let Some(duration) = player.current_duration {
//                                 app.duration = duration;
//                             }
//                             app.playing = true;
//                             app.current_track = Some(track.clone());
//                             cover_changed = true;
//                         }
//                     }
//                     KeyCode::Char('b') => {
//                         if let Some(track) = playlist.previous() {
//                             app.reset();
//                             last_tick = Instant::now();
//                             let path = track.path.clone();
//
//                             player.play_file(&path)?;
//                             if let Some(duration) = player.current_duration {
//                                 app.duration = duration;
//                             }
//                             app.playing = true;
//                             app.current_track = Some(track.clone());
//                             cover_changed = true;
//                         }
//                     }
//                     KeyCode::Char('+') => {
//                         player.increase_volume();
//                     }
//                     KeyCode::Char('-') => {
//                         player.decrease_volume();
//                     }
//                     KeyCode::Char('h') => {
//                         print_help();
//                     }
//
//                     _ => {}
//                 }
//             }
//         }
//
//         if last_tick.elapsed() >= tick_rate {
//             let delta = last_tick.elapsed();
//
//             app.tick(delta);
//
//             last_tick = Instant::now();
//         }
//         if app.elapsed >= app.duration {
//             if let Some(track) = playlist.next() {
//                 app.reset();
//                 last_tick = Instant::now();
//                 let path = track.path.clone();
//
//                 player.play_file(&path)?;
//
//                 if let Some(duration) = player.current_duration {
//                     app.duration = duration;
//                 }
//                 app.playing = true;
//                 app.current_track = Some(track.clone());
//                 cover_changed = true;
//             }
//         }
//
//         if cover_changed {
//             if let Some(track) = &app.current_track {
//                 if let Some(cover) = &track.cover {
//                     show_image(cover, cover_area)?;
//                 }
//             }
//
//             cover_changed = false;
//         }
//     }
//
//     Ok(())
// }

fn render(frame: &mut Frame, app: &App) -> Rect {
    let area = frame.area();

    let [content, footer] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    let cover = render_player(frame, content, app);
    render_footer(frame, footer);

    cover
}

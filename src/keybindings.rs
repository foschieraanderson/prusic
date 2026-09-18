use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppMode},
    player::AudioPlayer,
    playlist::Playlist,
};

pub fn handle_key(
    app: &mut App,
    player: &mut AudioPlayer,
    playlist: &mut Playlist,
    key: KeyEvent,
) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('1') => {
            app.set_mode(AppMode::PlayerMode);
        }
        KeyCode::Char('2') => {
            app.set_mode(AppMode::LibraryMode);
        }
        KeyCode::Char('q') => {
            player.stop();
            return Ok(true);
        }
        _ => match app.mode {
            AppMode::PlayerMode => {
                handle_player_keys(app, player, playlist, key)?;
            }
            AppMode::LibraryMode => {
                handle_library_keys(app, player, playlist, key);
            }

            AppMode::PlaylistMode => {
                handle_playlist_keys(app, playlist, key);
            }

            AppMode::SearchMode => {
                handle_search_keys(app, key);
            }

            AppMode::HelpMode => {
                handle_help_keys(app, key);
            }
        },
    }

    Ok(false)
}

fn handle_player_keys(
    app: &mut App,
    player: &mut AudioPlayer,
    playlist: &mut Playlist,
    key: KeyEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('p') | KeyCode::Char(' ') => {
            app.playing = !app.playing;
            player.toggle_pause();
        }

        KeyCode::Char('n') => {
            if playlist.next().is_some() {
                app.play_current_track(playlist, player)?;
            }
        }

        KeyCode::Char('b') => {
            if playlist.previous().is_some() {
                app.play_current_track(playlist, player)?;
            }
        }

        KeyCode::Char('l') => {
            app.set_mode(AppMode::LibraryMode);
        }

        KeyCode::Char('z') => {
            playlist.toggle_shuffle();
            app.shuffle = playlist.shuffle;
        }

        KeyCode::Char('r') => {
            playlist.toggle_repeat();
            app.repeat = playlist.repeat;
        }

        _ => {}
    }

    Ok(())
}

fn handle_library_keys(
    app: &mut App,
    player: &mut AudioPlayer,
    playlist: &mut Playlist,
    key: KeyEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        // KeyCode::Char('j') | KeyCode::Down => {}
        // KeyCode::Char('k') | KeyCode::Up => {}
        KeyCode::Up => {
            app.library.select_previous();
        }

        KeyCode::Down => {
            app.library.select_next();
        }

        KeyCode::Char(' ') => {
            app.library.toggle_track_selection();
        }

        KeyCode::Enter => {
            app.library.toggle_selected();
        }

        KeyCode::Char('a') => {
            let was_empty = playlist.is_empty();

            app.library.add_selected_to_playlist(playlist);

            if was_empty && !playlist.is_empty() {
                app.play_current_track(playlist, player)?;
                app.set_mode(AppMode::PlayerMode);
            }
        }

        _ => {}
    }

    Ok(())
}

fn handle_playlist_keys(app: &mut App, playlist: &mut Playlist, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {}

        KeyCode::Char('j') | KeyCode::Down => {}

        KeyCode::Char('k') | KeyCode::Up => {}

        KeyCode::Enter => {}

        _ => {}
    }
}

fn handle_search_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {}

        KeyCode::Char('c') => {}

        KeyCode::Backspace => {}

        _ => {}
    }
}

fn handle_help_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') => {
            app.set_mode(AppMode::PlayerMode);
        }

        _ => {}
    }
}

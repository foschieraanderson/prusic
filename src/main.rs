mod app;
mod event;
mod scanner;
mod song;
mod ui;

use std::{env, io, path::PathBuf};

use crossterm::{
    event::{Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;
use event::Event;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let home = env::var("HOME").map_err(io::Error::other)?;

    let music_dir = PathBuf::from(home).join("Music").join("Alternative");
    if !music_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Music directory not found: {}", music_dir.display()),
        ));
    }

    let songs = scanner::scan_songs(&music_dir)?;

    let mut app = App::new(songs);

    while app.running {
        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if let CrosstermEvent::Key(key) = crossterm::event::read()? {
            let event = Event::Key(key.code);

            match event {
                Event::Key(KeyCode::Char('q')) => {
                    app.quit();
                }

                Event::Key(KeyCode::Esc) => {
                    app.quit();
                }

                Event::Key(KeyCode::Up) => {
                    app.select_previous();
                }

                Event::Key(KeyCode::Down) => {
                    app.select_next();
                }

                Event::Key(_) => {}

                Event::Quit => {
                    app.quit();
                }
            }
        }
    }

    Ok(())
}

mod app;
mod event;
mod ui;

use std::io;

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
    let mut app = App::new();

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

                Event::Key(KeyCode::Char('i')) => {
                    app.increment();
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

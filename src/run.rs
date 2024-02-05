use crate::event::{AppEvent, EventReader};
use crate::page::{
    app::{App, AppState},
    Page, StatefulPage,
};
use anyhow::{Ok, Result};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub async fn run() -> Result<()> {
    // initialize terminal
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    // Event Reader
    let mut event_reader = EventReader::new();
    // Component Service
    // Application Page
    let mut app = App::new();
    app.init().await;
    // Main Loop
    loop {
        if app.get_state() == AppState::Quit {
            break;
        }
        let event = event_reader.read().await?;
        if &event == &AppEvent::Render {
            terminal.draw(|f| app.render(f, f.size()))?;
        }
        let mut msg = app.handle_events(event).await;
        while msg != None {
            msg = app.update(msg.unwrap()).await;
        }
    }
    // de-initialize terminal
    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("Goodbye!");
    Ok(())
}

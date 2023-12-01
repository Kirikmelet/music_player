use crate::event::EventReader;
use crate::page::app::AppState;
use crate::page::PageMsgActor;
use crate::page::{app::App, Page, StatefulPage};
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
    // Application Page
    let mut app = App::new();
    app.init(PageMsgActor {
        tx: Some(event_reader.get_sender()),
        id: "app".to_string(),
        parent_id: "".to_string(),
    });
    // Main Loop
    loop {
        terminal.draw(|f| app.render(f, f.size()))?;
        if app.get_state() == AppState::Quit {
            break;
        }
        app.handle_events(event_reader.read().await).await?;
    }
    // de-initialize terminal
    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("Goodbye!");
    Ok(())
}

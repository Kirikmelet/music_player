use std::io::Stdout;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

use std::io::stdout;
use std::thread::JoinHandle;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{App, AppState};
use crate::events::{EventEnum, Events};
use crate::input::_Input;
use crate::ui::Ui;
use crate::ui::{base_ui::BaseUi, AppUi};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn end_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn run(app: Arc<Mutex<App>>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let g_global_kill: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let events: Events = Events::run(g_global_kill.clone());
    let _input = _Input::new(app.clone());
    let mut ui: Ui = Ui::new();
    loop {
        terminal.draw(|f| {
            ui.render(f);
        })?;
        {
            //let event = events.read();
            //input.read(event).unwrap();
            let Ok(mut app) = app.try_lock() else {
                continue;
            };
            //match app.state {
            //    AppState::Quit => {
            //        g_global_kill.store(true, Relaxed);
            //        break;
            //    }
            //    AppState::Normal => {}
            //}
            match events.read() {
                Some(EventEnum::Input(key_event)) => match key_event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        kind: KeyEventKind::Press,
                        modifiers: KeyModifiers::NONE,
                        state: KeyEventState::NONE,
                    } => {
                        break;
                    }
                    _ => {}
                },
                _ => {
                    ui.update(None);
                }
            }
        }
    }
    end_terminal(&mut terminal)?;
    // events.join();
    Ok(())
}

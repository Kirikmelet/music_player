use anyhow::Result;
use app::App;
use config::Config;
use std::sync::{Arc, Mutex};

mod app;
mod audio;
mod config;
mod events;
mod fs;
mod input;
mod run;
mod ui;

fn main() -> Result<()> {
    let app = Arc::new(Mutex::new(App::default()));
    run::run(app)?;
    Ok(())
}

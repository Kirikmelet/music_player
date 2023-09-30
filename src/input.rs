use std::sync::{Arc, Mutex};

use anyhow::{Error, Result};

use crate::{app::App, events::EventEnum};

pub struct _Input {
    app_data: Arc<Mutex<App>>,
}

impl _Input {
    pub fn new(_app: Arc<Mutex<App>>) -> Self {
        Self { app_data: _app }
    }
    pub fn read(&self, event: EventEnum) -> Result<(), ()> {
        let Ok(mut app) = self.app_data.try_lock() else {
            return Err(());
        };
        Ok(())
    }
}

use std::{future::Future, thread, time::Duration};

use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use crate::page::app::App;

pub type EventCallback = dyn Future<Output = Result<(), anyhow::Error>>;

#[derive(Debug, Clone)]
pub struct AppEventMsg {
    pub sender_id: String,
    pub recipient_id: String, // The intended recipient
    pub msg_id: &'static str, // The msg title
    pub msg: Vec<u8>,         // The message (encoded in bytes for shits and giggles)
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Msg(AppEventMsg), // sender, msg details
}

pub struct EventReader {
    rx: UnboundedReceiver<AppEvent>,
    tx: UnboundedSender<AppEvent>,
}

impl EventReader {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<AppEvent>();
        let thread_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(250)).unwrap() {
                if match event::read().unwrap() {
                    Event::Key(x) => thread_tx.send(AppEvent::Key(x)),
                    Event::Mouse(x) => thread_tx.send(AppEvent::Mouse(x)),
                    _ => thread_tx.send(AppEvent::Tick),
                }
                .is_err()
                {
                    break;
                };
            }
        });
        Self { rx, tx }
    }
    pub async fn read(&mut self) -> AppEvent {
        self.rx.try_recv().unwrap_or(AppEvent::Tick)
    }
    pub fn get_sender(&mut self) -> UnboundedSender<AppEvent> {
        self.tx.clone()
    }
}

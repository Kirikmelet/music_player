use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::app::{App, AppState};
use crossterm::event::{poll, read, Event, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventEnum {
    //App(AppState),
    Input(KeyEvent),
}

pub struct Events {
    rx: Receiver<Option<EventEnum>>,
    _tx: Sender<Option<EventEnum>>,
    thread: JoinHandle<()>,
}

impl Events {
    pub fn run(kill_flag: Arc<AtomicBool>) -> Self {
        let (_tx, rx) = channel::<Option<EventEnum>>();
        let tx = _tx.clone();
        let thread = thread::spawn(move || loop {
            if poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = read().unwrap() {
                    {
                        tx.send(Some(EventEnum::Input(key_event))).unwrap();
                    }
                }
            }
            tx.send(None).unwrap();
        });
        Self { rx, _tx, thread }
    }
    pub fn read(&self) -> Option<EventEnum> {
        self.rx.recv().unwrap_or(None)
    }
    pub fn join(self) {
        self.thread.join().unwrap();
    }
}

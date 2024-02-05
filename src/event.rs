use anyhow::anyhow;
use std::time::Duration;

use futures::{FutureExt, StreamExt};

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, MouseEvent};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

#[derive(Debug, PartialEq)]
pub enum AppEvent {
    Tick,
    Error,
    Render,
    // Input
    Key(KeyCode),
    Mouse(MouseEvent),
}

pub struct EventReader {
    rx: UnboundedReceiver<AppEvent>,
    _tx: UnboundedSender<AppEvent>,
}

impl EventReader {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<AppEvent>();
        let thread_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let mut event_stream: EventStream = EventStream::new();
                let event_read = event_stream.next().fuse();
                let mut tick_interval = tokio::time::interval(Duration::from_millis(500));
                let mut render_interval =
                    tokio::time::interval(Duration::from_secs_f64(1f64 / 60f64));
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                select! {
                    event_opt = event_read => {
                        match event_opt {
                            Some(Ok(event)) => {
                                match event {
                                    Event::Key(x) if x.kind == KeyEventKind::Press => {thread_tx.send(AppEvent::Key(x.code)).unwrap();}
                                    Event::Mouse(x) => {thread_tx.send(AppEvent::Mouse(x)).unwrap();}
                                    _ => {}
                                }
                            }
                            Some(Err(_)) => {
                                thread_tx.send(AppEvent::Error).unwrap();
                            }
                            None => {}
                        };
                    },
                    _ = tick_delay => {
                        thread_tx.send(AppEvent::Tick).unwrap();
                    },
                    _ = render_delay => {
                        thread_tx.send(AppEvent::Render).unwrap();
                    }
                }
            }
            //loop {
            //    if event::poll(Duration::from_millis(250)).unwrap() {
            //        match event::read().unwrap() {
            //            Event::Key(x) => thread_tx.send(AppEvent::Key(x.code)),
            //            Event::Mouse(x) => thread_tx.send(AppEvent::Mouse(x)),
            //            _ => thread_tx.send(AppEvent::Tick),
            //        }
            //        .unwrap()
            //    }
            //}
        });
        Self { rx, _tx: tx }
    }
    pub async fn read(&mut self) -> anyhow::Result<AppEvent> {
        self.rx.recv().await.ok_or(anyhow!("cum"))
    }
}

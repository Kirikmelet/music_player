use crate::event::AppEvent;
use anyhow::Result;
use async_trait::async_trait;
use ratatui::{prelude::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

pub mod app;
pub mod file_list;

#[derive(Debug, Clone, Default)]
pub struct PageMsgActor {
    pub tx: Option<UnboundedSender<AppEvent>>,
    pub id: String,
    pub parent_id: String,
}

#[async_trait]
pub trait Page {
    type Message;
    async fn update(&mut self, msg: Self::Message) -> Result<()>;
    async fn handle_events(&mut self, event: AppEvent) -> Result<()>;
    fn render(&mut self, frame: &mut Frame, size: Rect);
    fn register_actor_details(&mut self, data: PageMsgActor);
    fn get_actor_details(&self) -> &PageMsgActor;
}

pub trait StatefulPage: Page {
    type State;
    fn get_state(&self) -> Self::State;
}

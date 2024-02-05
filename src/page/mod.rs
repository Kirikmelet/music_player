/* Messages; Global */

use crate::event::AppEvent;
use async_trait::async_trait;
use ratatui::{prelude::Rect, Frame};

pub mod app;
pub mod file_list;

pub trait Msg: Send + Sync {}

#[async_trait]
pub trait Page {
    fn render(&mut self, frame: &mut Frame, size: Rect);
}

#[async_trait]
pub trait StatefulPage: Page {
    type State;
    type Message: Msg;
    fn get_state(&self) -> Self::State;
    async fn update(&mut self, msg: Self::Message) -> Option<Self::Message>;
    async fn handle_events(&mut self, event: AppEvent) -> Option<Self::Message>;
}

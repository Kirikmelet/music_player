use std::path::Path;

use crate::db::audio_scanner::AudioScanner;

use super::Page;
use async_trait::async_trait;
use futures::pin_mut;
use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct FileList {
    file_list: Vec<String>,
    file_list_state: ListState,
    style: Style,
}

impl FileList {
    pub fn new() -> Self {
        let mut file_list_state = ListState::default();
        let file_list: Vec<String> = Vec::new();
        file_list_state.select(Some(0));
        let style = Style::default();
        Self {
            style,
            file_list,
            file_list_state,
        }
    }
    pub fn _set_file_list(&mut self, file_list: Vec<String>) {
        tracing::info!("Recieved: {} items", file_list.len());
        self.file_list = file_list;
    }
    pub async fn update_file_list<P: AsRef<Path>>(&mut self, path: P) {
        let stream = AudioScanner::scan_dir(path);
        // Only for debugging
        let mut item_count: i32 = 0;
        // We need to pin this as of now
        pin_mut!(stream);
        // For loops dont work with streams (yet)
        while let Some(item) = stream.next().await {
            tracing::info!("Recieved item: {} item", item.display());
            self.file_list.push(item.display().to_string());
            item_count += 1;
        }
        tracing::info!("Recieved total items: {}", item_count);
    }
    pub fn next(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => {
                if i >= self.file_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }
    pub fn prev(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.file_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }
}

#[async_trait]
impl Page for FileList {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().borders(Borders::ALL).title("File list");
        if self.file_list.is_empty() {
            let error_text = Paragraph::new("Directory is empty/invalid!")
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(error_text, rect);
            return;
        }
        let items: Vec<ListItem<'_>> = self
            .file_list
            .iter()
            .map(|f| ListItem::new(f.as_str()))
            .collect();
        let list = List::new(items)
            .block(block)
            .style(self.style)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(list, rect, &mut self.file_list_state);
    }
}

//#[async_trait]
//impl StatefulPage for FileList {
//    type State = FileListState;
//    type Message = FileListMsg;
//    async fn update(&mut self, msg: Self::Message) -> Option<Self::Message> {
//        match msg {
//            FileListMsg::State(state) => {
//                self.state = state;
//                match state {
//                    FileListState::Inactive => {
//                        self.style = self.style.add_modifier(Modifier::DIM);
//                    }
//                    FileListState::Active => {
//                        self.style = self.style.remove_modifier(Modifier::DIM);
//                    }
//                };
//            }
//            FileListMsg::UpdateFileList(list) => {
//                self.file_list = list;
//            }
//            FileListMsg::Increment => {
//                self.next();
//            }
//            FileListMsg::Decrement => {
//                self.prev();
//            }
//        }
//        None
//    }
//    async fn handle_events(&mut self, event: AppEvent) -> Option<Self::Message> {
//        if self.get_state() == FileListState::Inactive {
//            return None;
//        }
//        match event {
//            AppEvent::Key(x) => match x {
//                KeyCode::Char('j') => return Some(FileListMsg::Increment),
//                KeyCode::Char('k') => return Some(FileListMsg::Decrement),
//                _ => {}
//            },
//            _ => {}
//        }
//        None
//    }
//    fn get_state(&self) -> Self::State {
//        self.state
//    }
//}

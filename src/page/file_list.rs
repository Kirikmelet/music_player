use std::path::PathBuf;

use crate::event::AppEvent;

use super::{Page, PageMsgActor, StatefulPage};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use walkdir::{DirEntry, WalkDir};

pub const FILE_LIST_MSG_REFRESH_DB: &'static str = "file_list_msg_refresh_db";
pub const FILE_LIST_MSG_GET_DB: &'static str = "file_list_msg_get_db";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileListMsg {
    SetDir(PathBuf),
    State(FileListState),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum FileListState {
    #[default]
    Active,
    Inactive,
}

#[derive(Debug, Clone, Default)]
pub struct FileList {
    dir: PathBuf,
    file_list: Vec<DirEntry>,
    file_list_state: ListState,
    style: Style,
    state: FileListState,
    actor: PageMsgActor,
}

impl FileList {
    pub fn new() -> Self {
        // todo! Impelement a config system! For now I am using hard-coded directories
        let dir = PathBuf::from("C:\\Users\\troyd\\Music");
        let mut file_list_state = ListState::default();
        let file_list = Self::build_file_contents(&dir);
        if !file_list.is_empty() {
            file_list_state.select(Some(0));
        }
        let style = Style::default();
        Self {
            dir,
            style,
            file_list,
            file_list_state,
            ..Default::default()
        }
    }
    pub fn init(&mut self, actor: PageMsgActor) {
        self.register_actor_details(actor.clone());
    }
    fn build_file_contents(dir: &PathBuf) -> Vec<DirEntry> {
        let directory_entries = WalkDir::new(dir);
        directory_entries
            .sort_by_file_name()
            .into_iter()
            .filter_map(|f| f.ok())
            .collect()
    }
    fn next(&mut self) {
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
    fn prev(&mut self) {
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
    type Message = FileListMsg;
    async fn update(&mut self, msg: Self::Message) -> Result<()> {
        match msg {
            FileListMsg::SetDir(dir) => {
                self.dir = dir.clone();
                self.file_list = Self::build_file_contents(&dir);
                let _ = self.file_list.pop();
                return Ok(());
            }
            FileListMsg::State(state) => {
                self.state = state;
                match state {
                    FileListState::Inactive => {
                        self.style = self.style.add_modifier(Modifier::DIM);
                    }
                    FileListState::Active => {
                        self.style = self.style.remove_modifier(Modifier::DIM);
                    }
                };
                return Ok(());
            }
        }
    }
    async fn handle_events(&mut self, event: AppEvent) -> Result<()> {
        if self.get_state() == FileListState::Inactive {
            return Ok(());
        }
        match event {
            AppEvent::Key(x) => match x {
                KeyEvent {
                    code: KeyCode::Char('j'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.next();
                    return Ok(());
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.prev();
                    return Ok(());
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    let actor = self.actor.clone();
                    if let Some(action_tx) = actor.tx {}
                    return Ok(());
                }
                KeyEvent {
                    code: KeyCode::Char('R'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    let actor = self.actor.clone();
                    if let Some(action_tx) = actor.tx {}
                    return Ok(());
                }
                _ => Ok(()),
            },
            AppEvent::Msg(_sender, app_msg) => return Ok(()),
            _ => Ok(()),
        }
    }
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
            .filter_map(|f| {
                let name = f.file_name().to_str().map(|f| String::from(f));
                let style = if f.file_type().is_dir() {
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                match name {
                    Some(name) => Some(ListItem::new(name).style(style)),
                    None => None,
                }
            })
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
    fn register_actor_details(&mut self, tx: PageMsgActor) {
        self.actor = tx;
    }
    fn get_actor_details(&self) -> &PageMsgActor {
        &self.actor
    }
}

impl StatefulPage for FileList {
    type State = FileListState;
    fn get_state(&self) -> Self::State {
        self.state
    }
}

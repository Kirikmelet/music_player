use std::{hash::BuildHasher, path::PathBuf};

use anyhow::{Ok, Result};
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use directories::ProjectDirs;
use jammdb::DB;
use ratatui::{
    prelude::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tokio::sync::oneshot;

const APP_QUALIFIER: &'static str = "org";
const APP_ORGANIZATION: &'static str = "kirikmelet";
const APP_NAME: &'static str = "music_player";

pub const APP_MSG_QUIT: &'static str = "quit";

use crate::{
    config::{AppConfig, AppConfigHandler},
    db::{audio_scanner::AudioScanner, AppDB, DB_BUCKET_AUDIO},
    event::AppEvent,
};

use super::{
    file_list::{FileList, FILE_LIST_MSG_GET_DB, FILE_LIST_MSG_REFRESH_DB},
    Page, PageMsgActor, StatefulPage,
};

#[derive(Clone, Default)]
pub struct App {
    state: AppState,
    cmp_file_list: FileList,
    layout_constraints: Vec<Constraint>,
    directories: Option<ProjectDirs>,
    _test_md: String,
    db: Option<AppDB>,
    config: Option<AppConfigHandler>,
    actor: PageMsgActor,
}

pub enum AppMsg {
    State(AppState),
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
    Normal,
    DisplayHelp,
    Quit,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Normal,
            cmp_file_list: FileList::new(),
            layout_constraints: Vec::from([Constraint::Max(100), Constraint::Length(3)].as_ref()),
            directories: ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME),
            ..Default::default()
        }
    }
    pub fn init(&mut self, actor: PageMsgActor) {
        self.register_actor_details(actor.clone());
        self.cmp_file_list.init(PageMsgActor {
            id: "cmp_file_list".to_string(),
            parent_id: actor.id.clone(),
            ..actor.clone()
        });
        if let Some(dirs) = &self.directories {
            let data_path = dirs.data_local_dir();
            self.db = AppDB::new(data_path).ok();
            self.config = AppConfigHandler::new(data_path)
                .or_else(|_| Ok(AppConfigHandler::default()))
                .ok();
        }
        if let Some(ref config) = self.config {
            self._test_md = config.get_config().dir.music_dir.clone();
        }
    }
    fn get_directories(&self) -> &Option<ProjectDirs> {
        &self.directories
    }
}

#[async_trait]
impl Page for App {
    type Message = AppMsg;
    async fn update(&mut self, msg: Self::Message) -> Result<()> {
        match msg {
            AppMsg::State(state) => {
                self.state = state;
                return Ok(());
            }
        }
    }
    async fn handle_events(&mut self, event: AppEvent) -> Result<()> {
        match &event {
            AppEvent::Key(x) => match x {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.update(AppMsg::State(AppState::Quit)).await?;
                }
                KeyEvent {
                    code: KeyCode::Char('?'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if self.get_state() == AppState::DisplayHelp {
                        self.update(AppMsg::State(AppState::Normal)).await?
                    } else {
                        self.update(AppMsg::State(AppState::DisplayHelp)).await?
                    }
                }
                _ => {}
            },
            AppEvent::Msg(msg) if msg.recipient_id == self.actor.id => {
                let mut send_data: Vec<u8> = Vec::from([0]);
                let Some(tx) = &self.actor.tx else {
                    return Ok(());
                };
                if msg.msg_id == APP_MSG_QUIT {
                    self.update(AppMsg::State(AppState::Quit)).await?;
                } else if msg.msg_id == "get_project_directory" {
                    let project_dir = self
                        .get_directories()
                        .clone()
                        .and_then(|f| Some(PathBuf::from(f.data_local_dir())));
                    send_data = rmp_serde::to_vec(&project_dir)?;
                } else if msg.msg_id == FILE_LIST_MSG_REFRESH_DB {
                    let Some(ref db) = self.db else { return Ok(()) };
                    let db_threaded = db.clone();
                    let mut audio =
                        AudioScanner::new(self.config.as_ref().unwrap().get_config().clone());
                    let rt = tokio::runtime::Handle::current();
                    let audio_list = audio.scan_dir().await?;
                    let join_handle = rt.spawn_blocking(move || -> Result<()> {
                        db_threaded.refresh_audio_db(audio_list)?;
                        Ok(())
                    });
                    let _ = join_handle.await?;
                } else if msg.msg_id == FILE_LIST_MSG_GET_DB {
                    let Some(ref db) = self.db else { return Ok(()) };
                    let db_threaded = db.clone();
                    let rt = tokio::runtime::Handle::current();
                    let join_handle = rt.spawn_blocking(move || -> Result<Vec<String>> {
                        db_threaded.get_audio_db()
                    });
                    let data = join_handle.await?.unwrap_or(Vec::new());
                    send_data = rmp_serde::encode::to_vec(&data)?;
                }
                sender.send(send_data);
            }
            _ => {}
        }
        // At the end of this, all all other component event handlers
        self.cmp_file_list.handle_events(event).await?;
        Ok(())
    }
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        // Display help
        let display_help = Paragraph::new("Display help").block(Block::default());
        if self.get_state() == AppState::DisplayHelp {
            frame.render_widget(display_help, rect);
            return;
        }
        let block = Block::default().borders(Borders::ALL).title("title");
        let paragraph = Paragraph::new(self._test_md.as_str())
            .block(block)
            .alignment(Alignment::Center);
        let layout = Layout::default()
            .constraints(self.layout_constraints.as_slice())
            .split(rect);
        frame.render_widget(paragraph, layout[1]);
        self.cmp_file_list.render(frame, layout[0]);
    }
    fn register_actor_details(&mut self, rx: PageMsgActor) {
        self.actor = rx;
    }
    fn get_actor_details(&self) -> &PageMsgActor {
        &self.actor
    }
}

impl StatefulPage for App {
    type State = AppState;
    fn get_state(&self) -> Self::State {
        self.state
    }
}

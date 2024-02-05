use anyhow::Ok;
use async_trait::async_trait;
use crossterm::event::KeyCode;
use directories::ProjectDirs;
use ratatui::{
    prelude::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const APP_QUALIFIER: &'static str = "org";
const APP_ORGANIZATION: &'static str = "kirikmelet";
const APP_NAME: &'static str = "music_player";

use crate::{
    config::{AppConfig, AppConfigHandler},
    db::audio_scanner::AudioScanner,
    event::AppEvent,
};

use super::{file_list::FileList, Msg, Page, StatefulPage};

pub struct App {
    state: AppState,
    // Components
    cmp_file_list: FileList,
    layout_constraints: Vec<Constraint>,
    // App Important data
    audio_scanner: AudioScanner,
    directories: Option<ProjectDirs>,
    config: Option<AppConfigHandler>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    State(AppState),
    Quit,
    ListIncrement,
    ListDecrement,
    RefreshDb,
}

impl Msg for AppMsg {}

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
            audio_scanner: AudioScanner::new(AppConfig::default()),
            config: None,
        }
    }
    pub async fn init(&mut self) {
        if let Some(dirs) = &self.directories {
            let data_path = dirs.data_local_dir();
            self.config = AppConfigHandler::new(data_path)
                .or_else(|_| Ok(AppConfigHandler::default()))
                .ok();
        }
        if let Some(ref config) = self.config {
            self.audio_scanner.set_config(config.get_config().clone());
        }
    }
}

#[async_trait]
impl Page for App {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        // Display help
        let display_help = Paragraph::new("Display help").block(Block::default());
        if self.get_state() == AppState::DisplayHelp {
            frame.render_widget(display_help, rect);
            return;
        }
        let _block = Block::default().borders(Borders::ALL).title("title");
        let layout = Layout::default()
            .constraints(self.layout_constraints.as_slice())
            .split(rect);
        self.cmp_file_list.render(frame, layout[0]);
    }
}

#[async_trait]
impl StatefulPage for App {
    type State = AppState;
    type Message = AppMsg;
    async fn update(&mut self, msg: Self::Message) -> Option<Self::Message> {
        match msg {
            AppMsg::State(state) => {
                self.state = state;
            }
            AppMsg::ListDecrement => {
                self.cmp_file_list.prev();
            }
            AppMsg::ListIncrement => {
                self.cmp_file_list.next();
            }
            AppMsg::RefreshDb => {
                let Some(ref audio_config) = self.config else {
                    return None;
                };
                let music_dir = audio_config.get_config().dir.music_dir.clone();
                self.cmp_file_list.update_file_list(music_dir).await;
            }
            AppMsg::Quit => self.state = AppState::Quit,
        }
        None
    }
    async fn handle_events(&mut self, event: AppEvent) -> Option<Self::Message> {
        match event {
            AppEvent::Key(x) => match x {
                KeyCode::Char('q') => Some(AppMsg::Quit),
                KeyCode::Char('?') => {
                    if self.get_state() == AppState::DisplayHelp {
                        return Some(AppMsg::State(AppState::Normal));
                    }
                    Some(AppMsg::State(AppState::DisplayHelp))
                }
                KeyCode::Char('j') => Some(AppMsg::ListIncrement),
                KeyCode::Char('k') => Some(AppMsg::ListDecrement),
                KeyCode::Char('R') => Some(AppMsg::RefreshDb),
                _ => None,
            },
            AppEvent::Error => Some(AppMsg::Quit),
            _ => None,
        }
    }
    fn get_state(&self) -> Self::State {
        self.state
    }
}

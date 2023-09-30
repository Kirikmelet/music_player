use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{
        Alignment::{self, Center},
        Constraint, Layout,
    },
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{events::EventEnum, fs::temp_list_dir};

use self::base_ui::BaseUi;
pub mod base_ui;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiState {
    #[default]
    BaseUi,
}

pub trait AppUi {
    fn update(&mut self, state: Option<EventEnum>)
    where
        Self: Sized;
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>)
    where
        Self: Sized;
}

/* This struct handles the main UI state and thread!
 * TODO: Rename for conciseness */
#[derive(Debug)]
pub struct Ui<'a> {
    ui_state: UiState,
    base_ui: BaseUi<'a>,
}

impl<'a> Ui<'a> {
    pub fn new() -> Self {
        Self {
            ui_state: UiState::BaseUi,
            base_ui: BaseUi::default(),
        }
    }
}

impl<'a> AppUi for Ui<'a> {
    fn update(&mut self, state: Option<EventEnum>)
    where
        Self: Sized,
    {
        match self.ui_state {
            UiState::BaseUi => self.base_ui.update(state),
        }
    }
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>)
    where
        Self: Sized,
    {
        match self.ui_state {
            UiState::BaseUi => {
                self.base_ui.render(frame);
            }
        }
    }
}

fn error_screen<B: Backend>(frame: &mut Frame<B>, error_text: &str) {
    let block = Block::default()
        .title("!ERROR!")
        .title_alignment(Alignment::Center)
        .borders(Borders::all());
    let error_text = Paragraph::new(error_text)
        .alignment(Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(block);
    frame.render_widget(error_text, frame.size());
}

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment::Center, Constraint, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{events::EventEnum, fs::temp_list_dir};

use super::{error_screen, AppUi};
#[derive(Debug, Clone)]
pub struct BaseUi<'a> {
    list: List<'a>,
    list_items: Vec<String>,
    list_state: ListState,
}

impl<'a> Default for BaseUi<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BaseUi<'a> {
    pub fn new() -> Self {
        let items = temp_list_dir();
        let block = Block::default()
            .borders(Borders::ALL)
            .title_alignment(Center)
            .title("Hello World!");
        let list_items = items.clone();
        let items: Vec<ListItem> = items.into_iter().map(ListItem::new).collect();
        let list = List::new(items).block(block).highlight_symbol("> ");
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list,
            list_state,
            list_items,
        }
    }
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.list_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn unselect(&mut self) {
        self.list_state.select(None);
    }
}

impl<'a> AppUi for BaseUi<'a> {
    fn update(&mut self, state: Option<EventEnum>)
    where
        Self: Sized,
    {
        if let Some(EventEnum::Input(code)) = state {
            match code {
                KeyEvent {
                    code: KeyCode::Char('k'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.prev();
                }
                KeyEvent {
                    code: KeyCode::Char('j'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.next();
                }
                KeyEvent {
                    code: KeyCode::Char('u'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.unselect();
                }
                _ => {}
            }
        }
    }
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>)
    where
        Self: Sized,
    {
        if self.list_items.len() <= 0 {
            error_screen(frame, "LIST IS EMPTY!");
            return;
        }
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());
        frame.render_stateful_widget(self.list.clone(), layout[0], &mut self.list_state);
    }
}

use crate::config::Config;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
    Normal,
    Quit,
}

#[derive(Debug, Clone)]
pub struct App {
    pub state: AppState,
    pub config: Config,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: Config::new(),
            state: AppState::default(),
        }
    }
}

impl App {
    pub fn update(&mut self, new_state: AppState) {
        self.state = new_state;
    }
}

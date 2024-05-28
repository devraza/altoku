use std::error;

use ratatui::widgets::ListState;
use tui_input::Input;

use crate::api::SearchResult;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct ResultList {
    pub state: ListState,
    pub items: Vec<SearchResult>,
    pub last_selected: Option<usize>,
}

impl ResultList {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

/// Application state
pub struct App {
    /// Current value of the input box
    pub input: Input,
    /// Is the app running?
    pub running: bool,
    /// Results from the search query
    pub results: Vec<SearchResult>,
    /// Which chunk is currently selected?
    pub chunk: usize,
    /// Which list item is currently selected?
    pub list: ResultList,
    /// Is the user in an input field?
    pub editing: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            running: true,
            results: vec![],
            chunk: 1,
            list: ResultList {
                state: ListState::default(),
                items: Vec::new(),
                last_selected: None,
            },
            editing: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn go_top(&mut self) {
        self.list.state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        self.list.state.select(Some(self.list.items.len() - 1));
    }
}

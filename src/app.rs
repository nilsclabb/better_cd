use ratatui::widgets::ListState;
use std::{env, fs, path::PathBuf};

pub struct App {
    pub current_dir: PathBuf,
    pub items: Vec<PathBuf>,
    pub state: ListState,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            current_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            items: vec![],
            state: ListState::default(),
        };
        app.load_directories();
        app
    }

    pub fn load_directories(&mut self) {
        self.items.clear();
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            let mut dirs = vec![];
            let mut files = vec![];
            for entry in entries.flatten() {
                let path = entry.path();
                let meta = entry.metadata();
                if let Ok(m) = meta {
                    if m.is_dir() {
                        dirs.push(path);
                    } else {
                        files.push(path);
                    }
                }
            }
            dirs.sort();
            files.sort();
            self.items.extend(dirs);
            self.items.extend(files);
        }
        
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    self.items.len().saturating_sub(1)
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn go_to_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.load_directories();
        }
    }

    pub fn enter_directory(&mut self) -> bool {
        if let Some(selected) = self.state.selected() {
            if let Some(path) = self.items.get(selected) {
                if path.is_dir() {
                    self.current_dir = path.clone();
                    self.load_directories();
                    return true;
                }
            }
        }
        false
    }
    
    pub fn get_selected_path(&mut self) -> String {
        self.current_dir.to_string_lossy().to_string()
    }
}

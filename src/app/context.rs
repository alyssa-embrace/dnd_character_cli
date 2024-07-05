use ratatui::widgets::ListState;
use std::{collections::HashMap, fs};

#[derive(Clone)]
pub struct Context {
    pub settings: HashMap<String, String>,
    pub char_list: FileList,
    pub attack_list: FileList,
}

#[derive(Clone)]
pub struct FileList {
    pub state: ListState,
    pub files: Vec<String>,
}

impl FileList {
    pub fn new () -> Self {
        let mut char_list = FileList {
            state: ListState::default(),
            files: vec![
                    "/mnt/c/Users/username/characters".to_string(),
                    "/mnt/c/Users/username/encounters".to_string(),
                    "/mnt/c/Users/username/maps".to_string(),
                ],
        };
        char_list.state.select_first();
        char_list
    }

    pub fn add_directory(&mut self, path: &str) {
        if self.validate_directory(path) {
            self.files.push(path.to_string());
        }
    }

    fn validate_directory(&self, path: &str) -> bool {
        fs::metadata(path).is_ok() && fs::metadata(path).unwrap().is_dir()
    }

    pub fn next(&mut self) {
        self.state.select(Some((self.state.selected().unwrap_or(0) + 1).min(self.files.len() - 1)));
    }

    pub fn previous(&mut self) {
        self.state.select(Some(self.state.selected().unwrap_or(0).saturating_sub(1)));
    }

    pub fn get_selected_character_file(self) -> Option<String> {
        self.state.selected().map(|i| self.files[i].clone())
    }
}

impl Context {
    pub fn new(settings: HashMap<String, String>) -> Self {
        Context {
            char_list: FileList::new(),
            attack_list: FileList::new(),
            settings: settings,
        }
    }
}
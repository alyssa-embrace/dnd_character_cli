use ratatui::widgets::ListState;
use std::{collections::HashMap, fs};

#[derive(Clone)]
pub struct Context {
    pub settings: HashMap<String, String>,
    pub char_list: FileList,
    pub ability_list: FileList,
}

impl Context {
    pub fn new(settings: HashMap<String, String>) -> Self {
        let char_src = settings.get("character_source_directory").unwrap().clone();
        let ability_src = settings.get("ability_source_directory").unwrap().clone();
        let context = Context {
            char_list: FileList::new(char_src),
            ability_list: FileList::new(ability_src),
            settings: settings,
        };
        context
    }

    // pub fn get_character_src(&self) -> Option<&String> {
    //     self.settings.get("character_source_directory")
    // }

    // pub fn get_ability_src(&self) -> Option<&String> {
    //     self.settings.get("ability_source_directory")
    // }
}

#[derive(Clone)]
pub struct FileList {
    pub state: ListState,
    pub files: Vec<String>,
}

impl FileList {
    pub fn new (src: String) -> Self {
        let mut char_list = FileList {
            state: ListState::default(),
            files: vec![],
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
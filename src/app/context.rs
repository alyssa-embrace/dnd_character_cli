use ratatui::widgets::ListState;
use std::fs;

#[derive(Clone)]
pub struct Context {
    pub dir_list: CharacterList,
}

#[derive(Clone)]
pub struct CharacterList {
    pub state: ListState,
    pub directories: Vec<String>,
}

impl CharacterList {
    pub fn new () -> Self {
        let mut dir_list = CharacterList {
            state: ListState::default(),
            directories: vec![
                    "/mnt/c/Users/username/characters".to_string(),
                    "/mnt/c/Users/username/encounters".to_string(),
                    "/mnt/c/Users/username/maps".to_string(),
                ],
        };
        dir_list.state.select_first();
        dir_list
    }

    pub fn add_directory(&mut self, path: &str) {
        if self.validate_directory(path) {
            self.directories.push(path.to_string());
        }
    }

    fn validate_directory(&self, path: &str) -> bool {
        fs::metadata(path).is_ok() && fs::metadata(path).unwrap().is_dir()
    }

    pub fn next(&mut self) {
        self.state.select(Some((self.state.selected().unwrap_or(0) + 1).min(self.directories.len() - 1)));
    }

    pub fn previous(&mut self) {
        self.state.select(Some(self.state.selected().unwrap_or(0).saturating_sub(1)));
    }

    pub fn get_selected_character_file(self) -> Option<String> {
        self.state.selected().map(|i| self.directories[i].clone())
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            dir_list: CharacterList::new(),
        }
    }
}
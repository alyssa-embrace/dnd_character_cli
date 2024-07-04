use ratatui::widgets::ListState;
use std::fs;

#[derive(Clone)]
pub struct Context {
    pub dir_list: DirectoryList,
    pub esc_handler: Option<Fn() -> ()>,
}

#[derive(Clone)]
pub struct DirectoryList {
    pub state: ListState,
    pub directories: Vec<String>,
    pub last_selected: Option<usize>,
}

impl DirectoryList {
    pub fn add_directory(&mut self, path: &str) {
        if self.validate_directory(path) {
            self.directories.push(path.to_string());
        }
    }

    fn validate_directory(&self, path: &str) -> bool {
        fs::metadata(path).is_ok() && fs::metadata(path).unwrap().is_dir()
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            dir_list: DirectoryList {
                state: ListState::default(),
                directories: vec![
                    "/mnt/c/Users/username/characters".to_string(),
                    "/mnt/c/Users/username/encounters".to_string(),
                    "/mnt/c/Users/username/maps".to_string(),
                ],
                last_selected: None,
            },
        }
    }
}
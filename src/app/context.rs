use ratatui::widgets::ListState;

#[derive(Clone)]
pub struct Context {
    pub dir_list: DirectoryList,
}

#[derive(Clone)]
pub struct DirectoryList {
    pub state: ListState,
    pub directories: Vec<String>,
    pub last_selected: Option<usize>,
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
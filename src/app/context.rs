#[derive(Clone)]
pub struct Context {
    pub src_directories: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            src_directories: vec![],
        }
    }
}
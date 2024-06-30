use std::path::Path;
use crate::models::cli::DeleteCharArgs;

pub fn handle(args: &DeleteCharArgs) {
    if Path::new(args.path.as_str()).exists() {
        std::fs::remove_file(args.path.as_str()).unwrap();
    } else {
        // TODO: Replace with error handling
        println!("File not found");
    }
}
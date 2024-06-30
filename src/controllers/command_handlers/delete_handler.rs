use std::path::Path;
use crate::models::cli::DeleteArgs;

pub fn handle(args: &DeleteArgs) {
    let target_path = Path::new(args.path.as_str());
    if target_path.exists() && target_path.extension().unwrap() == "toml" {
        std::fs::remove_file(args.path.as_str()).unwrap();
    } else if target_path.extension().unwrap() != "toml" {
        // TODO: Replace with error handling
        println!("Target file is not a TOML file");
    } else {
        // TODO: Replace with error handling
        println!("File not found");
    }
}
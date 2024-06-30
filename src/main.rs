use clap::Parser;
use controllers::command_handlers;
use models::cli::{CliArgs, Command};

mod views;
mod controllers;
mod models;

fn main() {
    let args: CliArgs = CliArgs::parse();

    match args.command {
        Command::CreateCharacter(create_args) => 
            command_handlers::create_character_handler::handle(&create_args),
        Command::ModifyCharacter(modify_args) => 
            command_handlers::modify_character_handler::handle(&modify_args),
        Command::DeleteCharacter(delete_args) =>
            command_handlers::delete_character_handler::handle(&delete_args),
        Command::CreateAttack => println!("TODO: Create attack"),
        Command::ModifyAttack => println!("TODO: Modify attack"),
        Command::DeleteAttack => println!("TODO: Delete attack"),
    }
}
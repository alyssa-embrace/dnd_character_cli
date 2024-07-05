use app::context::Context;
use clap::Parser;
use controllers::command_handlers;
use models::cli::{CliArgs, CliCommand};
use crate::app::App;

mod views;
mod controllers;
mod models;
mod app;


fn main() -> color_eyre::Result<()> {
    views::error_hooks::install_hooks()?;
    let mut terminal = views::tui_setup::init()?;
    let mut context = Context::new();
    let mut app = App::default(&mut context);
    app.run(&mut terminal)?;
    
    /*
     What's the ideal shape of the app?
     Let's think about this for a bit.
        - Character Wizard
        - Initiative Wizard
     The Character Wizard is a TUI implementation of the character creation process.
     And ideally lets you create new ones on the fly.
     So that multi-execution isn't necessary.
     */

    views::tui_setup::restore()?;

    let args: CliArgs = CliArgs::parse();

    match args.command {
        CliCommand::CreateCharacter(create_args) => 
            command_handlers::create_character_handler::handle(&create_args),
        CliCommand::ModifyCharacter(modify_args) => 
            command_handlers::modify_character_handler::handle(&modify_args),
        CliCommand::DeleteCharacter(delete_args) =>
            command_handlers::delete_handler::handle(&delete_args),
        CliCommand::CreateAttack(create_args) => 
            command_handlers::create_attack_handler::handle(&create_args),
        CliCommand::ModifyAttack(modify_args) => 
            command_handlers::modify_attack_handler::handle(&modify_args),
        CliCommand::DeleteAttack(delete_args) => 
            command_handlers::delete_handler::handle(&delete_args),
    }

    Ok(())
}
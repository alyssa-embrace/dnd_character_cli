use app::{context::Context, AppMode};
use clap::Parser;
use controllers::command_handlers;
use models::cli::{CliArgs, CliCommand};
use crate::app::App;

mod views;
mod controllers;
mod models;
mod app;


fn main() -> color_eyre::Result<()> {    
    /*
     What's the ideal shape of the app?
     Let's think about this for a bit.
        - Character Wizard
        - Initiative Wizard
     The Character Wizard is a TUI implementation of the character creation process.
     And ideally lets you create new ones on the fly.
     So that multi-execution isn't necessary.
     */

    let args: CliArgs = CliArgs::parse();

    match args.command {
        CliCommand::EditSrcConfig => todo!(),
        CliCommand::CharacterWizard => run_app(AppMode::CharacterWizard)?,
        CliCommand::InitiativeWizard => run_app(AppMode::InitiativeWizard)?,
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

fn run_app(app_mode: AppMode) -> color_eyre::Result<()>{
    views::error_hooks::install_hooks()?;
    let mut terminal = views::tui_setup::init()?;
    let mut context = Context::new();
    let mut app = App::default(&mut context);
    app.app_mode = app_mode;
    app.run(&mut terminal)?;
    views::tui_setup::restore()?;
    Ok(())
}
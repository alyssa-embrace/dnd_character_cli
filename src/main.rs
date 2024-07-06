use std::collections::HashMap;

use app::{context::Context, AppMode};
use clap::Parser;
use config::Config;
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

    let settings = Config::builder()
        .add_source(config::File::with_name("dnd_tui_config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    let settings = settings.try_deserialize::<HashMap<String, String>>()?;

    let args: CliArgs = CliArgs::parse();

    match args.command {
        CliCommand::EditSrcConfig => todo!(),
        CliCommand::CharacterWizard => run_app(AppMode::CharacterWizard, settings)?,
        CliCommand::InitiativeWizard => run_app(AppMode::InitiativeWizard, settings)?,
        CliCommand::CreateCharacter(create_args) => 
            command_handlers::create_character_handler::handle(&create_args),
        CliCommand::ModifyCharacter(modify_args) => 
            command_handlers::modify_character_handler::handle(&modify_args),
        CliCommand::DeleteCharacter(delete_args) =>
            command_handlers::delete_handler::handle(&delete_args),
        CliCommand::CreateAbility(create_args) => 
            command_handlers::create_ability_handler::handle(&create_args),
        CliCommand::ModifyAbility(modify_args) => 
            command_handlers::modify_ability_handler::handle(&modify_args),
        CliCommand::DeleteAbility(delete_args) => 
            command_handlers::delete_handler::handle(&delete_args),
    }

    Ok(())
}

fn run_app(app_mode: AppMode, settings: HashMap<String, String>) -> color_eyre::Result<()>{
    views::error_hooks::install_hooks()?;
    let mut terminal = views::tui_setup::init()?;
    let mut context = Context::new(settings);
    let mut app = App::default(&mut context);
    app.app_mode = app_mode;
    app.run(&mut terminal)?;
    views::tui_setup::restore()?;
    Ok(())
}
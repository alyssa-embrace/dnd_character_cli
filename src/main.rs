use clap::{Args, Parser, Subcommand};
use controllers::command_handlers;

mod views;
mod controllers;
mod models;

#[derive(Parser, Debug)]
#[command(name = "dnd-cli")]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The command to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    CreateCharacter(CreateArgs),
    ModifyCharacter,
    DeleteCharacter(DeleteArgs),
    CreateAttack,
    ModifyAttack,
    DeleteAttack,
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(short, long)]
    path: String,
    
    #[arg(short, long)]
    overwrite: bool,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args: CliArgs = CliArgs::parse();

    match args.command {
        Command::CreateCharacter(create_args) => 
            command_handlers::handle_create(&create_args),
        Command::ModifyCharacter => println!("TODO: Modify character"),
        Command::DeleteCharacter(delete_args) =>
            command_handlers::handle_delete(&delete_args),
        Command::CreateAttack => println!("TODO: Create attack"),
        Command::ModifyAttack => println!("TODO: Modify attack"),
        Command::DeleteAttack => println!("TODO: Delete attack"),
    }
}
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
    Create(CreateArgs),
    Delete(DeleteArgs),
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
        Command::Create(create_args) => 
            command_handlers::handle_create(&create_args),
        Command::Delete(delete_args) =>
            command_handlers::handle_delete(&delete_args),
    }
}
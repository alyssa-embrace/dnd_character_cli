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
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let args: CliArgs = CliArgs::parse();

    match args.command {
        Command::Create(create_args) => 
            command_handlers::handle_create(&create_args),
    }
}
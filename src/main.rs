use clap::Parser;
use controllers::command_handlers;

mod views;
mod controllers;
mod models;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The command to run
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    views::hello();
    command_handlers::handle_cli_command(&args.command);
}
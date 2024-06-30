use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dnd-cli")]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The command to run
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    CreateCharacter(CreateCharArgs),
    ModifyCharacter(ModifyCharArgs),
    DeleteCharacter(DeleteCharArgs),
    CreateAttack,
    ModifyAttack,
    DeleteAttack,
}

#[derive(Args, Debug)]
pub struct CreateCharArgs {
    #[arg(short, long)]
    pub path: String,
    
    #[arg(short, long)]
    pub overwrite: bool,
}

#[derive(Args, Debug)]
pub struct ModifyCharArgs { 
    #[command(subcommand)]
    pub command: ModifyCommands,
    
    #[arg(short, long)]
    pub path: String
}

#[derive(Args, Debug)]
pub struct DeleteCharArgs {
    #[arg(short, long)]
    pub path: String,
}

#[derive(Subcommand, Debug)]
pub enum ModifyCommands {
    AbilityScores(ModifyVectorU8),
    ProficiencyBonus(ModifyU8),
    AddProficiency(StringArg),
    RmProficiency(StringArg),
    Hitpoints(ModifyU16),
    ArmorClass(ModifyU8),
    Speed(ModifyU8),
    Initiative(ModifyI8Args),
    Description,
    AddAttack(StringArg),
    RmAttack(StringArg),
}

#[derive(Args, Debug)]
pub struct ModifyVectorU8 {
    pub new_values: Vec<u8>,
}

#[derive(Args, Debug)]
pub struct ModifyU8 {
    pub new_value: u8,
}

#[derive(Args, Debug)]
pub struct ModifyU16 {
    pub new_value: u16,
}

#[derive(Args, Debug)]
pub struct ModifyI8Args {
    pub new_value: i8,
}

#[derive(Args, Debug)]
pub struct StringArg {
    pub value: String,
}
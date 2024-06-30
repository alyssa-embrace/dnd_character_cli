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
    CreateCharacter(CreateArgs),
    ModifyCharacter(ModifyCharArgs),
    DeleteCharacter(DeleteArgs),
    CreateAttack(CreateArgs),
    ModifyAttack(ModifyAttackArgs),
    DeleteAttack(DeleteArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    #[arg(short, long)]
    pub path: String,
    
    #[arg(short, long)]
    pub overwrite: bool,
}

#[derive(Args, Debug)]
pub struct ModifyCharArgs {  
    #[arg(short, long)]
    pub path: String,

    #[arg(short('P'), long)]
    pub proficiency_bonus: Option<u8>,

    #[arg(short('H'), long)]
    pub hitpoints: Option<u16>,

    #[arg(short, long)]
    pub armor_class: Option<u8>,

    #[arg(short, long)]
    pub speed: Option<u8>,

    #[arg(short, long("init"))]
    pub initiative: Option<i8>,
    
    #[arg(short('I'), long("desc"))]
    pub description: bool,

    #[arg(short('j'), long)]
    pub attacks: bool,

    #[arg(short('k'), long("prof"))]
    pub proficiencies: bool,

    #[arg(short('A'), long)]
    pub ability_scores:bool
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[arg(short, long)]
    pub path: String,
}

#[derive(Args, Debug)]
pub struct ModifyAttackArgs {
    #[arg(short, long)]
    pub path: String,
    
    #[arg(short, long)]
    pub name: Option<String>,
    
    #[arg(short('I'), long("desc"))]
    pub description: bool,
    
    #[arg(short, long)]
    pub attack_bonus: Option<i8>,
    
    #[arg(short, long)]
    pub damage_bonus: Option<i8>,
    
    #[arg(short('D'), long)]
    pub damage_dice: bool,
   
    #[arg(short, long)]
    pub count: Option<u8>,
}
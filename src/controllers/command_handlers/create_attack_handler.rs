use std::{fs::File, path::Path, io::Write};
use toml;
use crate::{models::attack::Attack, models::cli::CreateArgs, views::inputs};

pub fn handle(args: &CreateArgs){
    if Path::new(args.path.as_str()).exists() && !args.overwrite {
        // TODO: Replace with error handling
        println!("File already exists. Did you mean to enable overwrite?");
        return;
    } else {
        flush_to_file(
            create_attack(), 
            &args.path.as_str()
        );
    }
}

fn create_attack() -> Attack {
    let name = inputs::show_name_dialog();
    let description = inputs::show_description_editor();
    let attack_bonus = inputs::show_attack_bonus_dialog();
    let damage_bonus = inputs::show_damage_bonus_dialog();
    let damage_dice = inputs::show_damage_dice_dialog();
    let count = inputs::show_attack_count_dialog();

    Attack::new(name, description, attack_bonus, damage_bonus, damage_dice, count)
}

fn flush_to_file(attack: Attack, path: &str) {
    let serialized = toml::to_string_pretty(&attack).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
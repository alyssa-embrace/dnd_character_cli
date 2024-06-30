use std::{
    fs::File, 
    path::Path, 
    io::Write
};
use toml;
use crate::{
    models::{
        character::Character, 
        statistics::AbilityScore, 
        cli::CreateArgs
    },
    views::inputs,
};

pub fn handle(args: &CreateArgs){
    if Path::new(args.path.as_str()).exists() && !args.overwrite {
        // TODO: Replace with error handling
        println!("File already exists. Did you mean to enable overwrite?");
        return;
    } else {
        flush_to_file(
            create_character(), 
            &args.path.as_str()
        );
    }
}

fn create_character() -> Character {
    let name = inputs::show_name_dialog();
    let ability_scores = [
        inputs::show_ability_score_dialog(AbilityScore::Strength),
        inputs::show_ability_score_dialog(AbilityScore::Dexterity),
        inputs::show_ability_score_dialog(AbilityScore::Constitution),
        inputs::show_ability_score_dialog(AbilityScore::Intelligence),
        inputs::show_ability_score_dialog(AbilityScore::Wisdom),
        inputs::show_ability_score_dialog(AbilityScore::Charisma),
    ];

    let proficiency_bonus = inputs::show_proficiency_bonus_dialog();
    let hitpoints = inputs::show_hitpoints_dialog();
    let armor_class = inputs::show_armor_class_dialog();
    let speed = inputs::show_speed_dialog();
    let initiative_bonus = inputs::show_initiative_dialog();

    let description = inputs::show_description_editor();

    let proficiencies: Vec<String> = inputs::show_proficiency_dialog();
    let attacks: Vec<String> = Vec::new(); // TODO: Implement attack component of character creation

    Character::new(name, description, ability_scores, proficiency_bonus, 
        hitpoints, armor_class, speed, initiative_bonus, proficiencies, attacks)
}

fn flush_to_file(character: Character, path: &str) {
    let serialized = toml::to_string_pretty(&character).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
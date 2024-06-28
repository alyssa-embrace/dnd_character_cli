use std::fs::File;
use std::path::Path;
use std::io::Write;
use toml;
use crate::CreateArgs;
use crate::models::attack::Attack;
use crate::models::character::Character;
use crate::models::statistics::AbilityScore;
use crate::views::character_inputs;
use crate::DeleteArgs;

pub fn handle_create(args: &CreateArgs){
    if Path::new(args.path.as_str()).exists() && !args.overwrite {
        println!("File already exists. Did you mean to enable overwrite?");
        return;
    } else {
        let character = create_character_from_user_input();
        write_character_to_file(character, &args.path.as_str());
    }
}

fn create_character_from_user_input() -> Character {
    let name = character_inputs::show_name_dialog();
    let ability_scores = [
        character_inputs::show_ability_score_dialog(AbilityScore::Strength),
        character_inputs::show_ability_score_dialog(AbilityScore::Dexterity),
        character_inputs::show_ability_score_dialog(AbilityScore::Constitution),
        character_inputs::show_ability_score_dialog(AbilityScore::Intelligence),
        character_inputs::show_ability_score_dialog(AbilityScore::Wisdom),
        character_inputs::show_ability_score_dialog(AbilityScore::Charisma),
    ];

    let proficiency_bonus = character_inputs::show_proficiency_dialog();
    let hitpoints = character_inputs::show_hitpoints_dialog();
    let armor_class = character_inputs::show_armor_class_dialog();
    let speed = character_inputs::show_speed_dialog();
    let initiative_bonus = character_inputs::show_initiative_dialog();

    let description = character_inputs::show_description_editor();

    let proficiencies: Vec<String> = Vec::new();
    let attacks: Vec<Attack> = Vec::new();

    Character::new(name, description, ability_scores, proficiency_bonus, 
        hitpoints, armor_class, speed, initiative_bonus, proficiencies, attacks)
}

fn write_character_to_file(character: Character, path: &str) {
    let serialized = toml::to_string_pretty(&character).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

pub fn handle_delete(args: &DeleteArgs) {
    if Path::new(args.path.as_str()).exists() {
        std::fs::remove_file(args.path.as_str()).unwrap();
    } else {
        println!("File not found");
    }
    
}
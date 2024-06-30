use dialoguer::{Editor, MultiSelect, Input, InputValidator};
use crate::models::statistics::AbilityScore;
use crate::views::validators;

fn show_input_dialog<T: InputValidator<String, Err = String>> (prompt: &str, initial_text: &str, validator: T) -> String {
    Input::new()
        .with_prompt(prompt)
        .allow_empty(false)
        .with_initial_text(initial_text)
        .validate_with(validator)
        .interact_text()
        .unwrap_or(initial_text.to_string())
}

pub fn show_ability_score_dialog(ability_score: AbilityScore) -> u8 {
    let prompt: &str;
    match ability_score {
        AbilityScore::Strength => prompt = "Please enter a strength score",
        AbilityScore::Dexterity => prompt = "Please enter a dexterity score",
        AbilityScore::Constitution => prompt = "Please enter a constitution score",
        AbilityScore::Intelligence => prompt = "Please enter an intelligence score",
        AbilityScore::Wisdom => prompt = "Please enter a wisdom score",
        AbilityScore::Charisma => prompt = "Please enter a charisma score",
    }

    let input = show_input_dialog(prompt, 
        "10", validators::AbilityScoreValidator{});
    input.parse::<u8>().unwrap_or(10)
}

pub fn show_proficiency_bonus_dialog() -> u8 {
    let input = show_input_dialog("Please enter a proficiency bonus",
        "2", validators::ProficiencyValidator{});
    input.parse::<u8>().unwrap_or(2)
}

pub fn show_hitpoints_dialog() -> u16 {
    let input = show_input_dialog("Please enter a hit point total", 
        "10", validators::HitpointsValidator{});
    input.parse::<u16>().unwrap_or(10)
}

pub fn show_armor_class_dialog() -> u8 {
    let input = show_input_dialog("Please enter an armor class",
        "10", validators::ArmorClassValidator{});
    input.parse::<u8>().unwrap_or(10)
}

pub fn show_speed_dialog() -> u8 {
    let input = show_input_dialog("Please enter a speed", 
        "30", validators::SpeedValidator{});
    input.parse::<u8>().unwrap_or(30)
}

pub fn show_initiative_dialog() -> i8 {
    let input = show_input_dialog("Please enter an initiative bonus", 
        "0", validators::InitiativeValidator{});
    input.parse::<i8>().unwrap_or(0)
}

pub fn show_proficiency_dialog() -> Vec<String> {
    let items = vec!["Strength", "Dexterity", "Constitution", 
        "Intelligence", "Wisdom", "Charisma", "Acrobatics", "Animal Handling", "Arcana", "Athletics", 
        "Deception", "History", "Insight", "Intimidation", "Investigation", 
        "Medicine", "Nature", "Perception", "Performance", "Persuasion", 
        "Religion", "Sleight of Hand", "Stealth", "Survival", "Martial", "Simple",
        "Light", "Medium", "Heavy", "Shields"];
    let selection = MultiSelect::new()
        .with_prompt("Please select proficiencies (Space to select, Enter to confirm)")
        .items(&items)
        .interact()
        .unwrap();
    let mut proficiencies: Vec<String> = Vec::new();
    for i in selection.iter() {
        proficiencies.push(items[*i].to_string());
    }
    proficiencies
}

pub fn show_attack_bonus_dialog() -> i8 {
    let input = show_input_dialog("Please enter an attack bonus", 
        "0", validators::AttackBonusValidator{});
    input.parse::<i8>().unwrap_or(0)
}

pub fn show_damage_bonus_dialog() -> i8 {
    let input = show_input_dialog("Please enter a damage bonus", 
        "0", validators::DamageBonusValidator{});
    input.parse::<i8>().unwrap_or(0)
}

pub fn show_name_dialog() -> String {
    Input::new()
        .with_prompt("Please enter a name")
        .allow_empty(false)
        .with_initial_text("Unnamed")
        .interact_text()
        .unwrap_or("Unnamed".to_string())
}

pub fn show_damage_dice_dialog() -> Vec<u8> {
    let mut damage_dice = Vec::<u8>::new();
    
    loop {
        let input = show_input_dialog("Please enter a die size (e.g. 4 or 6); q to quit",
            "6", validators::DamageDieValidator{});
        if input.contains("q") {
            break;
        }   
        let die_size = input.parse::<u8>().unwrap_or(6); // Not an ideal default here
        damage_dice.push(die_size);
    }

    damage_dice
}

pub fn show_attack_count_dialog() -> u8 {
    let input = show_input_dialog("Please enter a count", 
        "1", validators::CountValidator{});
    input.parse::<u8>().unwrap_or(1)
}

pub fn show_description_editor() -> String {
    if let Some(description) = 
        Editor::new().edit("Please enter a description").unwrap() {
        description
    } else {
        String::from("No description provided")
    }
}
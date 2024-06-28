use dialoguer::{Editor, Input, InputValidator};
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

pub fn show_proficiency_dialog() -> u8 {
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

pub fn show_name_dialog() -> String {
    Input::new()
        .with_prompt("Please enter a name")
        .allow_empty(false)
        .with_initial_text("Unnamed Character")
        .interact_text()
        .unwrap_or("Unnamed Character".to_string())
}

pub fn show_description_editor() -> String {
    if let Some(description) = 
        Editor::new().edit("Please enter a description").unwrap() {
        description
    } else {
        String::from("No description provided")
    }
}
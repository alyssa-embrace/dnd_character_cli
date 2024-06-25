//use std::num::ParseIntError;

use dialoguer::{Editor, Input};
use crate::models::statistics::AbilityScore;

pub fn show_ability_score_dialog(ability_score: AbilityScore) -> i8 {
    let prompt: &str;
    match ability_score {
        AbilityScore::Strength => prompt = "Please enter a strength score for your character",
        AbilityScore::Dexterity => prompt = "Please enter a dexterity score for your character",
        AbilityScore::Constitution => prompt = "Please enter a constitution score for your character",
        AbilityScore::Intelligence => prompt = "Please enter an intelligence score for your character",
        AbilityScore::Wisdom => prompt = "Please enter a wisdom score for your character",
        AbilityScore::Charisma => prompt = "Please enter a charisma score for your character",
    }

    let input: String = Input::new()
        .with_prompt(prompt)
        .allow_empty(false)
        .validate_with(|input: &String| {
            let result = input.parse::<i8>();
    
            if result.is_err() {
                return Err("Ability score must be a number");
            }
            else if !(1..=20).contains(&result.unwrap()) {
                return Err("Ability score must be between 1 and 20");
            }
    
            Ok(())
        })
        .interact_text()
        .unwrap();
    input.parse::<i8>().unwrap_or(10)
}

pub fn request_description() -> String {
    // prototyping an editor
    if let Some(description) = 
        Editor::new().edit("Please enter a description for your character").unwrap() {
        description
    } else {
        String::from("No description provided")
    }
}
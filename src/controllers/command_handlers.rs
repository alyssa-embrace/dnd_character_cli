use dialoguer::Editor;
use crate::models::statistics::AbilityScore;
use crate::views::character_inputs;

pub fn handle_cli_command(command: &str) {
    match command {
        "create" => {
            handle_create();
        }
        "edit" => {
            println!("Editing an existing character");
        }
        "delete" => {
            println!("Deleting an existing character");
        }
        "list" => {
            println!("Listing all characters");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn handle_create() {
    character_inputs::show_ability_score_dialog(AbilityScore::Strength);
    character_inputs::show_ability_score_dialog(AbilityScore::Dexterity);
    character_inputs::show_ability_score_dialog(AbilityScore::Constitution);
    character_inputs::show_ability_score_dialog(AbilityScore::Intelligence);
    character_inputs::show_ability_score_dialog(AbilityScore::Wisdom);
    character_inputs::show_ability_score_dialog(AbilityScore::Charisma);

    // prototyping an editor
    if let Some(description) = Editor::new().edit("Please enter a description for your character").unwrap() {
        println!("Your description: {}", description);
    } else {
        println!("No description provided");
    }

}
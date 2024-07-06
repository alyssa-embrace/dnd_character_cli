use crate::{
    views::inputs,
    models::{
        character::Character,
        cli::ModifyCharArgs,
        statistics::AbilityScore,
    },
    command_handlers::serde_utils::{
        construct_from_file,
        flush_to_file,
    },
};

pub fn handle(args: &ModifyCharArgs) {
    match construct_from_file::<Character>(&args.path) {
        Ok(character) => {
            match flush_to_file(
                modify(&character, &args), 
                &args.path
            ) {
                Ok(_) => {
                    // There should be a view output function wired here
                    println!("Character modified successfully");
                },
                Err(e) => {
                    // There should be a view output function wired here
                    println!("Error writing modified character to file {:?}: {:?}", &args.path, e);
                }
            }
        },
        Err(e) => {
            // There should be a view output function wired here
            println!("Error Constructing Character from given file {:?}: {:?}", &args.path, e);
            return;
        }
    }
}

fn modify (character: &Character, args: &ModifyCharArgs) -> Character {
    let mut new_character = character.clone();

    if let Some(proficiency_bonus) = args.proficiency_bonus {
        new_character.proficiency_bonus = proficiency_bonus;
    }
    if let Some(hitpoints) = args.hitpoints {
        new_character.hit_points = hitpoints;
    }
    if let Some(armor_class) = args.armor_class {
        new_character.armor_class = armor_class;
    }
    if let Some(speed) = args.speed {
        new_character.speed = speed;
    }
    if let Some(initiative) = args.initiative {
        new_character.initiative_bonus = initiative;
    }
    if args.description {
        new_character.description = inputs::show_description_editor();
    }
    if args.abilities {
        // TODO: Implement attack modification
        new_character.abilities = vec![];
    }
    if args.proficiencies {
        new_character.proficiencies = inputs::show_proficiency_dialog();
    }
    if args.ability_scores {
        new_character.ability_scores = [
            inputs::show_ability_score_dialog(AbilityScore::Strength),
            inputs::show_ability_score_dialog(AbilityScore::Dexterity),
            inputs::show_ability_score_dialog(AbilityScore::Constitution),
            inputs::show_ability_score_dialog(AbilityScore::Intelligence),
            inputs::show_ability_score_dialog(AbilityScore::Wisdom),
            inputs::show_ability_score_dialog(AbilityScore::Charisma),
        ];
    }

    new_character
}
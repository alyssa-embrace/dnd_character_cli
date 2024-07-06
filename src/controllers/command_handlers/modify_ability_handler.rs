use crate::{
    command_handlers::{errors::ModifyOpError, serde_utils::construct_from_file},
    models::{ability::Ability, cli::ModifyAbilityArgs},
    views::inputs,
};

pub fn handle(args: &ModifyAbilityArgs) {
    match construct_from_file::<Ability>(&args.path) {
        Ok(ability) => {
            match modify(&ability, &args) {
                Ok(modified_ability) => {
                    match super::serde_utils::flush_to_file(modified_ability, &args.path) {
                        Ok(_) => {
                            println!("Ability modified successfully");
                        },
                        Err(e) => {
                            println!("Error writing modified ability to file {:?}: {:?}", &args.path, e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error modifying ability: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Error Constructing Ability from given file {:?}: {:?}", &args.path, e);
            return;
        }
    }
}

fn modify(ability: &Ability, args: &ModifyAbilityArgs) -> Result<Ability, ModifyOpError> {
    let mut new_ability = ability.clone();
    
    if let Some(new_name) = &args.name {
        new_ability.name = new_name.clone();
    }
    if let Some(new_attack_bonus) = &args.attack_bonus {
        new_ability.attack_bonus = *new_attack_bonus;
    }
    if let Some(new_damage_bonus) = &args.damage_bonus {
        new_ability.damage_bonus = *new_damage_bonus;
    }
    if let Some(new_count) = &args.count {
        new_ability.count = *new_count;
    }
    if args.description {
        new_ability.description = inputs::show_description_editor();
    }
    if args.damage_dice {
        new_ability.damage_dice = inputs::show_damage_dice_dialog();
    }

    Ok(new_ability)
}
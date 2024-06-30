use crate::{
    command_handlers::{errors::ModifyOpError, serde_utils::construct_from_file},
    models::{attack::Attack, cli::ModifyAttackArgs},
    views::inputs,
};

pub fn handle(args: &ModifyAttackArgs) {
    match construct_from_file::<Attack>(&args.path) {
        Ok(attack) => {
            match modify(&attack, &args) {
                Ok(modified_attack) => {
                    match super::serde_utils::flush_to_file(modified_attack, &args.path) {
                        Ok(_) => {
                            println!("Attack modified successfully");
                        },
                        Err(e) => {
                            println!("Error writing modified attack to file {:?}: {:?}", &args.path, e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error modifying attack: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Error Constructing Attack from given file {:?}: {:?}", &args.path, e);
            return;
        }
    }
}

fn modify(attack: &Attack, args: &ModifyAttackArgs) -> Result<Attack, ModifyOpError> {
    let mut new_attack = attack.clone();
    
    if let Some(new_name) = &args.name {
        new_attack.name = new_name.clone();
    }
    if let Some(new_attack_bonus) = &args.attack_bonus {
        new_attack.attack_bonus = *new_attack_bonus;
    }
    if let Some(new_damage_bonus) = &args.damage_bonus {
        new_attack.damage_bonus = *new_damage_bonus;
    }
    if let Some(new_count) = &args.count {
        new_attack.count = *new_count;
    }
    if args.description {
        new_attack.description = inputs::show_description_editor();
    }
    if args.damage_dice {
        new_attack.damage_dice = inputs::show_damage_dice_dialog();
    }

    Ok(new_attack)
}
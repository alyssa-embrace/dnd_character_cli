use std::io::Write;

use crate::models::character::Character;
use crate::models::cli::{ModifyCharArgs, ModifyCommands, ModifyU8};

#[derive(Debug)]
enum ModifyCharOpError {
    CouldNotFindFile(std::io::Error),
    CouldNotCreateFile(std::io::Error),
    CouldNotWriteFile(std::io::Error),
    Deserialization(toml::de::Error),
    Serialization(toml::ser::Error),
}

pub fn handle(args: &ModifyCharArgs) {
    match construct_character_from_file(&args.path) {
        Ok(character) => {
            match write_character_to_file(
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

fn modify(character: &Character, args: &ModifyCharArgs) -> Character {    
    match &args.command {
        ModifyCommands::AbilityScores(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                if a.new_values.len() != 6 {
                    // Reject changes that don't have 6 ability scores
                    // TODO: We should throw an error here
                    return c.clone();
                }
                let mut new_character: Character = c.clone();
                for index in 0..a.new_values.len() {
                    new_character.ability_scores[index] = a.new_values[index];
                }
                new_character
            }),
        ModifyCommands::ProficiencyBonus(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.proficiency_bonus = a.new_value;
                new_character
            }),
        ModifyCommands::AddProficiency(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.proficiencies.push(a.value.clone());
                new_character
            }),
        ModifyCommands::RmProficiency(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.proficiencies.retain(|p| p != &a.value);
                new_character
            }),
        ModifyCommands::Hitpoints(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.hit_points = a.new_value;
                new_character
            }),
        ModifyCommands::ArmorClass(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.armor_class = a.new_value;
                new_character
            }),
        ModifyCommands::Speed(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.speed = a.new_value;
                new_character
            }),
        ModifyCommands::Initiative(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.initiative_bonus = a.new_value;
                new_character
            }),
        ModifyCommands::Description => Character::default(),
        ModifyCommands::AddAttack(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.attacks.push(a.value.clone());
                new_character
            }),
        ModifyCommands::RmAttack(margs) => 
            modify_value(character, margs, |c, a| -> Character {
                let mut new_character = c.clone();
                new_character.attacks.retain(|p| p != &a.value);
                new_character
            }),
    }
}

fn construct_character_from_file(path: &str) -> Result<Character, ModifyCharOpError>{
    let file_contents = std::fs::read_to_string(path)
        .map_err(ModifyCharOpError::CouldNotFindFile)?;
    let character = toml::from_str(&file_contents)
        .map_err(ModifyCharOpError::Deserialization)?;
    Ok(character)
}

fn write_character_to_file(character: Character, path: &str) -> Result<(), ModifyCharOpError>{
    let serialized = toml::to_string_pretty(&character).map_err(ModifyCharOpError::Serialization)?;
    let mut file = std::fs::File::create(path).map_err(ModifyCharOpError::CouldNotCreateFile)?;
    file.write_all(serialized.as_bytes()).map_err(ModifyCharOpError::CouldNotWriteFile)?;
    Ok(())
}

fn modify_value<T>(character: &Character, args: &T, f: fn(&Character, &T) -> Character) -> Character {
    f(character, args)
}
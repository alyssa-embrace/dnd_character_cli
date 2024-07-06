use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub attack_bonus: i8,
    pub damage_bonus: i8,
    pub damage_dice: Vec<u8>,
    pub count: u8,
}

impl Ability {
    pub fn new(name: String, description: String, attack_bonus: i8, damage_bonus: i8, damage_dice: Vec<u8>, count: u8) -> Ability {
        Ability {
            name,
            description,
            attack_bonus,
            damage_bonus,
            damage_dice,
            count,
        }
    }
}
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attack {
    pub name: String,
    pub description: String,
    pub attack_bonus: u8,
    pub damage_bonus: u8,
    pub damage_dice: Vec<u8>,
    pub count: u8,
}

impl Attack {
    pub fn new(name: String, description: String, attack_bonus: u8, damage_bonus: u8, damage_dice: Vec<u8>, count: u8) -> Attack {
        Attack {
            name,
            description,
            attack_bonus,
            damage_bonus,
            damage_dice,
            count,
        }
    }
}
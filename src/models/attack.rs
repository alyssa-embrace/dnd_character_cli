use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub name: String,
    pub description: String,
    pub attack_bonus: u8,
    pub damage_bonus: u8,
    pub damage_dice: Vec<u8>,
    pub count: u8,
}
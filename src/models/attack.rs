use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub name: String,
    pub description: String,
    pub attack_bonus: i8,
    pub damage_bonus: i8,
    pub damage_dice: Vec<i8>,
}
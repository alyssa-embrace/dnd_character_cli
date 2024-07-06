use crate::models::statistics::AbilityScore;
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub ability_scores: [u8; 6],
    pub proficiency_bonus: u8,
    pub hit_points: u16,
    pub armor_class: u8,
    pub speed: u8,
    pub initiative_bonus: i8,
    pub proficiencies: Vec<String>,
    pub abilities: Vec<String>,
}

impl Character {
    pub fn new(name: String, 
        description: String,
        ability_scores: [u8; 6], 
        proficiency_bonus: u8, 
        hitpoints: u16, 
        armor_class: u8, 
        speed: u8, 
        initiative_bonus: i8,
        proficiencies: Vec<String>,
        abilities: Vec<String> ) -> Character {
        Character {
            name,
            description,
            ability_scores,
            proficiency_bonus,
            hit_points: hitpoints,
            armor_class,
            speed,
            initiative_bonus,
            proficiencies,
            abilities
        }
    }

    pub fn get_ability_score(&self, ability_score: AbilityScore) -> u8 {
        match ability_score {
            AbilityScore::Strength => self.ability_scores[0],
            AbilityScore::Dexterity => self.ability_scores[1],
            AbilityScore::Constitution => self.ability_scores[2],
            AbilityScore::Intelligence => self.ability_scores[3],
            AbilityScore::Wisdom => self.ability_scores[4],
            AbilityScore::Charisma => self.ability_scores[5],
        }
    }

    pub fn is_proficient(&self, proficiency: &String) -> bool {
        self.proficiencies.contains(&proficiency)
    }
}
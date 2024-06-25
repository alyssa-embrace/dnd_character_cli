use crate::models::attack::Attack;
use crate::models::statistics::AbilityScore;

pub struct Character {
    pub name: String,
    pub description: String,
    pub ability_scores: [i8; 6],
    pub proficiency_bonus: i8,
    pub hit_points: i16,
    pub armor_class: i8,
    pub speed: i8,
    pub initiative_bonus: i8,
    pub proficiencies: Vec<String>,
    pub attacks: Vec<Attack>,
}

impl Character {
    pub fn new(name: String, 
        description: String,
        ability_scores: [i8; 6], 
        proficiency_bonus: i8, 
        hit_points: i16, 
        armor_class: i8, 
        speed: i8, 
        initiative_bonus: i8,
        proficiencies: Vec<String>,
        attacks: Vec<Attack> ) -> Character {
        Character {
            name,
            description,
            ability_scores,
            proficiency_bonus,
            hit_points,
            armor_class,
            speed,
            initiative_bonus,
            proficiencies,
            attacks
        }
    }

    pub fn get_ability_score(&self, ability_score: AbilityScore) -> i8 {
        self.ability_scores[ability_score as usize]
    }

    pub fn set_ability_score(&mut self, ability_score: AbilityScore, value: i8) {
        self.ability_scores[ability_score as usize] = value
    }

    pub fn is_proficient(&self, proficiency: &String) -> bool {
        self.proficiencies.contains(&proficiency)
    }
}
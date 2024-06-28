use dialoguer::InputValidator;
pub struct ProficiencyValidator {}

impl InputValidator<String> for ProficiencyValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<u8>();
    
            if result.is_err() {
                return Err("Proficiency bonus must be a number".into());
            }
            else if !(2..=6).contains(&result.unwrap()) {
                return Err("Proficiency bonus must be between 2 and 6".into());
            }
    
            Ok(())
    }
}

pub struct AbilityScoreValidator {}

impl InputValidator<String> for AbilityScoreValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<u8>();
    
            if result.is_err() {
                return Err("Ability score must be a number".into());
            }
            else if !(1..=20).contains(&result.unwrap()) {
                return Err("Ability score must be between 1 and 20".into());
            }
    
            Ok(())
    }
}

pub struct ArmorClassValidator {}

impl InputValidator<String> for ArmorClassValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<u8>();
    
            if result.is_err() {
                return Err("Armor class must be a number".into());
            }
            else if !(1..=30).contains(&result.unwrap()) {
                return Err("Armor class must be between 1 and 30".into());
            }
    
            Ok(())
    }
}

pub struct InitiativeValidator {}

impl InputValidator<String> for InitiativeValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<i8>();
    
        if result.is_err() {
            return Err("Initiative bonus must be a number".into());
        }

        Ok(())
    }
}

pub struct HitpointsValidator {}

impl InputValidator<String> for HitpointsValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<u16>();
    
        if result.is_err() {
            return Err("Hit points must be a positive number".into());
        }
    
        Ok(())
    }
}

pub struct SpeedValidator {}

impl InputValidator<String> for SpeedValidator {
    type Err = String;
    
    fn validate(&mut self, input: &String) -> Result<(), String> {
        let result = input.parse::<u8>();
    
        if result.is_err() {
            return Err("Speed must be a number".into());
        }
        else if !(0..=240).contains(&result.unwrap()) {
            return Err("Speed must be between 0 and 240".into());
        }
    
        Ok(())
    }
}
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relic {
    Vajra,
    BurningBlood,
}

pub fn get_relic_visuals(relic: &Relic) -> (String, String, Color) {
    match relic {
        Relic::Vajra => ("V".to_string(), "Vajra: +1 Strength.".to_string(), Color::srgb(0.8, 0.2, 0.2)),
        Relic::BurningBlood => ("BB".to_string(), "Burning Blood: Heal 6 HP at end of combat.".to_string(), Color::srgb(0.6, 0.0, 0.0)),
    }
}

pub fn get_relic_name(relic: &Relic) -> String {
    match relic {
        Relic::Vajra => "Vajra".to_string(),
        Relic::BurningBlood => "Burning Blood".to_string(),
    }
}
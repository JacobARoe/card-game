use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Potion {
    Health,
    Strength,
    Energy,
}

pub fn get_potion_name(potion: &Potion) -> String {
    match potion {
        Potion::Health => "Health Potion".to_string(),
        Potion::Strength => "Strength Potion".to_string(),
        Potion::Energy => "Energy Potion".to_string(),
    }
}

pub fn get_potion_visuals(potion: &Potion) -> (String, String, Color) {
    match potion {
        Potion::Health => (
            "HP".to_string(),
            "Health Potion: Heal 10 HP.".to_string(),
            Color::srgb(0.8, 0.2, 0.2),
        ),
        Potion::Strength => (
            "STR".to_string(),
            "Strength Potion: +2 Strength.".to_string(),
            Color::srgb(0.8, 0.4, 0.0),
        ),
        Potion::Energy => (
            "NRG".to_string(),
            "Energy Potion: +2 Energy.".to_string(),
            Color::srgb(0.2, 0.8, 0.8),
        ),
    }
}

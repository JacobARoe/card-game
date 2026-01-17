use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relic {
    Vajra,
    BurningBlood,
    Anchor,
    OddlySmoothStone,
    BagOfMarbles,
}

pub fn get_relic_visuals(relic: &Relic) -> (String, String, Color) {
    match relic {
        Relic::Vajra => ("V".to_string(), "Vajra: +1 Strength.".to_string(), Color::srgb(0.8, 0.2, 0.2)),
        Relic::BurningBlood => ("BB".to_string(), "Burning Blood: Heal 6 HP at end of combat.".to_string(), Color::srgb(0.6, 0.0, 0.0)),
        Relic::Anchor => ("A".to_string(), "Anchor: Start combat with 10 Block.".to_string(), Color::srgb(0.2, 0.2, 0.8)),
        Relic::OddlySmoothStone => ("S".to_string(), "Smooth Stone: +1 Block from cards.".to_string(), Color::srgb(0.6, 0.6, 0.6)),
        Relic::BagOfMarbles => ("M".to_string(), "Marbles: Apply 1 Weak to enemy at start.".to_string(), Color::srgb(0.2, 0.8, 0.2)),
    }
}

pub fn get_relic_name(relic: &Relic) -> String {
    match relic {
        Relic::Vajra => "Vajra".to_string(),
        Relic::BurningBlood => "Burning Blood".to_string(),
        Relic::Anchor => "Anchor".to_string(),
        Relic::OddlySmoothStone => "Oddly Smooth Stone".to_string(),
        Relic::BagOfMarbles => "Bag of Marbles".to_string(),
    }
}
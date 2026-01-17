use bevy::prelude::*;
use crate::components::Card;
use rand::{thread_rng, Rng};

pub fn get_card_visuals(card: &Card) -> (Color, Color) {
    if card.apply_poison > 0 || card.apply_weak > 0 {
        (Color::srgb(0.2, 0.0, 0.3), Color::srgb(0.6, 0.0, 0.8)) // Purple theme (Status)
    } else if card.damage > 0 {
        (Color::srgb(0.3, 0.1, 0.1), Color::srgb(0.8, 0.2, 0.2)) // Red theme (Attack)
    } else if card.block > 0 {
        (Color::srgb(0.1, 0.1, 0.3), Color::srgb(0.2, 0.2, 0.8)) // Blue theme (Defense)
    } else {
        (Color::srgb(0.2, 0.2, 0.2), Color::srgb(0.5, 0.5, 0.5)) // Grey theme (Utility/Other)
    }
}

pub fn strike() -> Card {
    Card { name: "Strike".to_string(), damage: 6, block: 0, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn iron_wave() -> Card {
    Card { name: "Iron Wave".to_string(), damage: 5, block: 5, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn deadly_poison() -> Card {
    Card { name: "Deadly Poison".to_string(), damage: 0, block: 0, cost: 1, apply_poison: 5, apply_weak: 0, upgraded: false }
}

pub fn heavy_blade() -> Card {
    Card { name: "Heavy Blade".to_string(), damage: 14, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn shrug_it_off() -> Card {
    Card { name: "Shrug It Off".to_string(), damage: 0, block: 8, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn clothesline() -> Card {
    Card { name: "Clothesline".to_string(), damage: 12, block: 0, cost: 2, apply_poison: 0, apply_weak: 2, upgraded: false }
}

pub fn quick_slash() -> Card {
    Card { name: "Quick Slash".to_string(), damage: 4, block: 0, cost: 0, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn bludgeon() -> Card {
    Card { name: "Bludgeon".to_string(), damage: 20, block: 0, cost: 3, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn poison_cloud() -> Card {
    Card { name: "Poison Cloud".to_string(), damage: 0, block: 0, cost: 2, apply_poison: 4, apply_weak: 1, upgraded: false }
}

pub fn fortify() -> Card {
    Card { name: "Fortify".to_string(), damage: 0, block: 12, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false }
}

pub fn generate_random_card() -> Card {
    let mut rng = thread_rng();
    let r = rng.gen_range(0..10);
    match r {
        0 => iron_wave(),
        1 => deadly_poison(),
        2 => heavy_blade(),
        3 => shrug_it_off(),
        4 => clothesline(),
        5 => quick_slash(),
        6 => bludgeon(),
        7 => poison_cloud(),
        8 => fortify(),
        _ => strike(),
    }
}
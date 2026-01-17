use bevy::prelude::*;
use crate::components::{Card, Rarity};
use rand::{thread_rng, Rng};

pub fn get_card_visuals(card: &Card) -> (Color, Color) {
    let bg_color = if card.apply_poison > 0 || card.apply_weak > 0 {
        Color::srgb(0.2, 0.0, 0.3) // Purple theme (Status)
    } else if card.damage > 0 {
        Color::srgb(0.3, 0.1, 0.1) // Red theme (Attack)
    } else if card.block > 0 {
        Color::srgb(0.1, 0.1, 0.3) // Blue theme (Defense)
    } else {
        Color::srgb(0.2, 0.2, 0.2) // Grey theme (Utility/Other)
    };

    let border_color = match card.rarity {
        Rarity::Common => Color::srgb(0.5, 0.5, 0.5),
        Rarity::Rare => Color::srgb(0.2, 0.8, 1.0),
        Rarity::Legendary => Color::srgb(1.0, 0.84, 0.0),
    };

    (bg_color, border_color)
}

pub fn strike() -> Card {
    Card { name: "Strike".to_string(), damage: 7, block: 0, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn defend() -> Card {
    Card { name: "Defend".to_string(), damage: 0, block: 8, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn bash() -> Card {
    Card { name: "Bash".to_string(), damage: 12, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn iron_wave() -> Card {
    Card { name: "Iron Wave".to_string(), damage: 5, block: 5, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn deadly_poison() -> Card {
    Card { name: "Deadly Poison".to_string(), damage: 0, block: 0, cost: 1, apply_poison: 10, apply_weak: 0, upgraded: false, rarity: Rarity::Rare }
}

pub fn heavy_blade() -> Card {
    Card { name: "Heavy Blade".to_string(), damage: 16, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Legendary }
}

pub fn shrug_it_off() -> Card {
    Card { name: "Shrug It Off".to_string(), damage: 0, block: 8, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn clothesline() -> Card {
    Card { name: "Clothesline".to_string(), damage: 12, block: 0, cost: 2, apply_poison: 0, apply_weak: 2, upgraded: false, rarity: Rarity::Rare }
}

pub fn quick_slash() -> Card {
    Card { name: "Quick Slash".to_string(), damage: 4, block: 0, cost: 0, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Common }
}

pub fn bludgeon() -> Card {
    Card { name: "Bludgeon".to_string(), damage: 30, block: 0, cost: 3, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Legendary }
}

pub fn poison_cloud() -> Card {
    Card { name: "Poison Cloud".to_string(), damage: 0, block: 0, cost: 2, apply_poison: 5, apply_weak: 3, upgraded: false, rarity: Rarity::Rare }
}

pub fn fortify() -> Card {
    Card { name: "Fortify".to_string(), damage: 0, block: 20, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false, rarity: Rarity::Rare }
}

pub fn generate_random_card() -> Card {
    let mut rng = thread_rng();
    let roll = rng.gen_range(0..100);

    if roll < 60 {
        // Common (60%)
        let r = rng.gen_range(0..3);
        match r {
            0 => iron_wave(),
            1 => shrug_it_off(),
            _ => quick_slash(),
        }
    } else if roll < 90 {
        // Rare (30%)
        let r = rng.gen_range(0..4);
        match r {
            0 => deadly_poison(),
            1 => clothesline(),
            2 => poison_cloud(),
            _ => fortify(),
        }
    } else {
        // Legendary (10%)
        let r = rng.gen_range(0..2);
        match r {
            0 => heavy_blade(),
            _ => bludgeon(),
        }
    }
}
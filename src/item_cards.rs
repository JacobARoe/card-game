use crate::components::{Card, CharacterClass, Rarity, SpellElement};
use bevy::prelude::*;
use rand::{Rng, thread_rng};

pub fn get_card_visuals(card: &Card) -> (Color, Color) {
    let bg_color = if card.apply_poison > 0 || card.apply_weak > 0 {
        Color::srgb(0.2, 0.0, 0.3) // Purple theme (Status)
    } else if card.apply_stun > 0 {
        Color::srgb(0.4, 0.4, 0.0) // Yellow/Gold theme (Stun)
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
    Card {
        name: "Strike".to_string(),
        damage: 7,
        block: 0,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn defend() -> Card {
    Card {
        name: "Defend".to_string(),
        damage: 0,
        block: 8,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn bash() -> Card {
    Card {
        name: "Bash".to_string(),
        damage: 12,
        block: 0,
        cost: 2,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn iron_wave() -> Card {
    Card {
        name: "Iron Wave".to_string(),
        damage: 5,
        block: 5,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn deadly_poison() -> Card {
    Card {
        name: "Deadly Poison".to_string(),
        damage: 0,
        block: 0,
        cost: 1,
        apply_poison: 10,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn heavy_blade() -> Card {
    // Requires 3 combo points to deal 32 damage
    Card {
        name: "Heavy Blade".to_string(),
        damage: 16,
        block: 0,
        cost: 2,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Legendary,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 3,
    }
}

pub fn shrug_it_off() -> Card {
    Card {
        name: "Shrug It Off".to_string(),
        damage: 0,
        block: 8,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn clothesline() -> Card {
    Card {
        name: "Clothesline".to_string(),
        damage: 12,
        block: 0,
        cost: 2,
        apply_poison: 0,
        apply_weak: 2,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn quick_slash() -> Card {
    Card {
        name: "Quick Slash".to_string(),
        damage: 4,
        block: 0,
        cost: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 2,
        finisher_combo_cost: 0,
    }
}

pub fn bludgeon() -> Card {
    Card {
        name: "Bludgeon".to_string(),
        damage: 30,
        block: 0,
        cost: 3,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Legendary,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn poison_cloud() -> Card {
    Card {
        name: "Poison Cloud".to_string(),
        damage: 0,
        block: 0,
        cost: 2,
        apply_poison: 5,
        apply_weak: 3,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn fortify() -> Card {
    Card {
        name: "Fortify".to_string(),
        damage: 0,
        block: 20,
        cost: 2,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn uppercut() -> Card {
    Card {
        name: "Uppercut".to_string(),
        damage: 8,
        block: 0,
        cost: 2,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 1,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 1,
        finisher_combo_cost: 0,
    }
}

pub fn flashbang() -> Card {
    Card {
        name: "Flashbang".to_string(),
        damage: 0,
        block: 0,
        cost: 1,
        apply_poison: 0,
        apply_weak: 2,
        apply_stun: 1,
        upgraded: false,
        rarity: Rarity::Rare,
        is_spell_modifier: false,
        is_spell_source: false,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn magic_bolt() -> Card {
    Card {
        name: "Magic Bolt".to_string(),
        damage: 6,
        block: 0,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: true,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn magic_shield() -> Card {
    Card {
        name: "Magic Shield".to_string(),
        damage: 0,
        block: 8,
        cost: 1,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: true,
        element: SpellElement::Neutral,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn fire_essence() -> Card {
    Card {
        name: "Fire Essence".to_string(),
        damage: 3,
        block: 0,
        cost: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: true,
        is_spell_source: false,
        element: SpellElement::Fire,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn ice_essence() -> Card {
    Card {
        name: "Ice Essence".to_string(),
        damage: 3,
        block: 0,
        cost: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: true,
        is_spell_source: false,
        element: SpellElement::Ice,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn wind_essence() -> Card {
    Card {
        name: "Wind Essence".to_string(),
        damage: 3,
        block: 0,
        cost: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: true,
        is_spell_source: false,
        element: SpellElement::Wind,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn stone_essence() -> Card {
    Card {
        name: "Stone Essence".to_string(),
        damage: 3,
        block: 0,
        cost: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: true,
        is_spell_source: false,
        element: SpellElement::Stone,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    }
}

pub fn generate_random_card(class: CharacterClass) -> Card {
    let mut rng = thread_rng();
    let roll = rng.gen_range(0..100);

    match class {
        CharacterClass::Duelist => {
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
                let r = rng.gen_range(0..6);
                match r {
                    0 => deadly_poison(),
                    1 => clothesline(),
                    2 => poison_cloud(),
                    3 => fortify(),
                    4 => uppercut(),
                    _ => flashbang(),
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
        CharacterClass::Spellweaver => {
            // Simple distribution for Spellweaver for now
            let r = rng.gen_range(0..6);
            match r {
                0 => magic_bolt(),
                1 => magic_shield(),
                2 => fire_essence(),
                3 => ice_essence(),
                4 => stone_essence(),
                _ => wind_essence(),
            }
        }
    }
}

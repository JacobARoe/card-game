use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::components::*;
use crate::resources::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_game(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    // Despawn existing player if any (for restart)
    for entity in &player_query {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn Player (Persistent)
    commands.spawn((
        Player,
        Health { current: 50, max: 50 },
        Energy { current: 3, max: 3 },
        Block { value: 0 },
        StatusStore::default(),
        RelicStore { relics: vec![Relic::BurningBlood] },
        Gold { amount: 100 },
    ));

    // Create Deck
    let mut deck_cards = Vec::new();
    for _ in 0..5 {
        deck_cards.push(Card { name: "Strike".to_string(), damage: 6, block: 0, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false });
    }
    for _ in 0..4 {
        deck_cards.push(Card { name: "Bash".to_string(), damage: 10, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false });
    }
    for _ in 0..4 {
        deck_cards.push(Card { name: "Defend".to_string(), damage: 0, block: 5, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false });
    }
    // Add Poison Cards
    for _ in 0..2 {
        deck_cards.push(Card { name: "Poison Stab".to_string(), damage: 4, block: 0, cost: 1, apply_poison: 3, apply_weak: 0, upgraded: false });
    }
    // Add Weak Cards
    for _ in 0..2 {
        deck_cards.push(Card { name: "Intimidate".to_string(), damage: 0, block: 0, cost: 0, apply_poison: 0, apply_weak: 2, upgraded: false });
    }

    // Shuffle
    let mut rng = thread_rng();
    deck_cards.shuffle(&mut rng);

    commands.insert_resource(Deck { cards: deck_cards });
    commands.insert_resource(DiscardPile::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
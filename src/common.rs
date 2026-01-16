use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

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
        PotionStore { potions: vec![PotionType::Health, PotionType::Strength] },
        Gold { amount: 200 },
    ));

    // Create Deck
    let mut deck_cards = Vec::new();
    for _ in 0..5 {
        deck_cards.push(Card { name: "Strike".to_string(), damage: 6, block: 0, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false });
    }
    for _ in 0..2 {
        deck_cards.push(Card { name: "Bash".to_string(), damage: 10, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false });
    }
    for _ in 0..3 {
        deck_cards.push(Card { name: "Defend".to_string(), damage: 0, block: 5, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false });
    }

    // Shuffle
    let mut rng = thread_rng();
    deck_cards.shuffle(&mut rng);

    commands.insert_resource(Deck { cards: deck_cards });
    commands.insert_resource(DiscardPile::default());

    // Generate Map
    let mut levels = Vec::new();
    let mut rng = thread_rng();

    // Level 0: Start (3 Battle Nodes)
    let mut start_nodes = Vec::new();
    for _ in 0..3 {
        start_nodes.push(MapNodeData { 
            node_type: NodeType::Battle, 
            next_indices: vec![],
            y_jitter: rng.gen_range(-20.0..20.0),
        });
    }
    levels.push(start_nodes);

    // Levels 1-4: Random Encounters
    for i in 1..=4 {
        let mut nodes = Vec::new();
        for _ in 0..3 { // 3 nodes per level
            let r = rng.gen_range(0..100);
            let node_type = if i >= 2 && r < 20 { NodeType::Elite }
                else if r < 50 { NodeType::Battle } 
                else if r < 70 { NodeType::Event }
                else if r < 85 { NodeType::Shop } 
                else { NodeType::Rest };
            nodes.push(MapNodeData { 
                node_type, 
                next_indices: vec![],
                y_jitter: rng.gen_range(-40.0..40.0),
            });
        }
        levels.push(nodes);
    }

    // Level 5: Boss
    levels.push(vec![MapNodeData { 
        node_type: NodeType::Boss, 
        next_indices: vec![],
        y_jitter: 0.0,
    }]);

    // Connect Levels
    for i in 0..5 {
        let current_len = levels[i].len();
        let next_len = levels[i+1].len();
        
        for j in 0..current_len {
            // Ensure at least one connection
            let next_idx = rng.gen_range(0..next_len);
            levels[i][j].next_indices.push(next_idx);
            
            // Add random extra connections
            if rng.gen_bool(0.3) {
                let extra = rng.gen_range(0..next_len);
                if extra != next_idx {
                    levels[i][j].next_indices.push(extra);
                }
            }
        }
        
        // Ensure every node in next level has a parent
        for k in 0..next_len {
            let mut has_parent = false;
            for j in 0..current_len {
                if levels[i][j].next_indices.contains(&k) {
                    has_parent = true;
                    break;
                }
            }
            if !has_parent {
                let parent = rng.gen_range(0..current_len);
                levels[i][parent].next_indices.push(k);
            }
        }
    }

    commands.insert_resource(GameMap {
        levels,
        current_node: None,
        visited_path: Vec::new(),
    });
    commands.insert_resource(RewardStore::default());
}

pub fn despawn_screen<T: Component>(
    to_despawn: Query<(Entity, Option<&Parent>), With<T>>,
    parent_check: Query<(), With<T>>,
    mut commands: Commands,
) {
    for (entity, parent) in &to_despawn {
        let parent_has_component = parent
            .and_then(|p| parent_check.get(p.get()).ok())
            .is_some();

        if !parent_has_component {
            commands.entity(entity).despawn_recursive();
        }
    }
}
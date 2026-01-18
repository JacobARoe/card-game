use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::components::*;
use crate::resources::*;
use crate::item_cards;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    // Standard 2D Camera for UI, Battle, etc.
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}

pub fn setup_game(mut commands: Commands, player_query: Query<Entity, With<Player>>, run_state: Res<RunState>) {
    // Despawn existing player if any (for restart)
    for entity in &player_query {
        commands.entity(entity).despawn_recursive();
    }

    let (starting_relics, starting_deck) = match run_state.character_class {
        crate::components::CharacterClass::Duelist => {
            let mut deck = Vec::new();
            for _ in 0..5 { deck.push(item_cards::strike()); }
            for _ in 0..2 { deck.push(item_cards::bash()); }
            for _ in 0..3 { deck.push(item_cards::defend()); }
            (Vec::new(), deck)
        },
        crate::components::CharacterClass::Spellweaver => {
            let mut deck = Vec::new();
            for _ in 0..3 { deck.push(item_cards::fire_essence()); }
            for _ in 0..2 { deck.push(item_cards::wind_essence()); }
            for _ in 0..3 { deck.push(item_cards::magic_bolt()); }
            for _ in 0..2 { deck.push(item_cards::magic_shield()); }
            (Vec::new(), deck)
        }
    };

    // Spawn Player (Persistent)
    let mut player_cmds = commands.spawn((
        Player,
        Health { current: 50, max: 50 },
        Energy { current: 3, max: 3 },
        Block { value: 0 },
        StatusStore::default(),
        RelicStore { relics: starting_relics },
        PotionStore { potions: Vec::new() },
        Gold { amount: 200 },
    ));

    if run_state.character_class == CharacterClass::Spellweaver {
        player_cmds.insert((
            Mana { current: 0 },
            ActiveSpell::default(),
        ));
    }

    // Create Deck
    let mut deck_cards = starting_deck;

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
            visible: true,
        });
    }
    levels.push(start_nodes);

    // Levels 1-13: Random Encounters
    for i in 1..=13 {
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
                visible: false,
            });
        }
        levels.push(nodes);
    }

    // Level 14: Boss
    levels.push(vec![MapNodeData { 
        node_type: NodeType::Boss, 
        next_indices: vec![],
        visible: false,
    }]);

    // Connect Levels
    for i in 0..14 {
        let current_len = levels[i].len();
        let next_len = levels[i+1].len();
        
        for j in 0..current_len {
            // Identify valid adjacent candidates in the next level
            // A node at index j can connect to j-1, j, j+1 in the next level
            let min_next = j.saturating_sub(1);
            let max_next = (j + 1).min(next_len - 1);
            
            let mut candidates: Vec<usize> = (min_next..=max_next).collect();
            
            // If strict adjacency yields no candidates (e.g. due to width change), connect to closest
            if candidates.is_empty() {
                candidates.push(max_next);
            }
            
            // Ensure at least one connection
            if let Some(&selected) = candidates.choose(&mut rng) {
                levels[i][j].next_indices.push(selected);
            }
            
            // Chance to add more connections from valid candidates
            for &candidate in &candidates {
                if !levels[i][j].next_indices.contains(&candidate) {
                    if rng.gen_bool(0.3) { // 30% chance to add extra path
                        levels[i][j].next_indices.push(candidate);
                    }
                }
            }
            levels[i][j].next_indices.sort();
            levels[i][j].next_indices.dedup();
        }
        
        // Ensure every node in next level has a parent (to prevent unreachable nodes)
        for k in 0..next_len {
            let mut has_parent = false;
            for j in 0..current_len {
                if levels[i][j].next_indices.contains(&k) {
                    has_parent = true;
                    break;
                }
            }
            
            if !has_parent {
                // Find a valid parent for node k
                // Valid parents are those where k is adjacent (p-1 <= k <= p+1)
                let min_parent = k.saturating_sub(1);
                let max_parent = (k + 1).min(current_len - 1);
                let mut valid_parents: Vec<usize> = (min_parent..=max_parent).collect();
                
                // If strict adjacency yields no parents, use closest
                if valid_parents.is_empty() {
                    valid_parents.push(max_parent);
                }
                
                if let Some(&parent) = valid_parents.choose(&mut rng) {
                    levels[i][parent].next_indices.push(k);
                    levels[i][parent].next_indices.sort();
                    levels[i][parent].next_indices.dedup();
                }
            }
        }
    }

    commands.insert_resource(GameMap {
        levels,
        current_node: None,
        visited_path: Vec::new(),
    });
    commands.insert_resource(RewardStore::default());
    commands.insert_resource(ShopStore::default());
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

pub fn spawn_card_visual(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    card: &Card,
    extra_bundle: impl Bundle,
    extension: impl FnOnce(&mut ChildBuilder),
) {
    let (bg_color, border_color) = item_cards::get_card_visuals(card);

    parent.spawn((
        NodeBundle {
        style: Style {
            width: Val::Px(120.0),
            height: Val::Px(180.0),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: bg_color.into(),
        border_color: border_color.into(),
        ..default()
        },
        extra_bundle,
    )).with_children(|card_ui| {
        // Card Image
        card_ui.spawn(NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                margin: UiRect::top(Val::Px(10.0)),
                overflow: bevy::ui::Overflow::clip(),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
            }).with_children(|parent| {
                parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(200.0),
                    height: Val::Auto,
                    ..default()
                },
                image: asset_server.load(format!("images/cards/{}.jpg", card.name.trim_end_matches('+'))).into(),
                ..default()
            });
        });

        // Card Name
        card_ui.spawn(TextBundle::from_section(
            &card.name,
            TextStyle {
                font: Handle::default(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(5.0)),
            ..default()
        }));

        // Description
        let mut desc = String::new();
        if card.damage > 0 { desc.push_str(&format!("Deal {} Dmg\n", card.damage)); }
        if card.block > 0 { desc.push_str(&format!("Gain {} Blk\n", card.block)); }
        if card.apply_poison > 0 { desc.push_str(&format!("Apply {} Psn\n", card.apply_poison)); }
        if card.apply_weak > 0 { desc.push_str(&format!("Apply {} Wk\n", card.apply_weak)); }
        if card.apply_stun > 0 { desc.push_str("Stun Enemy\n"); }

        card_ui.spawn(TextBundle::from_section(
            desc,
            TextStyle {
                font: Handle::default(),
                font_size: 12.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(5.0)),
            ..default()
        }));

        extension(card_ui);

        // Energy Glyph
        card_ui.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(-5.0),
                top: Val::Px(-5.0),
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.2, 0.7, 0.9).into(),
            z_index: ZIndex::Local(10),
            ..default()
        }).with_children(|energy| {
            energy.spawn(TextBundle::from_section(
                card.cost.to_string(),
                TextStyle {
                    font: Handle::default(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

pub fn resize_background_system(
    window_query: Query<&Window>,
    mut bg_query: Query<(&mut Transform, &Handle<Image>), With<SceneBackground>>,
    images: Res<Assets<Image>>,
) {
    let window = window_query.single();
    for (mut transform, image_handle) in &mut bg_query {
        if let Some(image) = images.get(image_handle) {
            let win_w = window.width();
            let win_h = window.height();
            let img_w = image.size().x as f32;
            let img_h = image.size().y as f32;
            let scale = (win_w / img_w).max(win_h / img_h);
            transform.scale = Vec3::splat(scale);
        }
    }
}
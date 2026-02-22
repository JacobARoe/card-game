use bevy::color::Alpha;
use bevy::prelude::*;
use rand::Rng;
use rand::thread_rng;

use crate::cli::{EndTurnRequest, PlayCardRequest, TriggerReflexRequest};
use crate::common::spawn_card_visual;
use crate::components::*;
use crate::enemies::generate_enemy_move;
use crate::item_potions::{Potion, get_potion_visuals};
use crate::item_relics::Relic;
use crate::resources::*;
use crate::states::*;
#[derive(Component)]
pub struct CardAnimating {
    pub start: Vec2,
    pub target: Vec2,
    pub timer: Timer,
    pub card: Card,
    pub target_index: Option<usize>,
}

#[derive(Component)]
pub struct FirstTurn;

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    game_map: Res<GameMap>,
    deck: Res<Deck>,
    discard: Res<DiscardPile>,
    mut reward_store: ResMut<RewardStore>,
    player_query: Query<&RelicStore, With<Player>>,
) {
    println!("Setting up battle...");

    // Determine Enemy Type
    let (level, node_type) = if let Some((l, i)) = game_map.current_node {
        (l, game_map.levels[l][i].node_type)
    } else {
        (0, NodeType::Battle)
    };

    let mut rng = thread_rng();
    let mut enemies_to_spawn = Vec::new();

    match node_type {
        NodeType::Boss => enemies_to_spawn.push(EnemyKind::Dragon),
        NodeType::Elite => enemies_to_spawn.push(EnemyKind::DarkKnight),
        _ => {
            if level >= 3 {
                if rng.gen_bool(0.6) {
                    enemies_to_spawn.push(EnemyKind::Orc);
                    enemies_to_spawn.push(EnemyKind::Goblin);
                } else {
                    enemies_to_spawn.push(EnemyKind::Orc);
                }
            } else {
                if rng.gen_bool(0.5) {
                    enemies_to_spawn.push(EnemyKind::Goblin);
                    enemies_to_spawn.push(EnemyKind::Goblin);
                    if rng.gen_bool(0.3) {
                        enemies_to_spawn.push(EnemyKind::Goblin);
                    }
                } else {
                    enemies_to_spawn.push(EnemyKind::Orc);
                }
            }
        }
    };

    // Spawn Background
    // Use the first enemy type to determine background
    let bg_image = if let Some(first) = enemies_to_spawn.first() {
        match first {
            EnemyKind::Dragon => "images/backgrounds/DragonLayer.jpg",
            EnemyKind::Goblin => "images/backgrounds/PoisonCave.jpg",
            EnemyKind::Orc => "images/backgrounds/RuinedForest.jpg",
            _ => "images/backgrounds/Battlefield.jpg",
        }
    } else {
        "images/backgrounds/Battlefield.jpg"
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(bg_image),
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            ..default()
        },
        BattleEntity,
        SceneBackground,
        FirstTurn,
    ));

    // Relic Logic: Bag of Marbles
    let mut initial_weak = 0;
    if let Ok(relics) = player_query.get_single() {
        if relics.relics.contains(&Relic::BagOfMarbles) {
            initial_weak = 1;
            println!("Bag of Marbles applied 1 Weak!");
        }
    }

    // Spawn Enemies
    for (i, enemy_kind) in enemies_to_spawn.iter().enumerate() {
        let (hp, name, enemy_sprite) = match enemy_kind {
            EnemyKind::Goblin => (20, "Goblin", "images/enemies/Goblin.png"),
            EnemyKind::Orc => (40, "Orc", "images/enemies/Orc.png"),
            EnemyKind::Dragon => (150, "Dragon", "images/enemies/Dragon.png"),
            EnemyKind::DarkKnight => (80, "Dark Knight", "images/enemies/DarkKnight.png"),
        };

        let initial_move = generate_enemy_move(*enemy_kind, false);
        let x_offset = 50.0 + (i as f32 * 250.0);

        let mut entity_cmds = commands.spawn((
            Enemy { kind: *enemy_kind },
            Health {
                current: hp,
                max: hp,
            },
            Block { value: 0 },
            StatusStore {
                weak: initial_weak,
                ..default()
            },
            initial_move,
            BattleEntity,
            Interaction::default(), // Allow clicking
            Tooltip {
                text: String::new(),
            },
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(50.0),
                    right: Val::Px(x_offset),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            },
        ));

        if i == 0 {
            entity_cmds.insert(Selected);
        }

        entity_cmds.with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                image: asset_server.load(enemy_sprite).into(),
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                name,
                TextStyle {
                    font: Handle::default(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));

            // Health & Block Row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        TextBundle::from_section(
                            format!("HP: {}/{}", hp, hp),
                            TextStyle {
                                font: Handle::default(),
                                font_size: 20.0,
                                color: Color::srgb(0.9, 0.3, 0.3),
                            },
                        ),
                        EnemyHealthText,
                    ));
                    row.spawn((
                        TextBundle::from_section(
                            "Block: 0",
                            TextStyle {
                                font: Handle::default(),
                                font_size: 20.0,
                                color: Color::srgb(0.5, 0.5, 1.0),
                            },
                        ),
                        EnemyBlockText,
                    ));
                });

            // Status
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                },
                EnemyStatusText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Planning...",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 20.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                    },
                ),
                EnemyIntentText,
            ));
        });
    }

    // Spawn a "Hand" UI container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        },
        BattleEntity,
        HandContainer,
    ));

    // Spawn End Turn Button
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
            BattleEntity,
            EndTurnButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "End Turn",
                TextStyle {
                    font: Handle::default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

    // Spawn Player Stats Panel (Bottom Left)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
            BattleEntity,
        ))
        .with_children(|parent| {
            // Player Health
            parent.spawn((
                TextBundle::from_section(
                    "Player: 50/50",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                PlayerHealthText,
            ));
            // Player Block
            parent.spawn((
                TextBundle::from_section(
                    "Block: 0",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::srgb(0.5, 0.5, 1.0),
                    },
                ),
                PlayerBlockText,
            ));
            // Player Energy
            parent.spawn((
                TextBundle::from_section(
                    "Energy: 3/3",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::srgb(0.2, 0.8, 1.0),
                    },
                ),
                PlayerEnergyText,
            ));
            // Player Combo
            parent.spawn((
                TextBundle::from_section(
                    "Combo: 0",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.5, 0.0),
                    },
                ),
                PlayerComboText,
            ));
            // Player Spell Container
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(30.0),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                },
                PlayerSpellContainer,
            ));
            // Player Gold
            parent.spawn((
                TextBundle::from_section(
                    "Gold: 0",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.84, 0.0),
                    },
                ),
                PlayerGoldText,
            ));
            // Player Status
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                PlayerStatusText,
            ));
            // Player Relics
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                PlayerRelicText,
            ));
            // Potions Container
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                },
                PotionContainer,
            ));
        });

    // Spawn Deck UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    bottom: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                ..default()
            },
            BattleEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Deck: {}", deck.cards.len()),
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                DeckText,
            ));
        });

    // Spawn Discard UI
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    bottom: Val::Px(100.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                ..default()
            },
            BattleEntity,
            DiscardPileButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Discard: {}", discard.cards.len()),
                    TextStyle {
                        font: Handle::default(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                DiscardText,
            ));
        });

    // Spawn Damage Flash Overlay
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::srgba(1.0, 0.0, 0.0, 0.0).into(),
            z_index: ZIndex::Global(100),
            ..default()
        },
        BattleEntity,
        DamageFlashUi,
    ));

    // Spawn Block Flash Overlay
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.5, 1.0, 0.0).into(),
            z_index: ZIndex::Global(95),
            ..default()
        },
        BattleEntity,
        BlockFlashUi,
    ));

    // Reset Reward Store for the new battle
    *reward_store = RewardStore::default();

    next_turn_state.set(TurnState::PlayerTurnStart);
}

pub fn draw_cards_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut deck: ResMut<Deck>,
    mut discard: ResMut<DiscardPile>,
    hand_container_query: Query<Entity, With<HandContainer>>,
    mut player_query: Query<
        (
            &mut Energy,
            Option<&mut Mana>,
            &mut Block,
            &mut Health,
            &mut StatusStore,
            &RelicStore,
            Option<&mut ActiveSpell>,
        ),
        With<Player>,
    >,
    mut flash_query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
    first_turn_query: Query<Entity, With<FirstTurn>>,
    run_state: Res<RunState>,
) {
    if let Ok((mut energy, mut mana, mut block, mut health, mut status, relics, mut active_spell)) =
        player_query.get_single_mut()
    {
        // Reset Energy / Mana
        if run_state.character_class == CharacterClass::Spellweaver {
            if let Some(ref mut m) = mana {
                m.current += 2;
            }
            // No cap
        } else {
            energy.current = energy.max;
        }

        // Reset Active Spell for new turn
        if let Some(ref mut spell) = active_spell {
            **spell = ActiveSpell::default();
        }

        // Reset Block (unless Anchor on first turn)
        if let Ok(entity) = first_turn_query.get_single() {
            commands.entity(entity).remove::<FirstTurn>();
            if relics.relics.contains(&Relic::Anchor) {
                block.value += 10;
                println!("Anchor: +10 Block");
            }
        } else {
            block.value = 0;
        }

        // Poison Logic (Start of Turn)
        if status.poison > 0 {
            health.current -= status.poison;
            println!("Player takes {} poison damage!", status.poison);
            status.poison -= 1;
            // Flash red
            for mut bg in &mut flash_query {
                bg.0 = Color::srgba(1.0, 0.0, 0.0, 0.5).into();
            }
        }
        if health.current <= 0 {
            println!("Player died to poison!");
            next_game_state.set(GameState::GameOver);
            return;
        }
    }

    let mut hand_cards = Vec::new();

    // Draw 5 cards
    for _ in 0..5 {
        if deck.cards.is_empty() {
            if discard.cards.is_empty() {
                break;
            }
            // Shuffle discard into deck
            deck.cards.append(&mut discard.cards);
            let mut rng = thread_rng();
            use rand::seq::SliceRandom;
            deck.cards.shuffle(&mut rng);
            println!("Reshuffled discard pile into deck.");
        }

        if let Some(card) = deck.cards.pop() {
            hand_cards.push(card);
        }
    }

    // Spawn cards as children of HandContainer
    if let Ok(container) = hand_container_query.get_single() {
        let ui_hand = hand_cards.clone();
        commands.entity(container).with_children(|parent| {
            for card in ui_hand {
                spawn_card_visual(
                    parent,
                    &asset_server,
                    &card,
                    (
                        card.clone(),
                        BaseColor(Color::srgb(0.15, 0.15, 0.2)),
                        Button,
                        Interaction::default(),
                    ),
                    |_| {},
                );
            }
        });
    }

    // Output Hand for CLI Users
    println!("--- PLAYER TURN START ---");
    println!("Hand:");
    for (i, card) in hand_cards.iter().enumerate() {
        println!(
            "  [{}] {} (Cost: {}, Dmg: {}, Blk: {})",
            i, card.name, card.cost, card.damage, card.block
        );
    }
    println!("--- Type 'help' for commands ---");

    next_turn_state.set(TurnState::PlayerTurn);
}

pub fn discard_hand_system(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut discard: ResMut<DiscardPile>,
    query: Query<(Entity, &Card)>,
    mut player_status_query: Query<&mut StatusStore, With<Player>>,
) {
    for (entity, card) in query.iter() {
        discard.cards.push(card.clone());
        commands.entity(entity).despawn_recursive();
    }

    // Decrement Player Weak at end of turn
    if let Ok(mut status) = player_status_query.get_single_mut() {
        if status.weak > 0 {
            status.weak -= 1;
        }
    }

    next_turn_state.set(TurnState::EnemyTurn);
}

pub fn end_turn_button_system(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_turn_state.set(TurnState::PlayerTurnEnd);
        }
    }
}

pub fn card_interaction_system(
    card_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Card>)>,
    hand_container_query: Query<&Children, With<HandContainer>>,
    selected_enemy: Query<Entity, With<Selected>>,
    enemy_list: Query<Entity, With<Enemy>>,
    mut ev_play: EventWriter<PlayCardRequest>,
) {
    for (card_entity, interaction) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(children) = hand_container_query.get_single() {
                if let Some(hand_index) = children.iter().position(|&e| e == card_entity) {
                    let mut target_index = None;
                    if let Ok(selected_ent) = selected_enemy.get_single() {
                        target_index = enemy_list.iter().position(|e| e == selected_ent);
                    }

                    ev_play.send(PlayCardRequest {
                        hand_index,
                        target_index,
                    });
                }
            }
        }
    }
}

pub fn process_end_turn_requests(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut ev_end: EventReader<EndTurnRequest>,
) {
    for _ in ev_end.read() {
        next_turn_state.set(TurnState::PlayerTurnEnd);
    }
}

pub fn process_play_card_requests(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut ev_play: EventReader<PlayCardRequest>,
    hand_container_query: Query<&Children, With<HandContainer>>,
    card_query: Query<(&Card, &GlobalTransform), Without<CardAnimating>>,
    mut animating_query: Query<(Entity, &mut Style, &mut CardAnimating)>,
    mut targetable_enemies: Query<Entity, With<Enemy>>,
    mut player_query: Query<
        (
            &mut Energy,
            Option<&mut Mana>,
            Option<&mut ActiveSpell>,
            &StatusStore,
            &RelicStore,
            &mut Health,
            Option<&mut PlayerCombo>,
        ),
        With<Player>,
    >,
    mut player_block_query: Query<&mut Block, (With<Player>, Without<Enemy>)>,
    mut block_flash_query: Query<&mut BackgroundColor, With<BlockFlashUi>>,
    window_query: Query<&Window>,
    mut game_map: ResMut<GameMap>,
    time: Res<Time>,
    run_state: Res<RunState>,
    mut discard_pile: ResMut<DiscardPile>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Health,
            &mut Block,
            &mut StatusStore,
            &GlobalTransform,
            Option<&Selected>,
        ),
        (With<Enemy>, Without<Player>),
    >,
) {
    for ev in ev_play.read() {
        if let Ok(children) = hand_container_query.get_single() {
            if ev.hand_index < children.len() {
                let card_entity = children[ev.hand_index];
                if let Ok((card_data, transform)) = card_query.get(card_entity) {
                    // Pre-requisites (mana/energy)
                    let (mut energy, mut mana, _, _, _, _, _) =
                        if let Ok(e) = player_query.get_single_mut() {
                            e
                        } else {
                            continue;
                        };

                    if run_state.character_class == CharacterClass::Spellweaver {
                        if let Some(ref mut m) = mana {
                            if m.current < card_data.cost {
                                println!("Not enough mana!");
                                continue;
                            }
                            m.current -= card_data.cost;
                        }
                    } else {
                        if energy.current < card_data.cost {
                            println!("Not enough energy!");
                            continue;
                        }
                        energy.current -= card_data.cost;
                    }

                    // For targeted spells, CLI overrides visual selected
                    // If target wasn't explicitly provided by CLI, standard system logic will handle it by just using `None` and resolving to 'Selected' later

                    let window = window_query.single();
                    let half_w = window.width() / 2.0;
                    let half_h = window.height() / 2.0;
                    let start_world = transform.translation().truncate();
                    let start_pos = Vec2::new(start_world.x + half_w, start_world.y);
                    let target_pos = Vec2::new(half_w, half_h);

                    commands
                        .entity(card_entity)
                        .remove::<Interaction>()
                        .remove::<Parent>()
                        .insert(CardAnimating {
                            start: start_pos,
                            target: target_pos,
                            timer: Timer::from_seconds(0.4, TimerMode::Once),
                            card: card_data.clone(),
                            target_index: ev.target_index,
                        })
                        .insert(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(start_pos.x - 60.0),
                            bottom: Val::Px(start_pos.y - 90.0),
                            width: Val::Px(120.0),
                            height: Val::Px(180.0),
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .insert(ZIndex::Global(900));
                }
            }
        }
    }

    // 2. Handle Animations
    for (entity, mut style, mut anim) in animating_query.iter_mut() {
        anim.timer.tick(time.delta());
        let t = anim.timer.fraction();
        let t_eased = 1.0 - (1.0 - t).powi(3);

        let current = anim.start.lerp(anim.target, t_eased);
        style.left = Val::Px(current.x - 60.0);
        style.bottom = Val::Px(current.y - 90.0);

        if anim.timer.finished() {
            let card_data = &anim.card;
            println!("Player plays: {}", card_data.name);

            let (
                _,
                _,
                mut active_spell,
                player_status,
                player_relics,
                mut player_health,
                mut player_combo,
            ) = if let Ok(p) = player_query.get_single_mut() {
                p
            } else {
                continue;
            };

            // Handle Essence (Modifiers)
            if card_data.is_spell_modifier {
                if let Some(ref mut spell) = active_spell {
                    // Check for cancellation
                    let opposite = match card_data.element {
                        SpellElement::Fire => SpellElement::Ice,
                        SpellElement::Ice => SpellElement::Fire,
                        SpellElement::Wind => SpellElement::Stone,
                        SpellElement::Stone => SpellElement::Wind,
                        _ => SpellElement::Neutral,
                    };

                    let mut cancelled = false;
                    if opposite != SpellElement::Neutral {
                        if let Some(idx) = spell.essences.iter().position(|e| e.element == opposite)
                        {
                            // Remove the opposite essence
                            let removed = spell.essences.remove(idx);
                            spell.bonus_damage -= removed.damage;
                            spell.bonus_block -= removed.block;
                            cancelled = true;
                            println!(
                                "Essence Cancelled! Removed {:?} due to {:?}",
                                opposite, card_data.element
                            );

                            // Update main element if needed
                            if let Some(last) = spell.essences.last() {
                                spell.element = last.element;
                            } else {
                                spell.element = SpellElement::Neutral;
                            }
                        }
                    }

                    if !cancelled {
                        spell.bonus_damage += card_data.damage;
                        spell.bonus_block += card_data.block;
                        spell.element = card_data.element;
                        spell.essences.push(EssenceInfo {
                            element: card_data.element,
                            damage: card_data.damage,
                            block: card_data.block,
                        });
                        spell.essence_history.push(card_data.element);

                        let history_len = spell.essence_history.len();
                        if history_len >= 3 {
                            let third = spell.essence_history[history_len - 1].clone();
                            let second = spell.essence_history[history_len - 2].clone();
                            let first = spell.essence_history[history_len - 3].clone();

                            if first == SpellElement::Fire
                                && second == SpellElement::Ice
                                && third == SpellElement::Wind
                            {
                                spell.bonus_damage += 20;
                                println!("✨ Elemental Combo (Thermal Updraft)! +20 Damage!");
                                spell.essence_history.clear();
                            } else if first == SpellElement::Wind
                                && second == SpellElement::Fire
                                && third == SpellElement::Stone
                            {
                                spell.bonus_block += 20;
                                println!("✨ Elemental Combo (Volcanic Ash)! +20 Block!");
                                spell.essence_history.clear();
                            } else if first != second
                                && second != third
                                && first != third
                                && first != SpellElement::Neutral
                                && second != SpellElement::Neutral
                                && third != SpellElement::Neutral
                            {
                                spell.bonus_damage += 10;
                                spell.bonus_block += 10;
                                println!("✨ Elemental Convergence! +10 Damage, +10 Block!");
                                spell.essence_history.clear();
                            } else {
                                spell.essence_history.remove(0);
                            }
                        }
                        println!(
                            "Essence Added! Current Bonus: +{} Dmg / +{} Blk ({:?})",
                            spell.bonus_damage, spell.bonus_block, spell.element
                        );
                    }
                }

                discard_pile.cards.push(card_data.clone());
                commands.entity(entity).despawn_recursive();
                continue;
            }

            let mut final_damage = card_data.damage;
            let mut final_block = card_data.block;

            if let Some(mut combo) = player_combo {
                if card_data.finisher_combo_cost > 0 {
                    if combo.current >= card_data.finisher_combo_cost {
                        combo.current -= card_data.finisher_combo_cost;
                        final_damage *= 2;
                        println!(
                            "Finisher triggered! Consumed {} Combo, Final Damage: {}",
                            card_data.finisher_combo_cost, final_damage
                        );
                    } else {
                        println!("Not enough Combo points for Finisher.");
                    }
                }

                if card_data.combo_points_granted > 0 {
                    combo.current += card_data.combo_points_granted;
                    println!(
                        "Gained {} Combo points, total: {}",
                        card_data.combo_points_granted, combo.current
                    );
                }
            }

            // Spell Flags
            let mut spell_has_fire = false;
            let mut spell_has_ice = false;
            let mut spell_has_wind = false;
            let mut spell_has_stone = false;

            if card_data.is_spell_source {
                if let Some(ref mut spell) = active_spell {
                    final_damage += spell.bonus_damage;
                    final_block += spell.bonus_block;

                    for essence in &spell.essences {
                        match essence.element {
                            SpellElement::Fire => spell_has_fire = true,
                            SpellElement::Ice => spell_has_ice = true,
                            SpellElement::Wind => spell_has_wind = true,
                            SpellElement::Stone => spell_has_stone = true,
                            _ => {}
                        }
                    }
                    println!(
                        "Spell Cast! Total: {} Dmg / {} Blk",
                        final_damage, final_block
                    );

                    // Consume Essence
                    **spell = ActiveSpell::default();
                }
            }

            if final_block > 0 {
                if let Ok(mut block) = player_block_query.get_single_mut() {
                    let mut block_gain = final_block;
                    if player_relics.relics.contains(&Relic::OddlySmoothStone) {
                        block_gain += 1;
                    }
                    block.value += block_gain;
                    println!("Player gains {} block", block_gain);
                    for mut bg in &mut block_flash_query {
                        bg.0 = Color::srgba(0.0, 0.5, 1.0, 0.3).into();
                    }
                }
            }

            // Spawn Particles
            let pos = anim.target;
            for _ in 0..20 {
                let mut rng = thread_rng();
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(50.0..300.0);
                let vx = angle.cos() * speed;
                let vy = angle.sin() * speed;

                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(pos.x),
                            bottom: Val::Px(pos.y),
                            width: Val::Px(6.0),
                            height: Val::Px(6.0),
                            ..default()
                        },
                        background_color: Color::srgb(1.0, 0.9, 0.5).into(),
                        z_index: ZIndex::Global(950),
                        ..default()
                    },
                    Particle {
                        velocity: Vec2::new(vx, vy),
                        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                    },
                    BattleEntity,
                ));
            }

            // Identify Targets
            let mut target_entities = Vec::new();
            if spell_has_wind {
                for (e, _, _, _, _, _) in enemy_query.iter() {
                    target_entities.push(e);
                }
            } else {
                if let Some(explicit_idx) = anim.target_index {
                    let mut i = 0;
                    for (e, _, _, _, _, _) in enemy_query.iter() {
                        if i == explicit_idx {
                            target_entities.push(e);
                            break;
                        }
                        i += 1;
                    }
                } else {
                    if let Some((e, _, _, _, _, _)) =
                        enemy_query.iter().find(|(_, _, _, _, _, s)| s.is_some())
                    {
                        target_entities.push(e);
                    }
                }
            }

            let mut damage = final_damage;
            let (_, _, _, player_status, player_relics, _, _) =
                if let Ok(p) = player_query.get_single_mut() {
                    p
                } else {
                    continue;
                };

            if player_relics.relics.contains(&Relic::Vajra) && damage > 0 {
                damage += 1;
            }
            if player_status.strength > 0 && damage > 0 {
                damage += player_status.strength;
            }
            if player_status.weak > 0 {
                damage = (damage as f32 * 0.75) as i32;
            }

            if damage > 0 && !target_entities.is_empty() {
                // Spawn Offensive Reflex
                if let Ok(window) = window_query.get_single() {
                    let cx = window.width() / 2.0;
                    let cy = window.height() / 2.0;

                    // Container
                    commands
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(cx - 200.0),
                                    bottom: Val::Px(cy - 20.0),
                                    width: Val::Px(400.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                border_color: Color::WHITE.into(),
                                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                                ..default()
                            },
                            BattleEntity,
                        ))
                        .with_children(|parent| {
                            // Perfect Window Indicator (static)
                            parent.spawn(NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(70.0),  // 0.7
                                    width: Val::Percent(10.0), // 0.8 - 0.7 = 0.1
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: Color::srgba(0.0, 1.0, 0.0, 0.3).into(),
                                ..default()
                            });

                            // Moving Slider
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        width: Val::Px(10.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                ReflexState {
                                    start_time: 0.0,
                                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                                    perfect_window_start: 0.7,
                                    perfect_window_end: 0.8,
                                    result: None,
                                    is_defensive: false,
                                    source_entity: None,
                                    target_entity: Some(target_entities[0]),
                                    base_damage: damage,
                                    visual_type: ReflexVisualType::LinearSlider,
                                },
                                ReflexUI,
                            ));
                        });
                }

                let mut elements = Vec::new();
                if spell_has_fire {
                    elements.push(SpellElement::Fire);
                }
                if spell_has_ice {
                    elements.push(SpellElement::Ice);
                }
                if spell_has_wind {
                    elements.push(SpellElement::Wind);
                }
                if spell_has_stone {
                    elements.push(SpellElement::Stone);
                }

                commands.spawn((
                    PendingPlayerAttack {
                        targets: target_entities.clone(),
                        damage,
                        card: card_data.clone(),
                        spell_elements: elements,
                    },
                    BattleEntity,
                ));

                next_turn_state.set(TurnState::PlayerAttackAnimating);
            } else {
                // Apply non-damage statuses immediately
                for target_entity in target_entities {
                    if let Ok((_, _, _, mut enemy_status, _, _)) =
                        enemy_query.get_mut(target_entity)
                    {
                        if card_data.apply_poison > 0 {
                            enemy_status.poison += card_data.apply_poison;
                        }
                        if card_data.apply_weak > 0 {
                            enemy_status.weak += card_data.apply_weak;
                        }
                        if card_data.apply_stun > 0 {
                            enemy_status.stun += card_data.apply_stun;
                        }
                    }
                }
            }

            discard_pile.cards.push(card_data.clone());
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn enemy_selection_system(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut BorderColor), With<Enemy>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let mut new_selection = None;

    for (entity, interaction) in &interaction_query {
        if *interaction == Interaction::Pressed {
            new_selection = Some(entity);
            break;
        }
    }

    if let Some(new_entity) = new_selection {
        // Deselect current
        if let Ok(current) = selected_query.get_single() {
            commands.entity(current).remove::<Selected>();
            if let Ok((_, mut border)) = enemy_query.get_mut(current) {
                *border = Color::srgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }

        // Select new
        commands.entity(new_entity).insert(Selected);
        if let Ok((_, mut border)) = enemy_query.get_mut(new_entity) {
            *border = Color::srgb(1.0, 1.0, 0.0).into();
        }
    } else if selected_query.is_empty() {
        // Auto-select first available if none selected (e.g. previous died)
        if let Some((entity, mut border)) = enemy_query.iter_mut().next() {
            commands.entity(entity).insert(Selected);
            *border = Color::srgb(1.0, 1.0, 0.0).into();
        }
    }
}

pub fn update_potion_ui(
    mut commands: Commands,
    player_potion_query: Query<&PotionStore, (With<Player>, Changed<PotionStore>)>,
    potion_ui_query: Query<Entity, With<PotionContainer>>,
    player_potions_all: Query<&PotionStore, With<Player>>,
    ui_added: Query<Entity, Added<PotionContainer>>,
) {
    let spawn_potions = |parent: &mut ChildBuilder, potions: &PotionStore| {
        for (index, potion) in potions.potions.iter().enumerate() {
            let (text, tooltip, color) = get_potion_visuals(potion);

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        background_color: color.into(),
                        border_color: Color::WHITE.into(),
                        ..default()
                    },
                    PotionButton { index },
                    Tooltip {
                        text: tooltip.to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
    };

    if !player_potion_query.is_empty() || !ui_added.is_empty() {
        if let (Ok(potions), Ok(ui_entity)) = (
            player_potions_all.get_single(),
            potion_ui_query.get_single(),
        ) {
            commands.entity(ui_entity).despawn_descendants();
            commands
                .entity(ui_entity)
                .with_children(|parent| spawn_potions(parent, potions));
        }
    }
}

pub fn potion_interaction_system(
    mut player_query: Query<
        (&mut Health, &mut Energy, &mut StatusStore, &mut PotionStore),
        With<Player>,
    >,
    interaction_query: Query<
        (&Interaction, &PotionButton),
        (Changed<Interaction>, With<PotionButton>),
    >,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((mut health, mut energy, mut status, mut potions)) =
                player_query.get_single_mut()
            {
                if button.index < potions.potions.len() {
                    let potion_type = potions.potions.remove(button.index);

                    match potion_type {
                        Potion::Health => {
                            health.current = (health.current + 10).min(health.max);
                            println!("Used Health Potion: +10 HP");
                        }
                        Potion::Energy => {
                            energy.current += 2;
                            println!("Used Energy Potion: +2 Energy");
                        }
                        Potion::Strength => {
                            status.strength += 2;
                            println!("Used Strength Potion: +2 Strength");
                        }
                    }
                    // PotionStore Changed event will trigger UI update
                }
            }
        }
    }
}

pub fn card_hover_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &BaseColor),
        (Changed<Interaction>, With<Card>),
    >,
) {
    for (interaction, mut bg_color, base_color) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.8, 0.8, 0.8).into();
            }
            Interaction::None => {
                *bg_color = base_color.0.into();
            }
            _ => {}
        }
    }
}

pub fn update_particles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Style, &mut BackgroundColor, &mut Particle)>,
) {
    for (entity, mut style, mut bg_color, mut particle) in &mut query {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            if let (Val::Px(x), Val::Px(y)) = (style.left, style.bottom) {
                style.left = Val::Px(x + particle.velocity.x * time.delta_seconds());
                style.bottom = Val::Px(y + particle.velocity.y * time.delta_seconds());
            }
            let alpha = particle.lifetime.fraction_remaining();
            bg_color.0.set_alpha(alpha);
        }
    }
}

pub fn update_damage_flash_system(
    time: Res<Time>,
    mut query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
) {
    for mut bg in &mut query {
        let a = bg.0.alpha();
        if a > 0.0 {
            bg.0.set_alpha((a - time.delta_seconds() * 2.0).max(0.0));
        }
    }
}

pub fn update_block_flash_system(
    time: Res<Time>,
    mut query: Query<&mut BackgroundColor, With<BlockFlashUi>>,
) {
    for mut bg in &mut query {
        let a = bg.0.alpha();
        if a > 0.0 {
            bg.0.set_alpha((a - time.delta_seconds() * 2.0).max(0.0));
        }
    }
}

pub fn reset_turn_state(mut next_turn_state: ResMut<NextState<TurnState>>) {
    next_turn_state.set(TurnState::Setup);
}

pub fn update_status_visuals_system(
    mut commands: Commands,
    player_query: Query<&StatusStore, (With<Player>, Changed<StatusStore>)>,
    enemy_query: Query<(&StatusStore, &Children), (With<Enemy>, Changed<StatusStore>)>,
    player_status_ui: Query<Entity, With<PlayerStatusText>>,
    enemy_status_ui_query: Query<Entity, With<EnemyStatusText>>,
    player_status_all: Query<&StatusStore, With<Player>>,
    player_ui_added: Query<Entity, Added<PlayerStatusText>>,
    enemy_ui_added: Query<(Entity, &Parent), Added<EnemyStatusText>>,
) {
    let spawn_badges = |parent: &mut ChildBuilder, status: &StatusStore| {
        if status.poison > 0 {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.2, 0.8, 0.2, 0.5).into(),
                        border_color: Color::srgb(0.2, 1.0, 0.2).into(),
                        ..default()
                    },
                    Interaction::None,
                    Tooltip {
                        text: "Poison: Takes damage at start of turn.".to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        format!("Psn {}", status.poison),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
        if status.weak > 0 {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.5, 0.5, 0.5, 0.5).into(),
                        border_color: Color::srgb(0.7, 0.7, 0.7).into(),
                        ..default()
                    },
                    Interaction::None,
                    Tooltip {
                        text: "Weak: Deal 25% less damage.".to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        format!("Wk {}", status.weak),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
        if status.stun > 0 {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.8, 0.8, 0.2, 0.5).into(),
                        border_color: Color::srgb(1.0, 1.0, 0.2).into(),
                        ..default()
                    },
                    Interaction::None,
                    Tooltip {
                        text: "Stunned: Cannot act this turn.".to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Stunned",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
        if status.burning > 0 {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(1.0, 0.4, 0.0, 0.5).into(),
                        border_color: Color::srgb(1.0, 0.6, 0.0).into(),
                        ..default()
                    },
                    Interaction::None,
                    Tooltip {
                        text: "Burning: Takes damage at start of turn.".to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        format!("Burn {}", status.burning),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
        if status.frozen > 0 {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::right(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.2, 0.8, 1.0, 0.5).into(),
                        border_color: Color::srgb(0.4, 0.9, 1.0).into(),
                        ..default()
                    },
                    Interaction::None,
                    Tooltip {
                        text: "Frozen: Deal 25% less damage.".to_string(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        format!("Frz {}", status.frozen),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: Handle::default(),
                        },
                    ));
                });
        }
    };

    if !player_query.is_empty() || !player_ui_added.is_empty() {
        if let (Ok(status), Ok(ui_entity)) = (
            player_status_all.get_single(),
            player_status_ui.get_single(),
        ) {
            if commands.get_entity(ui_entity).is_some() {
                commands.entity(ui_entity).despawn_descendants();
                commands
                    .entity(ui_entity)
                    .with_children(|parent| spawn_badges(parent, status));
            }
        }
    }

    // Update changed enemies
    for (status, children) in enemy_query.iter() {
        for &child in children.iter() {
            if let Ok(ui_entity) = enemy_status_ui_query.get(child) {
                // IMPORTANT: The parent enemy might be dead this exact frame,
                // in which case we MUST NOT touch its children or UI elements.
                if commands.get_entity(ui_entity).is_some() && commands.get_entity(child).is_some()
                {
                    commands.entity(ui_entity).despawn_descendants();
                    commands
                        .entity(ui_entity)
                        .with_children(|parent| spawn_badges(parent, status));
                }
            }
        }
    }

    // Initialize added UI
    for (ui_entity, parent) in enemy_ui_added.iter() {
        if commands.get_entity(ui_entity).is_none() || commands.get_entity(parent.get()).is_none() {
            continue;
        }
        if let Ok((status, _)) = enemy_query.get(parent.get()) {
            commands.entity(ui_entity).despawn_descendants();
            commands
                .entity(ui_entity)
                .with_children(|parent| spawn_badges(parent, status));
        }
    }
}

pub fn tooltip_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &Tooltip), Changed<Interaction>>,
    tooltip_ui_query: Query<Entity, With<TooltipUi>>,
) {
    for (interaction, tooltip) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                for e in tooltip_ui_query.iter() {
                    commands.entity(e).despawn_recursive();
                }
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(10.0),
                                left: Val::Percent(50.0),
                                margin: UiRect::left(Val::Px(-100.0)),
                                width: Val::Px(200.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgba(0.1, 0.1, 0.1, 0.95).into(),
                            border_color: Color::WHITE.into(),
                            z_index: ZIndex::Global(200),
                            ..default()
                        },
                        TooltipUi,
                    ))
                    .with_children(|p| {
                        p.spawn(TextBundle::from_section(
                            &tooltip.text,
                            TextStyle {
                                font_size: 18.0,
                                color: Color::WHITE,
                                font: Handle::default(),
                            },
                        ));
                    });
            }
            Interaction::None => {
                for e in tooltip_ui_query.iter() {
                    commands.entity(e).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

pub fn cleanup_battle_deck(
    mut deck: ResMut<Deck>,
    mut discard: ResMut<DiscardPile>,
    card_query: Query<&Card>,
) {
    for card in card_query.iter() {
        deck.cards.push(card.clone());
    }

    deck.cards.append(&mut discard.cards);

    let mut rng = thread_rng();
    use rand::seq::SliceRandom;
    deck.cards.shuffle(&mut rng);
}

pub fn discard_pile_click_system(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<DiscardPileButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_turn_state.set(TurnState::ViewingDiscard);
        }
    }
}

pub fn reflex_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut ev_reflex: EventWriter<TriggerReflexRequest>,
) {
    if input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        ev_reflex.send(TriggerReflexRequest);
    }
}

pub fn process_reflex_requests(
    mut ev_reflex: EventReader<TriggerReflexRequest>,
    mut query: Query<&mut ReflexState>,
) {
    for _ in ev_reflex.read() {
        for mut reflex in query.iter_mut() {
            if reflex.result.is_none() {
                let ratio = reflex.timer.fraction();
                if ratio >= reflex.perfect_window_start && ratio <= reflex.perfect_window_end {
                    reflex.result = Some(ReflexSuccess::Perfect);
                } else if ratio >= reflex.perfect_window_start - 0.1
                    && ratio <= reflex.perfect_window_end + 0.1
                {
                    reflex.result = Some(ReflexSuccess::Good);
                } else {
                    reflex.result = Some(ReflexSuccess::Miss);
                }
            }
        }
    }
}

pub fn update_reflex_ui_system(
    mut query: Query<(&mut Style, &ReflexState), With<ReflexUI>>,
    window_query: Query<&Window>,
) {
    let window = if let Ok(w) = window_query.get_single() {
        w
    } else {
        return;
    };
    let cx = window.width() / 2.0;
    let cy = window.height() / 2.0;

    for (mut style, reflex) in query.iter_mut() {
        if reflex.visual_type == ReflexVisualType::ShrinkingRing {
            let max_size = 300.0;
            let mut current_size = max_size * (1.1 - reflex.timer.fraction());
            current_size = current_size.max(0.0);
            style.width = Val::Px(current_size);
            style.height = Val::Px(current_size);

            style.left = Val::Px(cx - current_size / 2.0);
            style.bottom = Val::Px(cy - 150.0 + (max_size - current_size) / 2.0);
        } else if reflex.visual_type == ReflexVisualType::LinearSlider {
            // Slider moves from left to right within its parent
            let max_width = 400.0; // Same as parent width
            style.left = Val::Px(max_width * reflex.timer.fraction());
        }
    }
}

pub fn player_attack_animating_system(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut reflex_query: Query<(Entity, Option<&Parent>, &mut ReflexState)>,
    pending_query: Query<(Entity, &PendingPlayerAttack)>,
    mut enemy_query: Query<(Entity, &mut Health, &mut Block, &mut StatusStore), With<Enemy>>,
    mut player_query: Query<(&RelicStore, &mut Health), (With<Player>, Without<Enemy>)>,
    mut game_map: ResMut<GameMap>,
    time: Res<Time>,
    window_query: Query<&Window>,
    relic_ui_query: Query<(&RelicIcon, &GlobalTransform)>,
) {
    if let Ok((reflex_entity, reflex_parent, mut reflex)) = reflex_query.get_single_mut() {
        reflex.timer.tick(time.delta());

        if reflex.result.is_some() || reflex.timer.finished() {
            let multiplier = match reflex.result {
                Some(ReflexSuccess::Perfect) => 1.5,
                Some(ReflexSuccess::Good) => 1.25,
                _ => 1.0,
            };

            if let Ok((pending_entity, pending_attack)) = pending_query.get_single() {
                let final_damage = (pending_attack.damage as f32 * multiplier) as i32;
                println!(
                    "Offensive Reflex result: {:?} (x{}), Final Damage: {}",
                    reflex.result, multiplier, final_damage
                );

                let spell_has_fire = pending_attack.spell_elements.contains(&SpellElement::Fire);
                let spell_has_ice = pending_attack.spell_elements.contains(&SpellElement::Ice);
                let spell_has_stone = pending_attack.spell_elements.contains(&SpellElement::Stone);
                let spell_has_wind = pending_attack.spell_elements.contains(&SpellElement::Wind);

                let mut burning_snapshot = Vec::new();
                if spell_has_wind {
                    for (e, _, _, status) in enemy_query.iter() {
                        if status.burning > 0 && pending_attack.targets.contains(&e) {
                            burning_snapshot.push((e, status.burning));
                        }
                    }
                }

                let mut enemies_remaining = enemy_query.iter().count();
                for target_entity in &pending_attack.targets {
                    if let Ok((e, mut hp, mut block, mut status)) =
                        enemy_query.get_mut(*target_entity)
                    {
                        // Statuses apply
                        if pending_attack.card.apply_poison > 0 {
                            status.poison += pending_attack.card.apply_poison;
                        }
                        if pending_attack.card.apply_weak > 0 {
                            status.weak += pending_attack.card.apply_weak;
                        }
                        if pending_attack.card.apply_stun > 0 {
                            status.stun += pending_attack.card.apply_stun;
                        }

                        let mut target_dmg = final_damage;

                        if spell_has_ice && status.burning > 0 {
                            status.burning = 0;
                            println!("Extinguished!");
                        }
                        if spell_has_fire && status.frozen > 0 {
                            status.frozen = 0;
                            println!("Melted!");
                        }
                        if spell_has_stone && status.frozen > 0 {
                            status.frozen = 0;
                            target_dmg *= 2;
                            println!("Shattered! Double Damage!");
                        }
                        if spell_has_wind {
                            for (source_entity, amount) in &burning_snapshot {
                                if *source_entity != e {
                                    status.burning += amount;
                                    println!("Burning spread to {:?}", e);
                                }
                            }
                        }

                        let block_dmg = std::cmp::min(target_dmg, block.value);
                        block.value -= block_dmg;
                        hp.current -= target_dmg - block_dmg;
                        println!("Dealt {} damage to {:?}", target_dmg, target_entity);

                        // Threshold Status checks
                        if target_dmg > 10 {
                            if spell_has_fire {
                                status.burning += 3;
                            }
                            if spell_has_ice {
                                status.frozen += 2;
                            }
                            if spell_has_stone {
                                status.stun += 1;
                            }
                        }

                        if hp.current <= 0 {
                            commands.entity(e).despawn_recursive();
                            enemies_remaining -= 1;
                        }
                    }
                }

                if enemies_remaining == 0 {
                    if let Ok((player_relics, mut player_health)) = player_query.get_single_mut() {
                        if player_relics.relics.contains(&Relic::BurningBlood) {
                            player_health.current =
                                (player_health.current + 6).min(player_health.max);
                            println!("Burning Blood heals 6 HP!");

                            // Visuals
                            if let Ok(window) = window_query.get_single() {
                                for (icon, transform) in &relic_ui_query {
                                    if icon.relic == Relic::BurningBlood {
                                        let half_w = window.width() / 2.0;
                                        let half_h = window.height() / 2.0;
                                        let pos = transform.translation();

                                        for _ in 0..20 {
                                            let mut rng = rand::thread_rng();
                                            use rand::Rng;
                                            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                                            let speed = rng.gen_range(30.0..100.0);
                                            let vx = angle.cos() * speed;
                                            let vy = angle.sin() * speed;

                                            commands.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        position_type: PositionType::Absolute,
                                                        left: Val::Px(pos.x + half_w),
                                                        bottom: Val::Px(pos.y + half_h),
                                                        width: Val::Px(4.0),
                                                        height: Val::Px(4.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::srgb(1.0, 0.2, 0.2)
                                                        .into(),
                                                    z_index: ZIndex::Global(200),
                                                    ..default()
                                                },
                                                Particle {
                                                    velocity: Vec2::new(vx, vy),
                                                    lifetime: Timer::from_seconds(
                                                        0.8,
                                                        TimerMode::Once,
                                                    ),
                                                },
                                                BattleEntity,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some((level, index)) = game_map.current_node {
                        if !game_map.visited_path.contains(&(level, index)) {
                            game_map.visited_path.push((level, index));
                        }
                        let visited = game_map.visited_path.clone();
                        for l in 0..=level {
                            for (i, node) in game_map.levels[l].iter_mut().enumerate() {
                                if !visited.contains(&(l, i)) {
                                    node.visible = false;
                                }
                            }
                        }
                        if level + 1 < game_map.levels.len() {
                            let next_indices = game_map.levels[level][index].next_indices.clone();
                            for next_idx in next_indices {
                                game_map.levels[level + 1][next_idx].visible = true;
                            }
                        }
                    }

                    next_game_state.set(GameState::Victory);
                } else {
                    next_turn_state.set(TurnState::PlayerTurn);
                }

                commands.entity(pending_entity).despawn_recursive();
            }

            if let Some(parent) = reflex_parent {
                commands.entity(parent.get()).despawn_recursive();
            } else {
                commands.entity(reflex_entity).despawn_recursive();
            }
        }
    }
}

pub fn update_combo_ui_system(
    player_query: Query<&PlayerCombo, With<Player>>,
    mut text_query: Query<&mut Text, With<PlayerComboText>>,
) {
    if let Ok(combo) = player_query.get_single() {
        for mut text in &mut text_query {
            if combo.current > 0 {
                text.sections[0].value = format!("Combo: {}", combo.current);
            } else {
                text.sections[0].value = "Combo: 0".to_string();
            }
        }
    }
}

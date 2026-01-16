use bevy::prelude::*;
use rand::thread_rng;
use rand::Rng;

use crate::components::*;
use crate::resources::*;
use crate::states::*;

#[derive(Component)]
pub struct DamageFlashUi;

pub fn setup_battle(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    game_map: Res<GameMap>,
    deck: Res<Deck>,
    discard: Res<DiscardPile>,
    mut reward_store: ResMut<RewardStore>,
) {
    println!("Setting up battle...");

    // Determine Enemy Type
    let level = game_map.current_node.map(|(l, _)| l).unwrap_or(0);
    let mut rng = thread_rng();
    let enemy_kind = if level == 5 {
        EnemyKind::Dragon
    } else if level >= 3 {
        if rng.gen_bool(0.7) { EnemyKind::Orc } else { EnemyKind::Goblin }
    } else {
        if rng.gen_bool(0.9) { EnemyKind::Goblin } else { EnemyKind::Orc }
    };
    let (hp, name, color) = match enemy_kind {
        EnemyKind::Goblin => (20, "Goblin", Color::srgb(0.2, 0.8, 0.2)),
        EnemyKind::Orc => (40, "Orc", Color::srgb(0.8, 0.2, 0.2)),
        EnemyKind::Dragon => (150, "Dragon", Color::srgb(0.6, 0.1, 0.8)),
    };

    // Spawn Enemy with Visuals
    commands.spawn((
        Enemy { kind: enemy_kind },
        Health { current: hp, max: hp },
        Block { value: 0 },
        StatusStore::default(),
        BattleEntity,
        Interaction::None,
        Tooltip { text: String::new() },
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                right: Val::Px(50.0),
                width: Val::Px(150.0),
                height: Val::Px(150.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: color.into(),
            border_color: Color::WHITE.into(),
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(name, TextStyle {
            font: Handle::default(),
            font_size: 30.0,
            color: Color::WHITE,
        }));
        
        parent.spawn((
            TextBundle::from_section("Planning...", TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::srgb(1.0, 1.0, 0.0),
            }),
            EnemyIntentText,
        ));
    });

    // Spawn a "Hand" UI container
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }, BattleEntity, HandContainer));

    // Spawn End Turn Button
    commands.spawn((
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
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("End Turn", TextStyle {
            font: Handle::default(),
            font_size: 20.0,
            color: Color::WHITE,
        }));
    });

    // Spawn Player Stats Panel (Bottom Left)
    commands.spawn((
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
    )).with_children(|parent| {
        // Player Health
        parent.spawn((
            TextBundle::from_section("Player: 50/50", TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            PlayerHealthText,
        ));
        // Player Block
        parent.spawn((
            TextBundle::from_section("Block: 0", TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::srgb(0.5, 0.5, 1.0),
            }),
            PlayerBlockText,
        ));
        // Player Energy
        parent.spawn((
            TextBundle::from_section("Energy: 3/3", TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::srgb(0.2, 0.8, 1.0),
            }),
            PlayerEnergyText,
        ));
        // Player Gold
        parent.spawn((
            TextBundle::from_section("Gold: 0", TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::srgb(1.0, 0.84, 0.0),
            }),
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

    // Spawn Enemy Stats Panel (Top Right)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(220.0),
                top: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            border_color: Color::srgb(0.8, 0.2, 0.2).into(),
            ..default()
        },
        BattleEntity,
    )).with_children(|parent| {
        // Enemy Health
        parent.spawn((
            TextBundle::from_section(format!("Enemy: {}/{}", hp, hp), TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::srgb(0.9, 0.3, 0.3),
            }),
            EnemyHealthText,
        ));
        // Enemy Block
        parent.spawn((
            TextBundle::from_section("Block: 0", TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::srgb(0.5, 0.5, 1.0),
            }),
            EnemyBlockText,
        ));
        // Enemy Status
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
            EnemyStatusText,
        ));
    });

    // Spawn Deck UI
    commands.spawn((
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
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(format!("Deck: {}", deck.cards.len()), TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            DeckText,
        ));
    });

    // Spawn Discard UI
    commands.spawn((
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
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(format!("Discard: {}", discard.cards.len()), TextStyle {
                font: Handle::default(),
                font_size: 24.0,
                color: Color::WHITE,
            }),
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

    // Reset Reward Store for the new battle
    *reward_store = RewardStore::default();

    next_turn_state.set(TurnState::PlayerTurnStart);
}

pub fn draw_cards_system(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut deck: ResMut<Deck>,
    mut discard: ResMut<DiscardPile>,
    hand_container_query: Query<Entity, With<HandContainer>>,
    mut player_query: Query<(&mut Energy, &mut Block, &mut Health, &mut StatusStore), With<Player>>,
    mut flash_query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
) {
    if let Ok((mut energy, mut block, mut health, mut status)) = player_query.get_single_mut() {
        // Reset Energy
        energy.current = energy.max;
        // Reset Block
        block.value = 0;
        
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
        commands.entity(container).with_children(|parent| {
            let card_style = Style {
                width: Val::Px(100.0),
                height: Val::Px(150.0),
                margin: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            };
            let text_style = TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::WHITE,
            };

            for card in hand_cards {
                let color = if card.name == "Strike" {
                    Color::srgb(0.5, 0.1, 0.1)
                } else {
                    Color::srgb(0.1, 0.1, 0.5)
                };

                parent.spawn((
                    card.clone(),
                    BaseColor(color),
                    ButtonBundle {
                        style: card_style.clone(),
                        background_color: color.into(),
                        border_color: Color::WHITE.into(),
                        ..default()
                    },
                )).with_children(|p| {
                p.spawn(TextBundle::from_section(format!("Cost: {}", card.cost), text_style.clone()));
                    if card.damage > 0 {
                        p.spawn(TextBundle::from_section(format!("Dmg: {}", card.damage), text_style.clone()));
                    }
                    if card.block > 0 {
                        p.spawn(TextBundle::from_section(format!("Blk: {}", card.block), text_style.clone()));
                    }
                    if card.apply_poison > 0 {
                        p.spawn(TextBundle::from_section(format!("Psn: {}", card.apply_poison), text_style.clone()));
                    }
                    if card.apply_weak > 0 {
                        p.spawn(TextBundle::from_section(format!("Wk: {}", card.apply_weak), text_style.clone()));
                    }
                    p.spawn(TextBundle::from_section(card.name.clone(), text_style.clone()));
                });
            }
        });
    }

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
        if status.weak > 0 { status.weak -= 1; }
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

pub fn update_enemy_tooltip_system(
    mut query: Query<(&Enemy, &StatusStore, &mut Tooltip), Or<(Added<Enemy>, Changed<StatusStore>)>>,
) {
    for (enemy, status, mut tooltip) in query.iter_mut() {
        let (base_damage, move_name) = match enemy.kind {
            EnemyKind::Goblin => (5, "Stabs"),
            EnemyKind::Orc => (12, "Smashes"),
            EnemyKind::Dragon => (25, "Incinerates"),
        };
        
        let mut final_damage = base_damage;
        if status.weak > 0 {
            final_damage = (final_damage as f32 * 0.75) as i32;
        }
        
        tooltip.text = format!("Intent: {}\nDamage: {} (Base: {})", move_name, final_damage, base_damage);
    }
}

pub fn play_card_system(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut discard_pile: ResMut<DiscardPile>,
    card_query: Query<(Entity, &Card, &Interaction), Changed<Interaction>>,
    mut enemy_query: Query<(&mut Health, &mut Block, &mut StatusStore), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&mut Energy, &StatusStore, &RelicStore, &mut Health), With<Player>>,
    mut player_block_query: Query<&mut Block, (With<Player>, Without<Enemy>)>,
) {
    for (card_entity, card_data, interaction) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            let (mut energy, player_status, player_relics, mut player_health) = if let Ok(e) = player_query.get_single_mut() { e } else { continue };
            if energy.current < card_data.cost {
                println!("Not enough energy!");
                continue;
            }
            energy.current -= card_data.cost;

            if card_data.block > 0 {
                if let Ok(mut block) = player_block_query.get_single_mut() {
                    block.value += card_data.block;
                    println!("Player gains {} block", card_data.block);
                }
            }

            println!("Player plays: {}", card_data.name);

            for (mut enemy_health, mut enemy_block, mut enemy_status) in enemy_query.iter_mut() {
                if card_data.apply_poison > 0 { enemy_status.poison += card_data.apply_poison; }
                if card_data.apply_weak > 0 { enemy_status.weak += card_data.apply_weak; }

                let mut damage = card_data.damage;
                if player_relics.relics.contains(&Relic::Vajra) && damage > 0 {
                    damage += 1;
                }
                if player_status.strength > 0 && damage > 0 {
                    damage += player_status.strength;
                }
                if player_status.weak > 0 {
                    damage = (damage as f32 * 0.75) as i32;
                }

                let block_damage = std::cmp::min(damage, enemy_block.value);
                enemy_block.value -= block_damage;
                enemy_health.current -= damage - block_damage;
                println!("Enemy Health: {}/{}", enemy_health.current, enemy_health.max);

                if enemy_health.current <= 0 {
                    println!("Enemy Defeated!");
                    if player_relics.relics.contains(&Relic::BurningBlood) {
                        player_health.current = (player_health.current + 6).min(player_health.max);
                        println!("Burning Blood heals 6 HP!");
                    }
                    next_game_state.set(GameState::Victory);
                    return;
                }
            }

            discard_pile.cards.push(card_data.clone());
            commands.entity(card_entity).despawn_recursive();
            break;
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
            let (text, tooltip, color) = match potion {
                PotionType::Health => ("HP", "Health Potion: Heal 10 HP.", Color::srgb(0.8, 0.2, 0.2)),
                PotionType::Strength => ("STR", "Strength Potion: +2 Strength.", Color::srgb(0.8, 0.4, 0.0)),
                PotionType::Energy => ("NRG", "Energy Potion: +2 Energy.", Color::srgb(0.2, 0.8, 0.8)),
            };

            parent.spawn((
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
                Tooltip { text: tooltip.to_string() },
            )).with_children(|p| {
                p.spawn(TextBundle::from_section(text, TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    font: Handle::default(),
                }));
            });
        }
    };

    if !player_potion_query.is_empty() || !ui_added.is_empty() {
        if let (Ok(potions), Ok(ui_entity)) = (player_potions_all.get_single(), potion_ui_query.get_single()) {
            commands.entity(ui_entity).despawn_descendants();
            commands.entity(ui_entity).with_children(|parent| spawn_potions(parent, potions));
        }
    }
}

pub fn potion_interaction_system(
    mut player_query: Query<(&mut Health, &mut Energy, &mut StatusStore, &mut PotionStore), With<Player>>,
    interaction_query: Query<(&Interaction, &PotionButton), (Changed<Interaction>, With<PotionButton>)>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((mut health, mut energy, mut status, mut potions)) = player_query.get_single_mut() {
                if button.index < potions.potions.len() {
                    let potion_type = potions.potions.remove(button.index);
                    
                    match potion_type {
                        PotionType::Health => {
                            health.current = (health.current + 10).min(health.max);
                            println!("Used Health Potion: +10 HP");
                        },
                        PotionType::Energy => {
                            energy.current += 2;
                            println!("Used Energy Potion: +2 Energy");
                        },
                        PotionType::Strength => {
                            status.strength += 2;
                            println!("Used Strength Potion: +2 Strength");
                        },
                    }
                    // PotionStore Changed event will trigger UI update
                }
            }
        }
    }
}

pub fn card_hover_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &BaseColor), (Changed<Interaction>, With<Card>)>,
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

pub fn enemy_turn_system(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Health, &mut Block, &RelicStore), With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Block, &mut Health, &mut StatusStore), Without<Player>>,
    mut intent_text_query: Query<&mut Text, With<EnemyIntentText>>,
    mut flash_query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
) {
    let (enemy, mut enemy_block, mut enemy_health, mut enemy_status) = enemy_query.single_mut();
    
    if enemy_status.poison > 0 {
        enemy_health.current -= enemy_status.poison;
        enemy_status.poison -= 1;
    }
    if enemy_health.current <= 0 {
        if let Ok((mut p_health, _, p_relics)) = player_query.get_single_mut() {
            if p_relics.relics.contains(&Relic::BurningBlood) {
                p_health.current = (p_health.current + 6).min(p_health.max);
            }
        }
        next_game_state.set(GameState::Victory);
        return;
    }
    
    enemy_block.value = 0;

    let (damage, action_desc) = match enemy.kind {
        EnemyKind::Goblin => (5, "Stabs"),
        EnemyKind::Orc => (12, "Smashes"),
        EnemyKind::Dragon => (25, "Incinerates"),
    };

    let mut final_damage = damage;
    if enemy_status.weak > 0 {
        final_damage = (final_damage as f32 * 0.75) as i32;
        enemy_status.weak -= 1;
    }

    println!("Enemy {}!", action_desc);
    
    for mut text in &mut intent_text_query {
        text.sections[0].value = format!("{}! ({} dmg)", action_desc, final_damage);
    }

    for (mut player_health, mut player_block, _) in player_query.iter_mut() {
        let block_damage = std::cmp::min(final_damage, player_block.value);
        let actual_damage = final_damage - block_damage;
        player_block.value -= block_damage;
        player_health.current -= actual_damage;
        println!("Player Health: {}/{}", player_health.current, player_health.max);
        if actual_damage > 0 {
            for mut bg in &mut flash_query {
                bg.0 = Color::srgba(1.0, 0.0, 0.0, 0.5).into();
            }
        }
        if player_health.current <= 0 {
            next_game_state.set(GameState::GameOver);
            return;
        }
    }

    next_turn_state.set(TurnState::PlayerTurnStart);
    println!("Player's turn.");
}

pub fn setup_victory_screen(mut commands: Commands, mut reward_store: ResMut<RewardStore>) {
    // Generate rewards if not already generated
    if !reward_store.generated {
        let mut rng = thread_rng();
        
        // Gold Reward
        reward_store.gold_reward = Some(rng.gen_range(20..=50));

        // Card Reward (3 random choices)
        let mut choices = Vec::new();
        for _ in 0..3 {
            choices.push(generate_random_card());
        }
        reward_store.card_choices = Some(choices);
        
        reward_store.generated = true;
    }

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        },
        RewardUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Enemy Defeated!", TextStyle {
            font: Handle::default(),
            font_size: 50.0,
            color: Color::srgb(1.0, 0.84, 0.0),
        }));
        
        parent.spawn(TextBundle::from_section("Choose Your Rewards", TextStyle {
            font: Handle::default(),
            font_size: 30.0,
            color: Color::WHITE,
        }));

        // Gold Reward Button
        if let Some(gold_amount) = reward_store.gold_reward {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    border_color: Color::srgb(1.0, 0.84, 0.0).into(),
                    ..default()
                },
                RewardGoldButton,
            )).with_children(|p| {
                p.spawn(TextBundle::from_section(format!("Gold: {}", gold_amount), TextStyle {
                    font: Handle::default(),
                    font_size: 25.0,
                    color: Color::srgb(1.0, 0.84, 0.0),
                }));
            });
        }

        // Card Reward Button
        if reward_store.card_choices.is_some() {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    border_color: Color::srgb(0.4, 0.4, 1.0).into(),
                    ..default()
                },
                RewardCardButton,
            )).with_children(|p| {
                p.spawn(TextBundle::from_section("Add a Card to Deck", TextStyle {
                    font: Handle::default(),
                    font_size: 25.0,
                    color: Color::WHITE,
                }));
            });
        }

        // Proceed Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(40.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            ProceedButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Proceed to Map", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

pub fn reward_interaction_system(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut reward_store: ResMut<RewardStore>,
    mut player_gold: Query<&mut Gold, With<Player>>,
    gold_btn_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<RewardGoldButton>)>,
    card_btn_query: Query<&Interaction, (Changed<Interaction>, With<RewardCardButton>)>,
    proceed_btn_query: Query<&Interaction, (Changed<Interaction>, With<ProceedButton>)>,
) {
    // Handle Gold Reward
    for (entity, interaction) in &gold_btn_query {
        if *interaction == Interaction::Pressed {
            if let Some(amount) = reward_store.gold_reward {
                if let Ok(mut gold) = player_gold.get_single_mut() {
                    gold.amount += amount;
                    reward_store.gold_reward = None;
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    // Handle Card Reward
    for interaction in &card_btn_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::RewardSelectCard);
        }
    }

    // Handle Proceed
    for interaction in &proceed_btn_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}

pub fn setup_reward_select_card_screen(mut commands: Commands, reward_store: Res<RewardStore>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.05, 0.98).into(),
            z_index: ZIndex::Global(100),
            ..default()
        },
        RewardSelectCardUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Select a Card", TextStyle {
            font: Handle::default(),
            font_size: 40.0,
            color: Color::WHITE,
        }));

        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(30.0)),
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        }).with_children(|cards_container| {
            if let Some(choices) = &reward_store.card_choices {
                for (index, card) in choices.iter().enumerate() {
                    let color = if card.name.contains("Strike") {
                        Color::srgb(0.5, 0.1, 0.1)
                    } else if card.name.contains("Defend") {
                        Color::srgb(0.1, 0.1, 0.5)
                    } else {
                        Color::srgb(0.3, 0.3, 0.3)
                    };

                    cards_container.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(220.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: color.into(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        },
                        CardChoiceButton { card_index: index },
                    )).with_children(|c| {
                        let text_style = TextStyle { font: Handle::default(), font_size: 18.0, color: Color::WHITE };
                        c.spawn(TextBundle::from_section(format!("Cost: {}", card.cost), text_style.clone()));
                        c.spawn(TextBundle::from_section(card.name.clone(), text_style.clone()));
                        if card.damage > 0 { c.spawn(TextBundle::from_section(format!("Dmg: {}", card.damage), text_style.clone())); }
                        if card.block > 0 { c.spawn(TextBundle::from_section(format!("Blk: {}", card.block), text_style.clone())); }
                    });
                }
            }
        });

        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(40.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: Color::srgb(0.4, 0.2, 0.2).into(),
                ..default()
            },
            SkipCardButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Skip", TextStyle {
                font: Handle::default(),
                font_size: 25.0,
                color: Color::WHITE,
            }));
        });
    });
}

pub fn reward_select_card_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut reward_store: ResMut<RewardStore>,
    mut deck: ResMut<Deck>,
    card_btn_query: Query<(&Interaction, &CardChoiceButton), (Changed<Interaction>, With<CardChoiceButton>)>,
    skip_btn_query: Query<&Interaction, (Changed<Interaction>, With<SkipCardButton>)>,
) {
    for (interaction, button) in &card_btn_query {
        if *interaction == Interaction::Pressed {
            if let Some(choices) = &reward_store.card_choices {
                if let Some(card) = choices.get(button.card_index) {
                    deck.cards.push(card.clone());
                    println!("Added {} to deck.", card.name);
                }
            }
            reward_store.card_choices = None; // Consumed
            next_game_state.set(GameState::Victory); // Return to reward screen
        }
    }

    for interaction in &skip_btn_query {
        if *interaction == Interaction::Pressed {
            reward_store.card_choices = None; // Consumed (Skipped)
            next_game_state.set(GameState::Victory); // Return to reward screen
        }
    }
}

fn generate_random_card() -> Card {
    let mut rng = thread_rng();
    let r = rng.gen_range(0..6);
    match r {
        0 => Card { name: "Iron Wave".to_string(), damage: 5, block: 5, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false },
        1 => Card { name: "Deadly Poison".to_string(), damage: 0, block: 0, cost: 1, apply_poison: 5, apply_weak: 0, upgraded: false },
        2 => Card { name: "Heavy Blade".to_string(), damage: 14, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false },
        3 => Card { name: "Shrug It Off".to_string(), damage: 0, block: 8, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false },
        4 => Card { name: "Clothesline".to_string(), damage: 12, block: 0, cost: 2, apply_poison: 0, apply_weak: 2, upgraded: false },
        _ => Card { name: "Strike".to_string(), damage: 6, block: 0, cost: 1, apply_poison: 0, apply_weak: 0, upgraded: false },
    }
}

pub fn setup_game_over_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
            z_index: ZIndex::Global(200),
            ..default()
        },
        GameOverUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("GAME OVER", TextStyle {
            font: Handle::default(),
            font_size: 60.0,
            color: Color::srgb(0.8, 0.1, 0.1),
        }));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
            RestartButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Restart Game", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

pub fn game_over_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
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

pub fn reset_turn_state(mut next_turn_state: ResMut<NextState<TurnState>>) {
    next_turn_state.set(TurnState::Setup);
}

pub fn update_status_visuals_system(
    mut commands: Commands,
    player_query: Query<&StatusStore, (With<Player>, Changed<StatusStore>)>,
    enemy_query: Query<&StatusStore, (With<Enemy>, Changed<StatusStore>)>,
    player_status_ui: Query<Entity, With<PlayerStatusText>>,
    enemy_status_ui: Query<Entity, With<EnemyStatusText>>,
    player_status_all: Query<&StatusStore, With<Player>>,
    enemy_status_all: Query<&StatusStore, With<Enemy>>,
    player_ui_added: Query<Entity, Added<PlayerStatusText>>,
    enemy_ui_added: Query<Entity, Added<EnemyStatusText>>,
) {
    let spawn_badges = |parent: &mut ChildBuilder, status: &StatusStore| {
        if status.poison > 0 {
            parent.spawn((
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
                Tooltip { text: "Poison: Takes damage at start of turn.".to_string() },
            )).with_children(|p| {
                p.spawn(TextBundle::from_section(format!("Psn {}", status.poison), TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    font: Handle::default(),
                }));
            });
        }
        if status.weak > 0 {
            parent.spawn((
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
                Tooltip { text: "Weak: Deal 25% less damage.".to_string() },
            )).with_children(|p| {
                p.spawn(TextBundle::from_section(format!("Wk {}", status.weak), TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    font: Handle::default(),
                }));
            });
        }
    };

    if !player_query.is_empty() || !player_ui_added.is_empty() {
        if let (Ok(status), Ok(ui_entity)) = (player_status_all.get_single(), player_status_ui.get_single()) {
            commands.entity(ui_entity).despawn_descendants();
            commands.entity(ui_entity).with_children(|parent| spawn_badges(parent, status));
        }
    }

    if !enemy_query.is_empty() || !enemy_ui_added.is_empty() {
        if let (Ok(status), Ok(ui_entity)) = (enemy_status_all.get_single(), enemy_status_ui.get_single()) {
            commands.entity(ui_entity).despawn_descendants();
            commands.entity(ui_entity).with_children(|parent| spawn_badges(parent, status));
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
                for e in tooltip_ui_query.iter() { commands.entity(e).despawn_recursive(); }
                commands.spawn((
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
                )).with_children(|p| {
                    p.spawn(TextBundle::from_section(&tooltip.text, TextStyle {
                        font_size: 18.0,
                        color: Color::WHITE,
                        font: Handle::default(),
                    }));
                });
            }
            Interaction::None => {
                for e in tooltip_ui_query.iter() { commands.entity(e).despawn_recursive(); }
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
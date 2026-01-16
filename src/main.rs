use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

// --- Components ---

#[derive(Component)]
struct Player;

#[derive(Debug, Clone, Copy)]
enum EnemyKind {
    Goblin,
    Orc,
}

#[derive(Component)]
struct Enemy {
    kind: EnemyKind,
}

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Energy {
    current: i32,
    max: i32,
}

#[derive(Component, Default)]
struct Block {
    value: i32,
}

#[derive(Component, Default)]
struct StatusStore {
    poison: i32,
    weak: i32,
}

#[derive(Component)]
struct Gold {
    amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relic {
    Vajra,
    BurningBlood,
}

#[derive(Component, Default)]
struct RelicStore {
    relics: Vec<Relic>,
}

#[derive(Component, Debug, Clone)]
struct Card {
    name: String,
    damage: i32,
    block: i32,
    cost: i32,
    apply_poison: i32,
    apply_weak: i32,
    upgraded: bool,
}

#[derive(Component)]
struct BaseColor(Color);

#[derive(Component)]
struct BattleEntity;

#[derive(Component)]
struct ShopUI;

#[derive(Component)]
struct RestUI;

#[derive(Component)]
struct VictoryUI;

#[derive(Component)]
struct MapUI;

#[derive(Component)]
struct PlayerHealthText;

#[derive(Component)]
struct EnemyHealthText;

#[derive(Component)]
struct PlayerGoldText;

#[derive(Component)]
struct PlayerEnergyText;

#[derive(Component)]
struct PlayerBlockText;

#[derive(Component)]
struct EnemyBlockText;

#[derive(Component)]
struct PlayerStatusText;

#[derive(Component)]
struct EnemyStatusText;

#[derive(Component)]
struct PlayerRelicText;

#[derive(Component)]
struct HandContainer;

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct EnemyIntentText;

#[derive(Component)]
struct EnterBattleButton;

#[derive(Component)]
struct VisitShopButton;

#[derive(Component)]
struct VisitRestButton;

#[derive(Component)]
struct LeaveShopButton;

#[derive(Component)]
struct BuyCardButton {
    card: Card,
    cost: i32,
}

#[derive(Component)]
struct BuyRelicButton {
    relic: Relic,
    cost: i32,
}

#[derive(Component)]
struct HealButton;

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct LeaveRestButton;


// --- Resources ---

#[derive(Resource, Default)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Resource, Default)]
struct DiscardPile {
    cards: Vec<Card>,
}

// State Management
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Battle,
    Victory,
    Shop,
    Rest,
    Map,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum TurnState {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
}

// --- Systems ---

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(mut commands: Commands) {
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

fn setup_battle(mut commands: Commands, mut next_turn_state: ResMut<NextState<TurnState>>) {
    println!("Setting up battle...");

    // Determine Enemy Type
    let mut rng = thread_rng();
    let enemy_kind = if rng.gen_bool(0.5) { EnemyKind::Goblin } else { EnemyKind::Orc };
    let (hp, name, color) = match enemy_kind {
        EnemyKind::Goblin => (20, "Goblin", Color::srgb(0.2, 0.8, 0.2)),
        EnemyKind::Orc => (40, "Orc", Color::srgb(0.8, 0.2, 0.2)),
    };

    // Spawn Enemy with Visuals
    commands.spawn((
        Enemy { kind: enemy_kind },
        Health { current: hp, max: hp },
        Block { value: 0 },
        StatusStore::default(),
        BattleEntity,
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

    // Spawn Health UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        BattleEntity,
    )).with_children(|parent| {
        // Player Health
        parent.spawn((
            TextBundle::from_section("Player: 50/50", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }),
            PlayerHealthText,
        ));
        // Player Gold
        parent.spawn((
            TextBundle::from_section("Gold: 0", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::srgb(1.0, 0.84, 0.0),
            }),
            PlayerGoldText,
        ));
        // Player Energy
        parent.spawn((
            TextBundle::from_section("Energy: 3/3", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::srgb(0.2, 0.8, 1.0),
            }),
            PlayerEnergyText,
        ));
        // Player Block
        parent.spawn((
            TextBundle::from_section("Block: 0", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::srgb(0.5, 0.5, 1.0),
            }),
            PlayerBlockText,
        ));
        // Player Status
        parent.spawn((
            TextBundle::from_section("Status: ", TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            }),
            PlayerStatusText,
        ));
        // Enemy Health
        parent.spawn((
            TextBundle::from_section(format!("Enemy: {}/{}", hp, hp), TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::srgb(0.9, 0.3, 0.3),
            }),
            EnemyHealthText,
        ));
        // Enemy Block
        parent.spawn((
            TextBundle::from_section("Block: 0", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::srgb(0.5, 0.5, 1.0),
            }),
            EnemyBlockText,
        ));
        // Enemy Status
        parent.spawn((
            TextBundle::from_section("Status: ", TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            }),
            EnemyStatusText,
        ));
        // Player Relics
        parent.spawn((
            TextBundle::from_section("Relics: ", TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::srgb(1.0, 0.84, 0.0),
            }),
            PlayerRelicText,
        ));
    });

    next_turn_state.set(TurnState::PlayerTurn);
}

fn draw_cards_system(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    mut discard: ResMut<DiscardPile>,
    hand_container_query: Query<Entity, With<HandContainer>>,
    mut player_query: Query<(&mut Energy, &mut Block, &mut Health, &mut StatusStore), With<Player>>,
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
        }
        if health.current <= 0 {
            println!("Player died to poison!");
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
}

fn discard_hand_system(
    mut commands: Commands,
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
}

fn end_turn_button_system(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_turn_state.set(TurnState::EnemyTurn);
        }
    }
}

// A system to play a card when clicked
fn play_card_system(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut discard_pile: ResMut<DiscardPile>,
    // Query for cards that have been clicked
    card_query: Query<(Entity, &Card, &Interaction), Changed<Interaction>>,
    // Query for the Enemy to take damage
    mut enemy_query: Query<(&mut Health, &mut Block, &mut StatusStore), (With<Enemy>, Without<Player>)>,
    // Query for Player Energy
    mut player_query: Query<(&mut Energy, &StatusStore, &RelicStore, &mut Health), With<Player>>,
    // Query for Player Block
    mut player_block_query: Query<&mut Block, (With<Player>, Without<Enemy>)>,
) {
    for (card_entity, card_data, interaction) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            // Check Energy
            let (mut energy, player_status, player_relics, mut player_health) = if let Ok(e) = player_query.get_single_mut() { e } else { continue };
            if energy.current < card_data.cost {
                println!("Not enough energy!");
                continue;
            }
            energy.current -= card_data.cost;

            // 1. Apply Block to Player
            if card_data.block > 0 {
                if let Ok(mut block) = player_block_query.get_single_mut() {
                    block.value += card_data.block;
                    println!("Player gains {} block", card_data.block);
                }
            }

            println!("Player plays: {}", card_data.name);

            // 2. Apply damage to Enemy
            for (mut enemy_health, mut enemy_block, mut enemy_status) in enemy_query.iter_mut() {
                // Apply Statuses
                if card_data.apply_poison > 0 { enemy_status.poison += card_data.apply_poison; }
                if card_data.apply_weak > 0 { enemy_status.weak += card_data.apply_weak; }

                // Calculate Damage (Player Weak check)
                let mut damage = card_data.damage;
                // Relic: Vajra (+1 Strength)
                if player_relics.relics.contains(&Relic::Vajra) && damage > 0 {
                    damage += 1;
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
                    // Relic: Burning Blood (Heal on victory)
                    if player_relics.relics.contains(&Relic::BurningBlood) {
                        player_health.current = (player_health.current + 6).min(player_health.max);
                        println!("Burning Blood heals 6 HP!");
                    }
                    next_game_state.set(GameState::Victory);
                    return;
                }
            }

            // 3. Add to Discard Pile
            discard_pile.cards.push(card_data.clone());

            // 3. Discard/Destroy the card entity after use
            commands.entity(card_entity).despawn_recursive();

            // 4. End Turn (Switch state)
            // We no longer end turn automatically on play
            break;
        }
    }
}

fn card_hover_system(
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

fn enemy_turn_system(
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Health, &mut Block, &RelicStore), With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Block, &mut Health, &mut StatusStore), Without<Player>>,
    mut intent_text_query: Query<&mut Text, With<EnemyIntentText>>,
) {
    let (enemy, mut enemy_block, mut enemy_health, mut enemy_status) = enemy_query.single_mut();
    
    // Poison Logic (Start of Turn)
    if enemy_status.poison > 0 {
        enemy_health.current -= enemy_status.poison;
        enemy_status.poison -= 1;
    }
    if enemy_health.current <= 0 {
        // Relic: Burning Blood (Heal on victory via poison)
        if let Ok((mut p_health, _, p_relics)) = player_query.get_single_mut() {
            if p_relics.relics.contains(&Relic::BurningBlood) {
                p_health.current = (p_health.current + 6).min(p_health.max);
            }
        }
        next_game_state.set(GameState::Victory);
        return;
    }
    
    // Reset Enemy Block at start of their turn
    enemy_block.value = 0;

    let (damage, action_desc) = match enemy.kind {
        EnemyKind::Goblin => (5, "Stabs"),
        EnemyKind::Orc => (12, "Smashes"),
    };

    // Calculate Damage (Enemy Weak check)
    let mut final_damage = damage;
    if enemy_status.weak > 0 {
        final_damage = (final_damage as f32 * 0.75) as i32;
        enemy_status.weak -= 1;
    }

    println!("Enemy {}!", action_desc);
    
    // Update Intent Text
    for mut text in &mut intent_text_query {
        text.sections[0].value = format!("{}! ({} dmg)", action_desc, final_damage);
    }

    for (mut player_health, mut player_block, _) in player_query.iter_mut() {
        let block_damage = std::cmp::min(final_damage, player_block.value);
        player_block.value -= block_damage;
        player_health.current -= final_damage - block_damage;
        println!("Player Health: {}/{}", player_health.current, player_health.max);
    }

    // Pass turn back to player
    next_turn_state.set(TurnState::PlayerTurn);
    println!("Player's turn.");
}

fn update_health_ui(
    player_health_query: Query<&Health, (With<Player>, Changed<Health>)>,
    enemy_health_query: Query<&Health, (With<Enemy>, Changed<Health>)>,
    player_block_query: Query<&Block, (With<Player>, Changed<Block>)>,
    enemy_block_query: Query<&Block, (With<Enemy>, Changed<Block>)>,
    player_gold_query: Query<&Gold, (With<Player>, Changed<Gold>)>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<EnemyHealthText>>,
        Query<&mut Text, With<PlayerBlockText>>,
        Query<&mut Text, With<EnemyBlockText>>,
        Query<&mut Text, With<PlayerGoldText>>,
    )>,
) {
    if let Ok(health) = player_health_query.get_single() {
        for mut text in text_queries.p0().iter_mut() {
            text.sections[0].value = format!("Player: {}/{}", health.current, health.max);
        }
    }
    if let Ok(health) = enemy_health_query.get_single() {
        for mut text in text_queries.p1().iter_mut() {
            text.sections[0].value = format!("Enemy: {}/{}", health.current, health.max);
        }
    }
    if let Ok(block) = player_block_query.get_single() {
        for mut text in text_queries.p2().iter_mut() {
            text.sections[0].value = format!("Block: {}", block.value);
        }
    }
    if let Ok(block) = enemy_block_query.get_single() {
        for mut text in text_queries.p3().iter_mut() {
            text.sections[0].value = format!("Block: {}", block.value);
        }
    }
    if let Ok(gold) = player_gold_query.get_single() {
        for mut text in text_queries.p4().iter_mut() {
            text.sections[0].value = format!("Gold: {}", gold.amount);
        }
    }
}

fn update_energy_ui(
    player_energy_query: Query<&Energy, (With<Player>, Changed<Energy>)>,
    mut energy_text_query: Query<&mut Text, With<PlayerEnergyText>>,
) {
    if let Ok(energy) = player_energy_query.get_single() {
        for mut text in &mut energy_text_query {
            text.sections[0].value = format!("Energy: {}/{}", energy.current, energy.max);
        }
    }
}

fn update_status_ui(
    player_status_query: Query<&StatusStore, (With<Player>, Changed<StatusStore>)>,
    enemy_status_query: Query<&StatusStore, (With<Enemy>, Changed<StatusStore>)>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<PlayerStatusText>>,
        Query<&mut Text, With<EnemyStatusText>>,
    )>,
) {
    if let Ok(status) = player_status_query.get_single() {
        for mut text in text_queries.p0().iter_mut() {
            let mut s = String::from("Status: ");
            if status.poison > 0 { s.push_str(&format!("Psn({}) ", status.poison)); }
            if status.weak > 0 { s.push_str(&format!("Wk({}) ", status.weak)); }
            text.sections[0].value = s;
        }
    }
    if let Ok(status) = enemy_status_query.get_single() {
        for mut text in text_queries.p1().iter_mut() {
            let mut s = String::from("Status: ");
            if status.poison > 0 { s.push_str(&format!("Psn({}) ", status.poison)); }
            if status.weak > 0 { s.push_str(&format!("Wk({}) ", status.weak)); }
            text.sections[0].value = s;
        }
    }
}

fn update_relic_ui(
    player_relic_query: Query<&RelicStore, (With<Player>, Changed<RelicStore>)>,
    mut text_query: Query<&mut Text, With<PlayerRelicText>>,
) {
    if let Ok(relics) = player_relic_query.get_single() {
        for mut text in &mut text_query {
            let mut s = String::from("Relics: ");
            for relic in &relics.relics {
                match relic {
                    Relic::Vajra => s.push_str("Vajra (+1 Dmg) "),
                    Relic::BurningBlood => s.push_str("Burning Blood (Heal 6) "),
                }
            }
            text.sections[0].value = s;
        }
    }
}

fn setup_victory_screen(mut commands: Commands) {
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
        VictoryUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Enemy Defeated!", TextStyle {
            font: Handle::default(),
            font_size: 50.0,
            color: Color::srgb(1.0, 0.84, 0.0),
        }));
        
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Next Stage", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.5, 0.1, 0.5).into(),
                ..default()
            },
            VisitRestButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Visit Rest Site", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

fn setup_shop_screen(mut commands: Commands, player_query: Query<&Gold, With<Player>>) {
    let gold = player_query.single().amount;

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
            background_color: Color::srgb(0.1, 0.1, 0.2).into(),
            ..default()
        },
        ShopUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(format!("Shop - Gold: {}", gold), TextStyle {
            font: Handle::default(),
            font_size: 40.0,
            color: Color::srgb(1.0, 0.84, 0.0),
        }));

        // Shop Items Container
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }).with_children(|items| {
            // Sell a Card
            let card = Card { name: "Heavy Blade".to_string(), damage: 15, block: 0, cost: 2, apply_poison: 0, apply_weak: 0, upgraded: false };
            let cost = 50;
            items.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(160.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.3, 0.4).into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                BuyCardButton { card: card.clone(), cost },
            )).with_children(|b| {
                b.spawn(TextBundle::from_section(format!("{}\n{}g", card.name, cost), TextStyle {
                    font: Handle::default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }));
            });

            // Sell a Relic
            let relic = Relic::Vajra;
            let cost = 75;
            items.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(160.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.4, 0.2, 0.2).into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                BuyRelicButton { relic, cost },
            )).with_children(|b| {
                b.spawn(TextBundle::from_section(format!("Vajra\n{}g", cost), TextStyle {
                    font: Handle::default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }));
            });
        });

        // Leave Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.5, 0.1, 0.1).into(),
                ..default()
            },
            LeaveShopButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Leave Shop", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

fn shop_interaction_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Gold, &mut RelicStore), With<Player>>,
    mut deck: ResMut<Deck>,
    mut card_interaction: Query<(Entity, &Interaction, &BuyCardButton), (Changed<Interaction>, With<BuyCardButton>)>,
    mut relic_interaction: Query<(Entity, &Interaction, &BuyRelicButton), (Changed<Interaction>, With<BuyRelicButton>)>,
) {
    let (mut gold, mut relics) = player_query.single_mut();

    for (entity, interaction, button) in &mut card_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            gold.amount -= button.cost;
            deck.cards.push(button.card.clone());
            println!("Bought {}", button.card.name);
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, interaction, button) in &mut relic_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            gold.amount -= button.cost;
            relics.relics.push(button.relic);
            println!("Bought Relic");
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn setup_rest_screen(mut commands: Commands, player_query: Query<&Health, With<Player>>) {
    let health = player_query.single();
    
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
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
        RestUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Rest Site", TextStyle {
            font: Handle::default(),
            font_size: 50.0,
            color: Color::WHITE,
        }));

        // Heal Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            HealButton,
        )).with_children(|p| {
            let heal_amount = (health.max as f32 * 0.3) as i32;
            p.spawn(TextBundle::from_section(format!("Heal ({} HP)", heal_amount), TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });

        // Upgrade Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.6, 0.2, 0.2).into(),
                ..default()
            },
            UpgradeButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Upgrade Random Card", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
        
        // Leave Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    margin: UiRect::top(Val::Px(40.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
            LeaveRestButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Leave", TextStyle {
                font: Handle::default(),
                font_size: 25.0,
                color: Color::WHITE,
            }));
        });
    });
}

fn rest_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<&mut Health, With<Player>>,
    mut deck: ResMut<Deck>,
    heal_query: Query<&Interaction, (Changed<Interaction>, With<HealButton>)>,
    upgrade_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
    leave_query: Query<&Interaction, (Changed<Interaction>, With<LeaveRestButton>)>,
) {
    for interaction in &heal_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut health) = player_query.get_single_mut() {
                let heal_amount = (health.max as f32 * 0.3) as i32;
                health.current = (health.current + heal_amount).min(health.max);
                println!("Healed for {}", heal_amount);
                next_game_state.set(GameState::Map);
            }
        }
    }

    for interaction in &upgrade_query {
        if *interaction == Interaction::Pressed {
            // Upgrade random card
            let mut rng = thread_rng();
            let upgradable_indices: Vec<usize> = deck.cards.iter().enumerate()
                .filter(|(_, c)| !c.upgraded)
                .map(|(i, _)| i)
                .collect();
            
            if let Some(&index) = upgradable_indices.choose(&mut rng) {
                let card = &mut deck.cards[index];
                card.upgraded = true;
                card.name.push_str("+");
                if card.damage > 0 { card.damage += 3; }
                if card.block > 0 { card.block += 3; }
                println!("Upgraded {}!", card.name);
                next_game_state.set(GameState::Map);
            } else {
                println!("No cards to upgrade!");
                next_game_state.set(GameState::Map);
            }
        }
    }

    for interaction in &leave_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}

fn setup_map_screen(mut commands: Commands) {
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
            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
            ..default()
        },
        MapUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Map Screen", TextStyle {
            font: Handle::default(),
            font_size: 50.0,
            color: Color::WHITE,
        }));
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.5, 0.1).into(),
                ..default()
            },
            EnterBattleButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Enter Next Battle", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.5).into(),
                ..default()
            },
            VisitShopButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Visit Shop", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

fn map_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    battle_query: Query<&Interaction, (Changed<Interaction>, With<EnterBattleButton>)>,
    shop_query: Query<&Interaction, (Changed<Interaction>, With<VisitShopButton>)>,
    rest_query: Query<&Interaction, (Changed<Interaction>, With<VisitRestButton>)>,
) {
    for interaction in &battle_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Battle);
        }
    }
    for interaction in &shop_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Shop);
        }
    }
    for interaction in &rest_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Rest);
        }
    }
    for interaction in &rest_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Rest);
        }
    }
}

fn victory_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}

fn shop_nav_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LeaveShopButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_turn_state(mut next_turn_state: ResMut<NextState<TurnState>>) {
    next_turn_state.set(TurnState::Setup);
}

// --- App Entry Point ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds windowing, input, etc.
        .init_state::<GameState>()
        .init_state::<TurnState>()
        .add_systems(Startup, (setup_camera, setup_game))
        .add_systems(OnEnter(GameState::Battle), setup_battle)
        .add_systems(
            Update,
            (
                play_card_system.run_if(in_state(TurnState::PlayerTurn)),
                enemy_turn_system.run_if(in_state(TurnState::EnemyTurn)),
                card_hover_system,
                update_health_ui,
                update_energy_ui,
                update_status_ui,
                update_relic_ui,
                end_turn_button_system.run_if(in_state(TurnState::PlayerTurn)),
            ).run_if(in_state(GameState::Battle))
        )
        .add_systems(OnEnter(TurnState::PlayerTurn), draw_cards_system)
        .add_systems(OnExit(TurnState::PlayerTurn), discard_hand_system)
        .add_systems(OnExit(GameState::Battle), (despawn_screen::<BattleEntity>, reset_turn_state))
        .add_systems(OnEnter(GameState::Victory), setup_victory_screen)
        .add_systems(Update, victory_interaction_system.run_if(in_state(GameState::Victory)))
        .add_systems(OnExit(GameState::Victory), despawn_screen::<VictoryUI>)
        .add_systems(OnEnter(GameState::Map), setup_map_screen)
        .add_systems(Update, map_interaction_system.run_if(in_state(GameState::Map)))
        .add_systems(OnExit(GameState::Map), despawn_screen::<MapUI>)
        .add_systems(OnEnter(GameState::Shop), setup_shop_screen)
        .add_systems(Update, (shop_interaction_system, shop_nav_system).run_if(in_state(GameState::Shop)))
        .add_systems(OnExit(GameState::Shop), despawn_screen::<ShopUI>)
        .add_systems(OnEnter(GameState::Rest), setup_rest_screen)
        .add_systems(Update, rest_interaction_system.run_if(in_state(GameState::Rest)))
        .add_systems(OnExit(GameState::Rest), despawn_screen::<RestUI>)
        .run();
}

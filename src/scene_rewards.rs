use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::item_cards::{get_card_visuals, generate_random_card};

pub fn setup_victory_screen(mut commands: Commands, mut reward_store: ResMut<RewardStore>, game_map: Res<GameMap>) {
    // Generate rewards if not already generated
    if !reward_store.generated {
        let mut rng = thread_rng();
        
        let node_type = if let Some((l, i)) = game_map.current_node {
            game_map.levels[l][i].node_type
        } else {
            NodeType::Battle
        };

        // Gold Reward
        let (min, max) = if node_type == NodeType::Elite { (50, 100) } else { (20, 50) };
        reward_store.gold_reward = Some(rng.gen_range(min..=max));

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
                    let (bg_color, border_color) = get_card_visuals(card);

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
                            background_color: bg_color.into(),
                            border_color: border_color.into(),
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
use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;

pub fn setup_shop_screen(mut commands: Commands, player_query: Query<&Gold, With<Player>>) {
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
        parent.spawn((
            TextBundle::from_section(format!("Shop - Gold: {}", gold), TextStyle {
                font: Handle::default(),
                font_size: 40.0,
                color: Color::srgb(1.0, 0.84, 0.0),
            }),
            ShopGoldText,
        ));

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

            // Sell a Potion
            let potion = PotionType::Health;
            let cost = 25;
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
                    background_color: Color::srgb(0.2, 0.5, 0.2).into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                BuyPotionButton { potion, cost },
            )).with_children(|b| {
                b.spawn(TextBundle::from_section(format!("Health Potion\n{}g", cost), TextStyle {
                    font: Handle::default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }));
            });

            // Remove Card Service
            let remove_cost = 75;
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
                    background_color: Color::srgb(0.4, 0.2, 0.4).into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                RemoveCardServiceButton { cost: remove_cost },
            )).with_children(|b| {
                b.spawn(TextBundle::from_section(format!("Remove Card\n{}g", remove_cost), TextStyle {
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

pub fn shop_interaction_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Gold, &mut RelicStore, &mut PotionStore), With<Player>>,
    mut deck: ResMut<Deck>,
    mut card_interaction: Query<(Entity, &Interaction, &BuyCardButton), (Changed<Interaction>, With<BuyCardButton>)>,
    mut relic_interaction: Query<(Entity, &Interaction, &BuyRelicButton), (Changed<Interaction>, With<BuyRelicButton>)>,
    mut potion_interaction: Query<(Entity, &Interaction, &BuyPotionButton), (Changed<Interaction>, With<BuyPotionButton>)>,
    mut remove_service_interaction: Query<(&Interaction, &RemoveCardServiceButton), (Changed<Interaction>, With<RemoveCardServiceButton>)>,
) {
    let (mut gold, mut relics, mut potions) = player_query.single_mut();

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

    for (entity, interaction, button) in &mut potion_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            gold.amount -= button.cost;
            potions.potions.push(button.potion);
            println!("Bought Potion");
            commands.entity(entity).despawn_recursive();
        }
    }

    for (interaction, button) in &mut remove_service_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            next_state.set(GameState::ShopRemoveCard);
        }
    }
}

pub fn update_shop_gold_ui(
    player_gold_query: Query<&Gold, (With<Player>, Changed<Gold>)>,
    mut shop_gold_text_query: Query<&mut Text, With<ShopGoldText>>,
) {
    if let Ok(gold) = player_gold_query.get_single() {
        for mut text in &mut shop_gold_text_query {
            text.sections[0].value = format!("Shop - Gold: {}", gold.amount);
        }
    }
}

pub fn setup_shop_remove_screen(mut commands: Commands, deck: Res<Deck>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.05, 0.98).into(),
            z_index: ZIndex::Global(100),
            ..default()
        },
        ShopRemoveUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Select Card to Remove (Cost: 75g)", TextStyle {
            font: Handle::default(),
            font_size: 40.0,
            color: Color::WHITE,
        }));

        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        }).with_children(|grid| {
            for (index, card) in deck.cards.iter().enumerate() {
                let color = if card.name.contains("Strike") {
                    Color::srgb(0.5, 0.1, 0.1)
                } else if card.name.contains("Defend") {
                    Color::srgb(0.1, 0.1, 0.5)
                } else {
                    Color::srgb(0.3, 0.3, 0.3)
                };
                
                grid.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(150.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: color.into(),
                        border_color: Color::WHITE.into(),
                        ..default()
                    },
                    CardToRemoveButton { index },
                )).with_children(|c| {
                    let text_style = TextStyle {
                        font: Handle::default(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    };
                    c.spawn(TextBundle::from_section(format!("Cost: {}", card.cost), text_style.clone()));
                    c.spawn(TextBundle::from_section(card.name.clone(), text_style.clone()));
                });
            }
        });

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
            CancelRemoveButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Cancel", TextStyle {
                font: Handle::default(),
                font_size: 25.0,
                color: Color::WHITE,
            }));
        });
    });
}

pub fn shop_remove_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut deck: ResMut<Deck>,
    mut player_query: Query<&mut Gold, With<Player>>,
    card_query: Query<(&Interaction, &CardToRemoveButton), (Changed<Interaction>, With<CardToRemoveButton>)>,
    cancel_query: Query<&Interaction, (Changed<Interaction>, With<CancelRemoveButton>)>,
) {
    for (interaction, button) in &card_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut gold) = player_query.get_single_mut() {
                if gold.amount >= 75 {
                    gold.amount -= 75;
                    if button.index < deck.cards.len() {
                        let removed = deck.cards.remove(button.index);
                        println!("Removed {} from deck.", removed.name);
                    }
                    next_state.set(GameState::Shop);
                }
            }
        }
    }

    for interaction in &cancel_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Shop);
        }
    }
}

pub fn shop_nav_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LeaveShopButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}
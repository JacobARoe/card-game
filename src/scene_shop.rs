use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::item_cards::generate_random_card;
use crate::item_relics::{Relic, get_relic_name};
use crate::item_potions::{Potion, get_potion_name};
use rand::{thread_rng, Rng};
use crate::common::spawn_card_visual;

pub fn setup_shop_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Gold, With<Player>>,
    mut shop_store: ResMut<ShopStore>
) {
    let gold = player_query.single().amount;

    if !shop_store.generated {
        let mut rng = thread_rng();
        
        // Generate Cards
        shop_store.cards.clear();
        for _ in 0..3 {
            let card = generate_random_card();
            let cost = rng.gen_range(40..80);
            shop_store.cards.push(Some((card, cost)));
        }

        // Generate Relic
        shop_store.relics.clear();
        let relic = if rng.gen_bool(0.5) { Relic::Vajra } else { Relic::BurningBlood };
        let cost = rng.gen_range(100..150);
        shop_store.relics.push(Some((relic, cost)));

        // Generate Potion
        shop_store.potions.clear();
        let potion_type = match rng.gen_range(0..3) {
            0 => Potion::Health,
            1 => Potion::Strength,
            _ => Potion::Energy,
        };
        let cost = rng.gen_range(20..40);
        shop_store.potions.push(Some((potion_type, cost)));

        shop_store.generated = true;
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
            for (index, item) in shop_store.cards.iter().enumerate() {
                if let Some((card, cost)) = item {
                    spawn_card_visual(
                        items,
                        &asset_server,
                        card,
                        (
                            Button,
                            Interaction::default(),
                            BuyCardButton { card: card.clone(), cost: *cost, index },
                        ),
                        |card_ui| {
                            card_ui.spawn(TextBundle::from_section(format!("{}g", cost), TextStyle {
                                font: Handle::default(),
                                font_size: 20.0,
                                color: Color::srgb(1.0, 0.84, 0.0),
                            }));
                        }
                    );
                }
            }

            // Sell a Relic
            for (index, item) in shop_store.relics.iter().enumerate() {
                if let Some((relic, cost)) = item {
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
                        BuyRelicButton { relic: *relic, cost: *cost, index },
                    )).with_children(|b| {
                        b.spawn(TextBundle::from_section(format!("{}\n{}g", get_relic_name(relic), cost), TextStyle {
                            font: Handle::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }));
                    });
                }
            }

            // Sell a Potion
            for (index, item) in shop_store.potions.iter().enumerate() {
                if let Some((potion, cost)) = item {
                    let name = get_potion_name(potion);
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
                        BuyPotionButton { potion: *potion, cost: *cost, index },
                    )).with_children(|b| {
                        b.spawn(TextBundle::from_section(format!("{}\n{}g", name, cost), TextStyle {
                            font: Handle::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }));
                    });
                }
            }

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
    mut shop_store: ResMut<ShopStore>,
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
            shop_store.cards[button.index] = None;
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, interaction, button) in &mut relic_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            gold.amount -= button.cost;
            relics.relics.push(button.relic);
            println!("Bought Relic");
            shop_store.relics[button.index] = None;
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, interaction, button) in &mut potion_interaction {
        if *interaction == Interaction::Pressed && gold.amount >= button.cost {
            gold.amount -= button.cost;
            potions.potions.push(button.potion);
            println!("Bought Potion");
            shop_store.potions[button.index] = None;
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

pub fn setup_shop_remove_screen(mut commands: Commands, asset_server: Res<AssetServer>, deck: Res<Deck>) {
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
                spawn_card_visual(
                    grid,
                    &asset_server,
                    card,
                    (
                        Button,
                        Interaction::default(),
                        CardToRemoveButton { index },
                    ),
                    |_| {}
                );
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
    mut shop_store: ResMut<ShopStore>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LeaveShopButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            shop_store.generated = false; // Reset shop for next visit
            next_game_state.set(GameState::Map);
        }
    }
}
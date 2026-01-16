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

pub fn shop_interaction_system(
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
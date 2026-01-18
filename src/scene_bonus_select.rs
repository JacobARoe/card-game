use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::resources::*;
use crate::item_relics::Relic;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn setup_bonus_select_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.05, 0.98).into(),
            z_index: ZIndex::Global(100),
            ..default()
        },
        BonusSelectUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Choose a Starting Bonus", TextStyle {
            font: Handle::default(),
            font_size: 40.0,
            color: Color::WHITE,
        }));

        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Px(40.0)),
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        }).with_children(|container| {
            // Gold Option
            spawn_bonus_button(container, "Gain 100 Gold", BonusType::Gold, Color::srgb(0.8, 0.6, 0.0));
            
            // Relic Option
            spawn_bonus_button(container, "Obtain Random Relic", BonusType::Relic, Color::srgb(0.2, 0.2, 0.6));
            
            // Upgrade Option
            spawn_bonus_button(container, "Upgrade Random Card", BonusType::Upgrade, Color::srgb(0.6, 0.2, 0.2));
        });
    });
}

fn spawn_bonus_button(parent: &mut ChildBuilder, text: &str, bonus_type: BonusType, color: Color) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(300.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: color.into(),
            border_color: Color::WHITE.into(),
            ..default()
        },
        BonusOptionButton { bonus_type },
    )).with_children(|p| {
        p.spawn(TextBundle::from_section(text, TextStyle {
            font: Handle::default(),
            font_size: 24.0,
            color: Color::WHITE,
        }));
    });
}

pub fn bonus_select_interaction_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Gold, &mut RelicStore), With<Player>>,
    mut deck: ResMut<Deck>,
    interaction_query: Query<(&Interaction, &BonusOptionButton), (Changed<Interaction>, With<BonusOptionButton>)>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let (mut gold, mut relic_store) = player_query.single_mut();
            
            match button.bonus_type {
                BonusType::Gold => {
                    gold.amount += 100;
                    println!("Bonus: Gained 100 Gold");
                },
                BonusType::Relic => {
                    let all_relics = vec![Relic::Vajra, Relic::BurningBlood, Relic::Anchor, Relic::OddlySmoothStone, Relic::BagOfMarbles];
                    let available: Vec<Relic> = all_relics.into_iter()
                        .filter(|r| !relic_store.relics.contains(r))
                        .collect();
                    
                    if let Some(relic) = available.choose(&mut thread_rng()) {
                        relic_store.relics.push(*relic);
                        println!("Bonus: Gained {:?}", relic);
                    }
                },
                BonusType::Upgrade => {
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
                        println!("Bonus: Upgraded {}", card.name);
                    }
                }
            }
            
            next_state.set(GameState::Map);
        }
    }
}
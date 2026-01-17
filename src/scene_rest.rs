use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::components::*;
use crate::resources::*;
use crate::states::*;

pub fn setup_rest_screen(mut commands: Commands, player_query: Query<&Health, With<Player>>) {
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

pub fn rest_interaction_system(
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
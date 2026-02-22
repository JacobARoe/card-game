use crate::components::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use rand::{Rng, thread_rng};

pub fn setup_event_screen(mut commands: Commands) {
    let mut rng = thread_rng();
    let event_type = rng.gen_range(0..2); // 0: Shrine, 1: Beggar

    commands
        .spawn((
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
                background_color: Color::srgba(0.05, 0.05, 0.1, 0.98).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
            EventUI,
        ))
        .with_children(|parent| {
            let (title, desc, options) = match event_type {
                0 => (
                    "Mysterious Shrine",
                    "You stumble upon an ancient shrine glowing with faint light.",
                    vec![
                        ("Pray (Heal 15 HP)", 0),
                        ("Desecrate (Gain 50 Gold, Lose 5 HP)", 1),
                        ("Leave", 2),
                    ],
                ),
                _ => (
                    "Old Beggar",
                    "A beggar approaches you, asking for spare change.",
                    vec![
                        ("Give 10 Gold (Remove a random Strike)", 3),
                        ("Attack (Gain 10 Gold)", 4),
                        ("Ignore", 2),
                    ],
                ),
            };

            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: Handle::default(),
                    font_size: 40.0,
                    color: Color::srgb(0.8, 0.8, 1.0),
                },
            ));

            parent.spawn(TextBundle::from_section(
                desc,
                TextStyle {
                    font: Handle::default(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));

            for (text, id) in options {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(60.0),
                                margin: UiRect::top(Val::Px(20.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.2, 0.3).into(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        },
                        EventOptionButton { effect_id: id },
                    ))
                    .with_children(|b| {
                        b.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: Handle::default(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            }
        });
}

pub fn event_interaction_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Health, &mut Gold), With<Player>>,
    mut deck: ResMut<Deck>,
    interaction_query: Query<
        (&Interaction, &EventOptionButton),
        (Changed<Interaction>, With<EventOptionButton>),
    >,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((mut health, mut gold)) = player_query.get_single_mut() {
                match button.effect_id {
                    0 => {
                        // Pray
                        health.current = (health.current + 15).min(health.max);
                        println!("Healed 15 HP");
                    }
                    1 => {
                        // Desecrate
                        gold.amount += 50;
                        health.current -= 5;
                        println!("Gained 50 Gold, Lost 5 HP");
                    }
                    2 => {
                        // Leave/Ignore
                        println!("Left event");
                    }
                    3 => {
                        // Give Gold (Remove Card)
                        if gold.amount >= 10 {
                            gold.amount -= 10;
                            if let Some(idx) = deck.cards.iter().position(|c| c.name == "Strike") {
                                deck.cards.remove(idx);
                                println!("Removed a Strike");
                            } else if !deck.cards.is_empty() {
                                deck.cards.remove(0);
                                println!("Removed a card");
                            }
                        } else {
                            println!("Not enough gold!");
                            continue;
                        }
                    }
                    4 => {
                        // Attack
                        gold.amount += 10;
                        println!("Gained 10 Gold");
                    }
                    _ => {}
                }
            }
            next_state.set(GameState::Map);
        }
    }
}

use bevy::prelude::*;
use crate::components::*;
use crate::states::*;

pub fn setup_main_menu(mut commands: Commands) {
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
        MainMenuUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Card Game", TextStyle {
            font: Handle::default(),
            font_size: 80.0,
            color: Color::srgb(0.9, 0.9, 0.9),
        }));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    margin: UiRect::top(Val::Px(50.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            StartGameButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Start Game", TextStyle {
                font: Handle::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }));
        });
    });
}

pub fn menu_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}
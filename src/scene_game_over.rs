use crate::components::*;
use crate::states::*;
use bevy::prelude::*;

pub fn setup_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: Handle::default(),
                    font_size: 60.0,
                    color: Color::srgb(0.8, 0.1, 0.1),
                },
            ));

            parent
                .spawn((
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
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Restart Game",
                        TextStyle {
                            font: Handle::default(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
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

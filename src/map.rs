use bevy::prelude::*;
use crate::components::*;
use crate::states::*;

pub fn setup_map_screen(mut commands: Commands) {
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

pub fn map_interaction_system(
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
}
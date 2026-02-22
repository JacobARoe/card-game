use crate::components::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;

pub fn setup_character_select_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/backgrounds/Menu.jpg"),
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            ..default()
        },
        CharacterSelectUI,
        SceneBackground,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            CharacterSelectUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Select Your Character",
                TextStyle {
                    font: Handle::default(),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(40.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|container| {
                    // Duelist
                    container
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(220.0),
                                    height: Val::Px(320.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.1, 0.1).into(),
                                border_color: Color::WHITE.into(),
                                ..default()
                            },
                            SelectDuelistButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(TextBundle::from_section(
                                "The Duelist",
                                TextStyle {
                                    font: Handle::default(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                            btn.spawn(
                                TextBundle::from_section(
                                    "A balanced warrior.",
                                    TextStyle {
                                        font: Handle::default(),
                                        font_size: 16.0,
                                        color: Color::srgb(0.8, 0.8, 0.8),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::top(Val::Px(20.0)),
                                    ..default()
                                }),
                            );
                        });

                    // Spellweaver
                    container
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(220.0),
                                    height: Val::Px(320.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.1, 0.1, 0.4).into(),
                                border_color: Color::WHITE.into(),
                                ..default()
                            },
                            SelectSpellweaverButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(TextBundle::from_section(
                                "The Spellweaver",
                                TextStyle {
                                    font: Handle::default(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                            btn.spawn(
                                TextBundle::from_section(
                                    "Master of the arcane.",
                                    TextStyle {
                                        font: Handle::default(),
                                        font_size: 16.0,
                                        color: Color::srgb(0.8, 0.8, 0.8),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::top(Val::Px(20.0)),
                                    ..default()
                                }),
                            );
                        });
                });
        });
}

pub fn character_select_interaction_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut run_state: ResMut<RunState>,
    duelist_query: Query<&Interaction, (Changed<Interaction>, With<SelectDuelistButton>)>,
    spellweaver_query: Query<&Interaction, (Changed<Interaction>, With<SelectSpellweaverButton>)>,
) {
    for interaction in &duelist_query {
        if *interaction == Interaction::Pressed {
            run_state.character_class = CharacterClass::Duelist;
            next_state.set(GameState::BonusSelect);
        }
    }

    for interaction in &spellweaver_query {
        if *interaction == Interaction::Pressed {
            run_state.character_class = CharacterClass::Spellweaver;
            next_state.set(GameState::BonusSelect);
        }
    }
}

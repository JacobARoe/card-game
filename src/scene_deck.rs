use crate::common::spawn_card_visual;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;

pub fn setup_view_deck_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck: Res<Deck>,
) {
    commands
        .spawn((
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
            ViewDeckUI,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn(TextBundle::from_section(
                format!("Deck ({})", deck.cards.len()),
                TextStyle {
                    font: Handle::default(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            // Card Grid
            parent
                .spawn(NodeBundle {
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
                })
                .with_children(|grid| {
                    let mut sorted_cards = deck.cards.clone();
                    sorted_cards.sort_by(|a, b| a.name.cmp(&b.name));

                    for card in sorted_cards {
                        spawn_card_visual(grid, &asset_server, &card, (), |_| {});
                    }
                });

            // Return Button
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
                    ReturnFromDeckButton,
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font: Handle::default(),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn view_deck_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReturnFromDeckButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Map);
        }
    }
}

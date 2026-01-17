use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::item_cards::get_card_visuals;

pub fn setup_view_deck_screen(mut commands: Commands, deck: Res<Deck>) {
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
        ViewDeckUI,
    )).with_children(|parent| {
        // Header
        parent.spawn(TextBundle::from_section(format!("Deck ({})", deck.cards.len()), TextStyle {
            font: Handle::default(),
            font_size: 40.0,
            color: Color::WHITE,
        }));

        // Card Grid
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
            let mut sorted_cards = deck.cards.clone();
            sorted_cards.sort_by(|a, b| a.name.cmp(&b.name));

            for card in sorted_cards {
                let (bg_color, border_color) = get_card_visuals(&card);
                
                grid.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        height: Val::Px(150.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: bg_color.into(),
                    border_color: border_color.into(),
                    ..default()
                }).with_children(|c| {
                    let text_style = TextStyle {
                        font: Handle::default(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    };
                    c.spawn(TextBundle::from_section(card.name.clone(), TextStyle {
                        font: Handle::default(),
                        font_size: 22.0,
                        color: Color::WHITE,
                    }));
                    c.spawn(TextBundle::from_section(format!("Cost: {}", card.cost), text_style.clone()));
                    if card.damage > 0 {
                        c.spawn(TextBundle::from_section(format!("Dmg: {}", card.damage), text_style.clone()));
                    }
                    if card.block > 0 {
                        c.spawn(TextBundle::from_section(format!("Blk: {}", card.block), text_style.clone()));
                    }
                });
            }
        });

        // Return Button
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
            ReturnFromDeckButton,
        )).with_children(|p| {
            p.spawn(TextBundle::from_section("Back", TextStyle {
                font: Handle::default(),
                font_size: 25.0,
                color: Color::WHITE,
            }));
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
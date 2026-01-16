use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn update_health_ui(
    player_health_query: Query<&Health, (With<Player>, Changed<Health>)>,
    enemy_health_query: Query<&Health, (With<Enemy>, Changed<Health>)>,
    player_block_query: Query<&Block, (With<Player>, Changed<Block>)>,
    enemy_block_query: Query<&Block, (With<Enemy>, Changed<Block>)>,
    player_gold_query: Query<&Gold, (With<Player>, Changed<Gold>)>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<EnemyHealthText>>,
        Query<&mut Text, With<PlayerBlockText>>,
        Query<&mut Text, With<EnemyBlockText>>,
        Query<&mut Text, With<PlayerGoldText>>,
    )>,
) {
    if let Ok(health) = player_health_query.get_single() {
        for mut text in text_queries.p0().iter_mut() {
            text.sections[0].value = format!("Player: {}/{}", health.current, health.max);
        }
    }
    if let Ok(health) = enemy_health_query.get_single() {
        for mut text in text_queries.p1().iter_mut() {
            text.sections[0].value = format!("Enemy: {}/{}", health.current, health.max);
        }
    }
    if let Ok(block) = player_block_query.get_single() {
        for mut text in text_queries.p2().iter_mut() {
            text.sections[0].value = format!("Block: {}", block.value);
        }
    }
    if let Ok(block) = enemy_block_query.get_single() {
        for mut text in text_queries.p3().iter_mut() {
            text.sections[0].value = format!("Block: {}", block.value);
        }
    }
    if let Ok(gold) = player_gold_query.get_single() {
        for mut text in text_queries.p4().iter_mut() {
            text.sections[0].value = format!("Gold: {}", gold.amount);
        }
    }
}

pub fn update_pile_ui(
    deck: Res<Deck>,
    discard: Res<DiscardPile>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<DeckText>>,
        Query<&mut Text, With<DiscardText>>,
    )>,
) {
    if deck.is_changed() {
        for mut text in text_queries.p0().iter_mut() {
            text.sections[0].value = format!("Deck: {}", deck.cards.len());
        }
    }
    if discard.is_changed() {
        for mut text in text_queries.p1().iter_mut() {
            text.sections[0].value = format!("Discard: {}", discard.cards.len());
        }
    }
}

pub fn update_energy_ui(
    player_energy_query: Query<&Energy, (With<Player>, Changed<Energy>)>,
    mut energy_text_query: Query<&mut Text, With<PlayerEnergyText>>,
) {
    if let Ok(energy) = player_energy_query.get_single() {
        for mut text in &mut energy_text_query {
            text.sections[0].value = format!("Energy: {}/{}", energy.current, energy.max);
        }
    }
}

pub fn update_relic_ui(
    mut commands: Commands,
    player_relic_query: Query<&RelicStore, (With<Player>, Changed<RelicStore>)>,
    relic_ui_query: Query<Entity, With<PlayerRelicText>>,
    player_relics_all: Query<&RelicStore, With<Player>>,
    ui_added: Query<Entity, Added<PlayerRelicText>>,
) {
    let spawn_relics = |parent: &mut ChildBuilder, relics: &RelicStore| {
        for relic in &relics.relics {
            let (text, tooltip, color) = match relic {
                Relic::Vajra => ("V", "Vajra: +1 Strength.", Color::srgb(0.8, 0.2, 0.2)),
                Relic::BurningBlood => ("BB", "Burning Blood: Heal 6 HP at end of combat.", Color::srgb(0.6, 0.0, 0.0)),
            };
            
            parent.spawn((
                NodeBundle {
                    style: Style {
                        margin: UiRect::right(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(3.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                    background_color: color.into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                Interaction::None,
                Tooltip { text: tooltip.to_string() },
            )).with_children(|p| {
                p.spawn(TextBundle::from_section(text, TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    font: Handle::default(),
                }));
            });
        }
    };

    if !player_relic_query.is_empty() || !ui_added.is_empty() {
        if let (Ok(relics), Ok(ui_entity)) = (player_relics_all.get_single(), relic_ui_query.get_single()) {
            commands.entity(ui_entity).despawn_descendants();
            commands.entity(ui_entity).with_children(|parent| spawn_relics(parent, relics));
        }
    }
}
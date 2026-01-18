use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::item_relics::get_relic_visuals;

pub fn update_health_ui(
    player_health_query: Query<&Health, (With<Player>, Changed<Health>)>,
    player_block_query: Query<&Block, (With<Player>, Changed<Block>)>,
    player_gold_query: Query<&Gold, (With<Player>, Changed<Gold>)>,

    parents: Query<&Parent>,
    enemy_data: Query<(&Health, &Block), With<Enemy>>,

    mut text_queries: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<PlayerBlockText>>,
        Query<&mut Text, With<PlayerGoldText>>,
        Query<(&Parent, &mut Text), With<EnemyHealthText>>,
        Query<(&Parent, &mut Text), With<EnemyBlockText>>,
    )>,
) {
    if let Ok(health) = player_health_query.get_single() {
        for mut text in text_queries.p0().iter_mut() {
            text.sections[0].value = format!("Player: {}/{}", health.current, health.max);
        }
    }
    if let Ok(block) = player_block_query.get_single() {
        for mut text in text_queries.p1().iter_mut() {
            text.sections[0].value = format!("Block: {}", block.value);
        }
    }
    if let Ok(gold) = player_gold_query.get_single() {
        for mut text in text_queries.p2().iter_mut() {
            text.sections[0].value = format!("Gold: {}", gold.amount);
        }
    }

    // Update Enemy Health UI
    for (parent, mut text) in text_queries.p3().iter_mut() {
        // Hierarchy: Text -> Row -> Enemy
        if let Ok(grandparent) = parents.get(parent.get()) {
            if let Ok((health, _)) = enemy_data.get(grandparent.get()) {
                let new_text = format!("HP: {}/{}", health.current, health.max);
                if text.sections[0].value != new_text {
                    text.sections[0].value = new_text;
                }
            }
        }
    }

    // Update Enemy Block UI
    for (parent, mut text) in text_queries.p4().iter_mut() {
        if let Ok(grandparent) = parents.get(parent.get()) {
            if let Ok((_, block)) = enemy_data.get(grandparent.get()) {
                let new_text = format!("Block: {}", block.value);
                if text.sections[0].value != new_text {
                    text.sections[0].value = new_text;
                }
            }
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
    player_query: Query<(&Energy, Option<&Mana>), With<Player>>,
    mut energy_text_query: Query<&mut Text, With<PlayerEnergyText>>,
    run_state: Res<RunState>,
) {
    if let Ok((energy, mana)) = player_query.get_single() {
        for mut text in &mut energy_text_query {
            if run_state.character_class == CharacterClass::Spellweaver {
                if let Some(m) = mana {
                    text.sections[0].value = format!("Mana: {}", m.current);
                    text.sections[0].style.color = Color::srgb(0.8, 0.4, 1.0);
                }
            } else {
                text.sections[0].value = format!("Energy: {}/{}", energy.current, energy.max);
                text.sections[0].style.color = Color::srgb(0.2, 0.8, 1.0);
            }
        }
    }
}

pub fn update_spell_ui(
    mut commands: Commands,
    player_query: Query<&ActiveSpell, (With<Player>, Changed<ActiveSpell>)>,
    container_query: Query<Entity, With<PlayerSpellContainer>>,
) {
    if let Ok(active_spell) = player_query.get_single() {
        if let Ok(container) = container_query.get_single() {
            commands.entity(container).despawn_descendants();
            
            commands.entity(container).with_children(|parent| {
                for essence in &active_spell.essences {
                    let (color, text_code) = match essence.element {
                        SpellElement::Fire => (Color::srgb(0.8, 0.2, 0.2), "F"),
                        SpellElement::Ice => (Color::srgb(0.2, 0.8, 1.0), "I"),
                        SpellElement::Wind => (Color::srgb(0.8, 0.8, 0.8), "W"),
                        SpellElement::Stone => (Color::srgb(0.5, 0.3, 0.1), "S"),
                        SpellElement::Neutral => (Color::srgb(0.5, 0.5, 0.5), "N"),
                    };
                    
                    let mut tooltip_text = String::new();
                    match essence.element {
                        SpellElement::Fire => tooltip_text.push_str("Fire Essence\n"),
                        SpellElement::Ice => tooltip_text.push_str("Ice Essence\n"),
                        SpellElement::Wind => tooltip_text.push_str("Wind Essence\n"),
                        SpellElement::Stone => tooltip_text.push_str("Stone Essence\n"),
                        SpellElement::Neutral => tooltip_text.push_str("Essence\n"),
                    }
                    if essence.damage > 0 { tooltip_text.push_str(&format!("+{} Damage\n", essence.damage)); }
                    if essence.block > 0 { tooltip_text.push_str(&format!("+{} Block\n", essence.block)); }

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                margin: UiRect::right(Val::Px(4.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: color.into(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        },
                        Interaction::default(),
                        Tooltip { text: tooltip_text },
                    )).with_children(|p| {
                        p.spawn(TextBundle::from_section(text_code, TextStyle {
                            font: Handle::default(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        }));
                    });
                }
            });
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
            let (text, tooltip, color) = get_relic_visuals(relic);
            
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
                RelicIcon { relic: *relic },
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
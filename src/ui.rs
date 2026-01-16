use bevy::prelude::*;
use crate::components::*;

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

pub fn update_status_ui(
    player_status_query: Query<&StatusStore, (With<Player>, Changed<StatusStore>)>,
    enemy_status_query: Query<&StatusStore, (With<Enemy>, Changed<StatusStore>)>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<PlayerStatusText>>,
        Query<&mut Text, With<EnemyStatusText>>,
    )>,
) {
    if let Ok(status) = player_status_query.get_single() {
        for mut text in text_queries.p0().iter_mut() {
            let mut s = String::from("Status: ");
            if status.poison > 0 { s.push_str(&format!("Psn({}) ", status.poison)); }
            if status.weak > 0 { s.push_str(&format!("Wk({}) ", status.weak)); }
            text.sections[0].value = s;
        }
    }
    if let Ok(status) = enemy_status_query.get_single() {
        for mut text in text_queries.p1().iter_mut() {
            let mut s = String::from("Status: ");
            if status.poison > 0 { s.push_str(&format!("Psn({}) ", status.poison)); }
            if status.weak > 0 { s.push_str(&format!("Wk({}) ", status.weak)); }
            text.sections[0].value = s;
        }
    }
}

pub fn update_relic_ui(
    player_relic_query: Query<&RelicStore, (With<Player>, Changed<RelicStore>)>,
    mut text_query: Query<&mut Text, With<PlayerRelicText>>,
) {
    if let Ok(relics) = player_relic_query.get_single() {
        for mut text in &mut text_query {
            let mut s = String::from("Relics: ");
            for relic in &relics.relics {
                match relic {
                    Relic::Vajra => s.push_str("Vajra (+1 Dmg) "),
                    Relic::BurningBlood => s.push_str("Burning Blood (Heal 6) "),
                }
            }
            text.sections[0].value = s;
        }
    }
}
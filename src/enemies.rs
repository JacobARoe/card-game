use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::item_relics::Relic;

pub fn update_enemy_tooltip_system(
    mut query: Query<(&Enemy, &StatusStore, &NextEnemyMove, &mut Tooltip), Or<(Added<Enemy>, Changed<StatusStore>, Changed<NextEnemyMove>)>>,
) {
    for (_enemy, status, next_move, mut tooltip) in query.iter_mut() {
        if status.stun > 0 {
            tooltip.text = "Intent: Stunned\nCannot attack this turn.".to_string();
            continue;
        }

        let base_damage = next_move.damage;
        let move_name = &next_move.name;
        
        let mut final_damage = base_damage;
        if status.weak > 0 && base_damage > 0 {
            final_damage = (final_damage as f32 * 0.75) as i32;
        }
        
        let mut desc = format!("Intent: {}\n", move_name);
        if base_damage > 0 { desc.push_str(&format!("Damage: {} (Base: {})\n", final_damage, base_damage)); }
        if next_move.block > 0 { desc.push_str(&format!("Block: {}\n", next_move.block)); }
        if next_move.poison > 0 { desc.push_str(&format!("Apply {} Poison\n", next_move.poison)); }
        if next_move.weak > 0 { desc.push_str(&format!("Apply {} Weak\n", next_move.weak)); }
        if next_move.steal_gold > 0 { desc.push_str(&format!("Steals {} Gold\n", next_move.steal_gold)); }
        if next_move.is_charging { desc.push_str("Charging up...\n"); }

        tooltip.text = desc;
    }
}

pub fn enemy_turn_system(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Health, &mut Block, &RelicStore, &mut Gold, &mut StatusStore), With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Block, &mut Health, &mut StatusStore, &mut NextEnemyMove), Without<Player>>,
    mut intent_text_query: Query<&mut Text, With<EnemyIntentText>>,
    mut flash_query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
    relic_ui_query: Query<(&RelicIcon, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut game_map: ResMut<GameMap>,
) {
    let (enemy, mut enemy_block, mut enemy_health, mut enemy_status, mut next_move) = enemy_query.single_mut();
    
    if enemy_status.poison > 0 {
        enemy_health.current -= enemy_status.poison;
        enemy_status.poison -= 1;
    }
    if enemy_health.current <= 0 {
        if let Ok((mut p_health, _, p_relics, _, _)) = player_query.get_single_mut() {
            if p_relics.relics.contains(&Relic::BurningBlood) {
                p_health.current = (p_health.current + 6).min(p_health.max);
                println!("Burning Blood heals 6 HP!");

                // Spawn particles for Burning Blood
                if let Ok(window) = window_query.get_single() {
                    for (icon, transform) in &relic_ui_query {
                        if icon.relic == Relic::BurningBlood {
                            let half_w = window.width() / 2.0;
                            let half_h = window.height() / 2.0;
                            let pos = transform.translation();
                            
                            for _ in 0..20 {
                                let mut rng = thread_rng();
                                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                                let speed = rng.gen_range(30.0..100.0);
                                let vx = angle.cos() * speed;
                                let vy = angle.sin() * speed;

                                commands.spawn((
                                    NodeBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(pos.x + half_w),
                                            bottom: Val::Px(pos.y + half_h),
                                            width: Val::Px(4.0),
                                            height: Val::Px(4.0),
                                            ..default()
                                        },
                                        background_color: Color::srgb(1.0, 0.2, 0.2).into(),
                                        z_index: ZIndex::Global(200),
                                        ..default()
                                    },
                                    Particle {
                                        velocity: Vec2::new(vx, vy),
                                        lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                                    },
                                    BattleEntity,
                                ));
                            }
                        }
                    }
                }
            }
        }
        // Reveal next nodes on map
        if let Some((level, index)) = game_map.current_node {
            game_map.visited_path.push((level, index));
            let visited = game_map.visited_path.clone();
            for l in 0..=level {
                for (i, node) in game_map.levels[l].iter_mut().enumerate() {
                    if !visited.contains(&(l, i)) {
                        node.visible = false;
                    }
                }
            }
            if level + 1 < game_map.levels.len() {
                let next_indices = game_map.levels[level][index].next_indices.clone();
                for next_idx in next_indices {
                    game_map.levels[level + 1][next_idx].visible = true;
                }
            }
        }
        next_game_state.set(GameState::Victory);
        return;
    }
    
    enemy_block.value = 0;

    if enemy_status.stun > 0 {
        println!("Enemy is stunned!");
        enemy_status.stun -= 1;
        
        for mut text in &mut intent_text_query {
            text.sections[0].value = "Stunned!".to_string();
        }

        next_turn_state.set(TurnState::PlayerTurnStart);
        println!("Player's turn.");
        return;
    }

    let mut final_damage = next_move.damage;
    if enemy_status.weak > 0 && final_damage > 0 {
        final_damage = (final_damage as f32 * 0.75) as i32;
        enemy_status.weak -= 1;
    }

    println!("Enemy {}!", next_move.name);
    
    for mut text in &mut intent_text_query {
        text.sections[0].value = format!("{}! ({} dmg)", next_move.name, final_damage);
    }

    // Apply Enemy Block
    if next_move.block > 0 {
        enemy_block.value += next_move.block;
        println!("Enemy gained {} block", next_move.block);
    }

    for (mut player_health, mut player_block, _, mut player_gold, mut player_status) in player_query.iter_mut() {
        let block_damage = std::cmp::min(final_damage, player_block.value);
        let actual_damage = final_damage - block_damage;
        player_block.value -= block_damage;
        player_health.current -= actual_damage;
        println!("Player Health: {}/{}", player_health.current, player_health.max);
        if actual_damage > 0 {
            for mut bg in &mut flash_query {
                bg.0 = Color::srgba(1.0, 0.0, 0.0, 0.5).into();
            }
        }

        if next_move.poison > 0 { player_status.poison += next_move.poison; }
        if next_move.weak > 0 { player_status.weak += next_move.weak; }
        if next_move.steal_gold > 0 {
            let stolen = std::cmp::min(player_gold.amount, next_move.steal_gold);
            player_gold.amount -= stolen;
            println!("Enemy stole {} gold!", stolen);
        }

        if player_health.current <= 0 {
            next_game_state.set(GameState::GameOver);
            return;
        }
    }

    // Generate Next Move
    *next_move = generate_enemy_move(enemy.kind, next_move.is_charging);

    next_turn_state.set(TurnState::PlayerTurnStart);
    println!("Player's turn.");
}

// Helper to generate moves
pub fn generate_enemy_move(kind: EnemyKind, prev_was_charging: bool) -> NextEnemyMove {
    if prev_was_charging {
        return NextEnemyMove {
            name: "Fire Breath".to_string(),
            damage: 50,
            block: 0,
            poison: 0,
            weak: 0,
            steal_gold: 0,
            is_charging: false,
        };
    }

    let mut rng = thread_rng();
    let roll = rng.gen_range(0..100);

    match kind {
        EnemyKind::Goblin => {
            if roll < 60 {
                NextEnemyMove { name: "Stabs".to_string(), damage: 5, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else {
                NextEnemyMove { name: "Thieve".to_string(), damage: 3, block: 0, poison: 0, weak: 0, steal_gold: 10, is_charging: false }
            }
        },
        EnemyKind::Orc => {
            if roll < 50 {
                NextEnemyMove { name: "Smashes".to_string(), damage: 12, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else if roll < 80 {
                NextEnemyMove { name: "Heavy Blow".to_string(), damage: 18, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else {
                NextEnemyMove { name: "Defends".to_string(), damage: 0, block: 15, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            }
        },
        EnemyKind::DarkKnight => {
            if roll < 40 {
                NextEnemyMove { name: "Executes".to_string(), damage: 18, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else if roll < 70 {
                NextEnemyMove { name: "Obliterates".to_string(), damage: 25, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else {
                NextEnemyMove { name: "Dark Magic".to_string(), damage: 10, block: 0, poison: 3, weak: 0, steal_gold: 0, is_charging: false }
            }
        },
        EnemyKind::Dragon => {
            if roll < 40 {
                NextEnemyMove { name: "Incinerates".to_string(), damage: 25, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: false }
            } else if roll < 70 {
                NextEnemyMove { name: "Roar".to_string(), damage: 15, block: 0, poison: 0, weak: 2, steal_gold: 0, is_charging: false }
            } else {
                NextEnemyMove { name: "Deep Breath".to_string(), damage: 0, block: 0, poison: 0, weak: 0, steal_gold: 0, is_charging: true }
            }
        },
    }
}
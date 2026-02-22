use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::components::*;
use crate::item_relics::Relic;
use crate::resources::*;
use crate::states::*;

pub fn update_enemy_tooltip_system(
    mut query: Query<
        (&Enemy, &StatusStore, &NextEnemyMove, &mut Tooltip),
        Or<(Added<Enemy>, Changed<StatusStore>, Changed<NextEnemyMove>)>,
    >,
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
        if base_damage > 0 {
            desc.push_str(&format!(
                "Damage: {} (Base: {})\n",
                final_damage, base_damage
            ));
        }
        if next_move.block > 0 {
            desc.push_str(&format!("Block: {}\n", next_move.block));
        }
        if next_move.poison > 0 {
            desc.push_str(&format!("Apply {} Poison\n", next_move.poison));
        }
        if next_move.weak > 0 {
            desc.push_str(&format!("Apply {} Weak\n", next_move.weak));
        }
        if next_move.steal_gold > 0 {
            desc.push_str(&format!("Steals {} Gold\n", next_move.steal_gold));
        }
        if next_move.is_charging {
            desc.push_str("Charging up...\n");
        }

        tooltip.text = desc;
    }
}

pub fn enemy_turn_system(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    enemy_query: Query<(Entity, &Health), (With<Enemy>, Without<Player>)>,
    mut queue_query: Query<(Entity, &mut EnemyQueue)>,
) {
    if queue_query.is_empty() {
        // Collect living enemies
        let mut queue = Vec::new();
        for (entity, health) in enemy_query.iter() {
            if health.current > 0 {
                queue.push(entity);
            }
        }
        commands.spawn((EnemyQueue(queue), BattleEntity));
        return; // Next tick will pick it up
    }

    let (queue_entity, mut queue) = queue_query.single_mut();

    // Process next enemy
    while !queue.0.is_empty() {
        let next_enemy = queue.0.remove(0);

        // Ensure enemy is still alive
        if let Ok((_, health)) = enemy_query.get(next_enemy) {
            if health.current > 0 {
                commands.entity(next_enemy).insert(AttackingEnemy);
                // We keep the queue_entity alive to resume processing the remaining queue
                next_turn_state.set(TurnState::EnemyAttackAnimating);
                return;
            }
        }
    }

    // Finished
    commands.entity(queue_entity).despawn_recursive();
    next_turn_state.set(TurnState::PlayerTurnStart);
    println!("Player's turn.");
}

pub fn enemy_attack_animating_system(
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut enemy_query: Query<
        (
            Entity,
            &Enemy,
            &mut Block,
            &mut Health,
            &mut StatusStore,
            &mut NextEnemyMove,
        ),
        (With<AttackingEnemy>, Without<Player>),
    >,
    mut player_query: Query<
        (Entity, &mut Health, &mut Block, &mut Gold, &mut StatusStore),
        (With<Player>, Without<Enemy>),
    >,
    mut intent_text_query: Query<(&Parent, &mut Text), With<EnemyIntentText>>,
    mut flash_query: Query<&mut BackgroundColor, With<DamageFlashUi>>,
    mut reflex_query: Query<(Entity, Option<&Parent>, &mut ReflexState)>,
    window_query: Query<&Window>,
    time: Res<Time>,
) {
    let (enemy_entity, enemy, mut enemy_block, mut enemy_health, mut enemy_status, mut next_move) =
        if let Ok(e) = enemy_query.get_single_mut() {
            e
        } else {
            // fallback if missing
            next_turn_state.set(TurnState::EnemyTurn);
            return;
        };

    let (player_entity, mut player_health, mut player_block, mut player_gold, mut player_status) =
        if let Ok(p) = player_query.get_single_mut() {
            p
        } else {
            return;
        };

    let window = window_query.single();
    let cx = window.width() / 2.0;
    let cy = window.height() / 2.0;

    // Phase 1: Initialize Reflex UI (first frame)
    if reflex_query.is_empty() {
        // Pre-calculate enemy damage to see if we even need a reflex (e.g., if stunned or not attacking)

        // Status Pre-Processing
        if enemy_status.poison > 0 {
            enemy_health.current -= enemy_status.poison;
            enemy_status.poison -= 1;
        }
        if enemy_status.burning > 0 {
            enemy_health.current -= enemy_status.burning;
            println!("Enemy takes {} burning damage!", enemy_status.burning);
            enemy_status.burning -= 1;
        }

        if enemy_health.current <= 0 {
            // Dead before attack
            commands.entity(enemy_entity).despawn_recursive();

            // Re-check victory
            let enemies_remaining = enemy_query.iter().count() - 1; // Since this one is dying
            if enemies_remaining == 0 {
                next_game_state.set(GameState::Victory);
            } else {
                next_turn_state.set(TurnState::EnemyTurn);
            }
            return;
        }

        enemy_block.value = 0;

        if enemy_status.stun > 0 {
            println!("Enemy is stunned!");
            enemy_status.stun -= 1;
            for (parent, mut text) in &mut intent_text_query {
                if parent.get() == enemy_entity {
                    text.sections[0].value = "Stunned!".to_string();
                }
            }
            commands.entity(enemy_entity).remove::<AttackingEnemy>();
            next_turn_state.set(TurnState::EnemyTurn);
            return;
        }

        let mut final_damage = next_move.damage;
        if enemy_status.weak > 0 && final_damage > 0 {
            final_damage = (final_damage as f32 * 0.75) as i32;
            enemy_status.weak -= 1;
        }
        if enemy_status.frozen > 0 && final_damage > 0 {
            final_damage = (final_damage as f32 * 0.75) as i32;
        }

        for (parent, mut text) in &mut intent_text_query {
            if parent.get() == enemy_entity {
                text.sections[0].value = format!("{}! ({} dmg)", next_move.name, final_damage);
            }
        }

        if next_move.block > 0 {
            enemy_block.value += next_move.block;
            println!("Enemy gained {} block", next_move.block);
        }

        if final_damage > 0 {
            // Spawn Reflex State / UI
            let duration = 1.0; // 1 second telegraph
            commands
                .spawn((
                    ReflexState {
                        start_time: 0.0,
                        timer: Timer::from_seconds(duration, TimerMode::Once),
                        perfect_window_start: 0.7,
                        perfect_window_end: 0.9,
                        result: None,
                        is_defensive: true,
                        source_entity: Some(enemy_entity),
                        target_entity: Some(player_entity),
                        base_damage: final_damage,
                        visual_type: ReflexVisualType::ShrinkingRing,
                    },
                    BattleEntity,
                    ReflexUI,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(cx - 150.0),
                            bottom: Val::Px(cy - 150.0),
                            width: Val::Px(300.0),
                            height: Val::Px(300.0),
                            border: UiRect::all(Val::Px(4.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                        border_color: Color::WHITE.into(),
                        z_index: ZIndex::Global(1000),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Target inner ring (static)
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(90.0), // 30% of 300, matching the 0.7 timer end roughly
                            height: Val::Px(90.0),
                            border: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        border_color: Color::srgba(0.0, 1.0, 0.0, 0.8).into(),
                        ..default()
                    });
                });
        } else {
            // No damage attack, skip reflex directly to resolution
            if next_move.poison > 0 {
                player_status.poison += next_move.poison;
            }
            if next_move.weak > 0 {
                player_status.weak += next_move.weak;
            }
            if next_move.steal_gold > 0 {
                let stolen = std::cmp::min(player_gold.amount, next_move.steal_gold);
                player_gold.amount -= stolen;
            }

            *next_move = generate_enemy_move(enemy.kind, next_move.is_charging);
            commands.entity(enemy_entity).remove::<AttackingEnemy>();
            next_turn_state.set(TurnState::EnemyTurn);
        }
        return;
    }

    // Phase 2: Update Timer and Wait for Resolution
    let (reflex_ent, reflex_parent, mut reflex) = reflex_query.single_mut();
    reflex.timer.tick(time.delta());

    if reflex.timer.finished() || reflex.result.is_some() {
        // Resolution!
        let mut final_damage = reflex.base_damage;
        let mut is_perfect = false;

        match reflex.result {
            Some(ReflexSuccess::Perfect) => {
                println!("PERFECT BLOCK!");
                final_damage = (final_damage as f32 * 0.1) as i32; // Take 10% damage
                is_perfect = true;
            }
            Some(ReflexSuccess::Good) => {
                println!("GOOD BLOCK!");
                final_damage = (final_damage as f32 * 0.5) as i32; // Take 50% damage
            }
            _ => {
                println!("MISSED BLOCK!");
            }
        }

        let block_damage = std::cmp::min(final_damage, player_block.value);
        let actual_damage = final_damage - block_damage;
        player_block.value -= block_damage;
        player_health.current -= actual_damage;

        if actual_damage > 0 {
            for mut bg in &mut flash_query {
                bg.0 = Color::srgba(1.0, 0.0, 0.0, 0.5).into();
            }
        }

        if is_perfect {
            for mut bg in &mut flash_query {
                bg.0 = Color::srgba(1.0, 1.0, 1.0, 0.5).into(); // White flash
            }
            // Spawn perfect block particles
            for _ in 0..10 {
                let mut rng = rand::thread_rng();
                use rand::Rng;
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(50.0..200.0);
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(cx),
                            bottom: Val::Px(cy),
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        z_index: ZIndex::Global(950),
                        ..default()
                    },
                    Particle {
                        velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                    },
                    BattleEntity,
                ));
            }
        }

        if next_move.poison > 0 {
            player_status.poison += next_move.poison;
        }
        if next_move.weak > 0 {
            player_status.weak += next_move.weak;
        }
        if next_move.steal_gold > 0 {
            let stolen = std::cmp::min(player_gold.amount, next_move.steal_gold);
            player_gold.amount -= stolen;
        }

        *next_move = generate_enemy_move(enemy.kind, next_move.is_charging);

        if enemy_status.frozen > 0 {
            enemy_status.frozen -= 1;
        }

        if let Some(parent) = reflex_parent {
            commands.entity(parent.get()).despawn_recursive();
        } else {
            commands.entity(reflex_ent).despawn_recursive();
        }
        commands.entity(enemy_entity).remove::<AttackingEnemy>();

        if player_health.current <= 0 {
            next_game_state.set(GameState::GameOver);
        } else {
            next_turn_state.set(TurnState::EnemyTurn);
        }
    }
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
                NextEnemyMove {
                    name: "Stabs".to_string(),
                    damage: 5,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else {
                NextEnemyMove {
                    name: "Thieve".to_string(),
                    damage: 3,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 10,
                    is_charging: false,
                }
            }
        }
        EnemyKind::Orc => {
            if roll < 50 {
                NextEnemyMove {
                    name: "Smashes".to_string(),
                    damage: 12,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else if roll < 80 {
                NextEnemyMove {
                    name: "Heavy Blow".to_string(),
                    damage: 18,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else {
                NextEnemyMove {
                    name: "Defends".to_string(),
                    damage: 0,
                    block: 15,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            }
        }
        EnemyKind::DarkKnight => {
            if roll < 40 {
                NextEnemyMove {
                    name: "Executes".to_string(),
                    damage: 18,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else if roll < 70 {
                NextEnemyMove {
                    name: "Obliterates".to_string(),
                    damage: 25,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else {
                NextEnemyMove {
                    name: "Dark Magic".to_string(),
                    damage: 10,
                    block: 0,
                    poison: 3,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            }
        }
        EnemyKind::Dragon => {
            if roll < 40 {
                NextEnemyMove {
                    name: "Incinerates".to_string(),
                    damage: 25,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else if roll < 70 {
                NextEnemyMove {
                    name: "Roar".to_string(),
                    damage: 15,
                    block: 0,
                    poison: 0,
                    weak: 2,
                    steal_gold: 0,
                    is_charging: false,
                }
            } else {
                NextEnemyMove {
                    name: "Deep Breath".to_string(),
                    damage: 0,
                    block: 0,
                    poison: 0,
                    weak: 0,
                    steal_gold: 0,
                    is_charging: true,
                }
            }
        }
    }
}

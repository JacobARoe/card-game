use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;

pub fn setup_map_screen(mut commands: Commands, game_map: Res<GameMap>) {
    let map_root = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgb(0.02, 0.02, 0.05).into(), // Darker atmosphere
            ..default()
        },
        MapUI,
    )).id();

    let window_width = 1280.0;
    let window_height = 720.0;
    let level_count = game_map.levels.len();
    let x_step = (window_width - 200.0) / (level_count as f32 - 1.0);

    let get_px_pos = |level: usize, index: usize, total: usize| -> Vec2 {
        let x = 100.0 + x_step * level as f32;
        let base_y = window_height / 2.0 + (index as f32 - (total as f32 - 1.0) / 2.0) * 120.0;
        let y = base_y; // + game_map.levels[level][index].y_jitter;
        Vec2::new(x, y)
    };

    // Spawn Paths
    for (l_idx, level) in game_map.levels.iter().enumerate() {
        if l_idx == game_map.levels.len() - 1 { break; }
        for (n_idx, node) in level.iter().enumerate() {
            let start_pos = get_px_pos(l_idx, n_idx, level.len());
            
            for &next_idx in &node.next_indices {
                let end_pos = get_px_pos(l_idx + 1, next_idx, game_map.levels[l_idx+1].len());
                
                let diff = end_pos - start_pos;
                let length = diff.length();
                let angle = diff.y.atan2(diff.x);
                let center = (start_pos + end_pos) / 2.0;
                
                let is_path_taken = game_map.visited_path.contains(&(l_idx, n_idx)) 
                    && game_map.visited_path.contains(&(l_idx + 1, next_idx));

                if is_path_taken {
                    let height = 6.0;
                    commands.spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(center.x - length / 2.0),
                                bottom: Val::Px(center.y - height / 2.0),
                                width: Val::Px(length),
                                height: Val::Px(height),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.8, 0.2).into(),
                            transform: Transform::from_rotation(Quat::from_rotation_z(angle)),
                            ..default()
                        },
                        MapUI,
                    )).set_parent(map_root);
                }
            }
        }
    }

    // Spawn Nodes
    for (l_idx, level) in game_map.levels.iter().enumerate() {
        for (n_idx, node) in level.iter().enumerate() {
            let pos = get_px_pos(l_idx, n_idx, level.len());
            
            let is_current = game_map.current_node == Some((l_idx, n_idx));
            let is_visited = game_map.visited_path.contains(&(l_idx, n_idx));
            
            let is_available = if let Some((curr_l, curr_i)) = game_map.current_node {
                l_idx == curr_l + 1 && game_map.levels[curr_l][curr_i].next_indices.contains(&n_idx)
            } else {
                // Start of game: Level 0 is available
                l_idx == 0
            };
            
            // Fog of War Logic: Only show Boss, Visited, Current, and Immediate Next nodes
            let is_visible = if node.node_type == NodeType::Boss {
                true
            } else if is_visited || is_current || is_available {
                true
            } else {
                false
            };

            let color = if is_current {
                Color::srgb(0.0, 1.0, 0.0) // Bright Green
            } else if is_visited {
                Color::srgb(0.4, 0.4, 0.4) // Dim Grey
            } else if is_available {
                match node.node_type {
                    NodeType::Boss => Color::srgb(0.8, 0.0, 0.0),
                    NodeType::Elite => Color::srgb(0.8, 0.0, 0.8), // Purple
                    NodeType::Event => Color::srgb(0.5, 0.5, 0.8), // Blue-ish
                    NodeType::Shop => Color::srgb(0.8, 0.6, 0.0), // Gold
                    NodeType::Rest => Color::srgb(0.2, 0.6, 0.8), // Cyan
                    NodeType::Battle => Color::WHITE,
                }
            } else if is_visible {
                Color::srgb(0.6, 0.0, 0.0) // Visible but not reachable (Boss)
            } else {
                Color::srgb(0.1, 0.1, 0.15) // Unknown / Dark Blue-Grey
            };
            
            let size = if node.node_type == NodeType::Boss { 60.0 } else if node.node_type == NodeType::Elite { 50.0 } else { 40.0 };
            
            let (label, tooltip_text) = if is_visible {
                match node.node_type {
                    NodeType::Battle => ("B", "Battle: Standard enemy encounter"),
                    NodeType::Shop => ("S", "Shop: Trade gold for goods"),
                    NodeType::Rest => ("R", "Rest: Heal or upgrade"),
                    NodeType::Boss => ("BOSS", "Boss: The final challenge"),
                    NodeType::Elite => ("E", "Elite: Powerful enemy, rare rewards"),
                    NodeType::Event => ("?", "Event: Unknown occurrence"),
                }
            } else {
                ("?", "Unknown Location")
            };

            let mut node_entity = commands.spawn((
                ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(pos.x - size/2.0),
                        bottom: Val::Px(pos.y - size/2.0),
                        width: Val::Px(size),
                        height: Val::Px(size),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: color.into(),
                    border_color: Color::WHITE.into(),
                    border_radius: BorderRadius::all(Val::Percent(if is_visible { 50.0 } else { 20.0 })),
                    ..default()
                },
                MapNodeButton { level: l_idx, index: n_idx, node_type: node.node_type },
                MapUI,
                Tooltip { text: tooltip_text.to_string() },
            ));
            
            node_entity.set_parent(map_root);
            
            node_entity.with_children(|p| {
                p.spawn(TextBundle::from_section(label, TextStyle {
                    font: Handle::default(),
                    font_size: if is_visible { 16.0 } else { 24.0 },
                    color: if is_visible { Color::BLACK } else { Color::srgb(0.5, 0.5, 0.5) },
                }));
            });
        }

        // View Deck Button
        commands.spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.2, 0.6).into(),
                ..default()
            },
            ViewDeckButton,
            MapUI,
        )).set_parent(map_root).with_children(|p| {
            p.spawn(TextBundle::from_section("View Deck", TextStyle {
                font: Handle::default(),
                font_size: 20.0,
                color: Color::WHITE,
            }));
        });
    }
}

pub fn map_interaction_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_map: ResMut<GameMap>,
    node_interaction_query: Query<(&Interaction, &MapNodeButton), (Changed<Interaction>, With<Button>)>,
    view_deck_query: Query<&Interaction, (Changed<Interaction>, With<ViewDeckButton>)>,
) {
    for (interaction, node_btn) in &node_interaction_query {
        if *interaction == Interaction::Pressed {
            // Check if valid move
            let is_available = if let Some((curr_l, curr_i)) = game_map.current_node {
                node_btn.level == curr_l + 1 && game_map.levels[curr_l][curr_i].next_indices.contains(&node_btn.index)
            } else {
                // Start of game
                node_btn.level == 0
            };

            if is_available {
                // Update Map State
                game_map.current_node = Some((node_btn.level, node_btn.index));
                game_map.visited_path.push((node_btn.level, node_btn.index));
                
                // Transition
                match node_btn.node_type {
                    NodeType::Battle | NodeType::Boss | NodeType::Elite => next_game_state.set(GameState::Battle),
                    NodeType::Shop => next_game_state.set(GameState::Shop),
                    NodeType::Rest => next_game_state.set(GameState::Rest),
                    NodeType::Event => next_game_state.set(GameState::Event),
                }
            }
        }
    }

    for interaction in &view_deck_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::ViewDeck);
        }
    }
}
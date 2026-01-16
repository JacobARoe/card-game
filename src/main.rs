use bevy::prelude::*;

mod components;
mod resources;
mod states;
mod battle;
mod common;
mod map;
mod rest;
mod shop;
mod ui;

use components::*;
use states::*;
use battle::*;
use common::*;
use map::*;
use rest::*;
use shop::*;
use ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds windowing, input, etc.
        .init_state::<GameState>()
        .init_state::<TurnState>()
        .add_systems(Startup, (setup_camera, setup_game))
        .add_systems(OnEnter(GameState::Battle), setup_battle)
        .add_systems(
            Update,
            (
                play_card_system.run_if(in_state(TurnState::PlayerTurn)),
                enemy_turn_system.run_if(in_state(TurnState::EnemyTurn)),
                card_hover_system,
                update_health_ui,
                update_energy_ui,
                update_status_visuals_system,
                tooltip_system,
                update_relic_ui,
                update_damage_flash_system,
                end_turn_button_system.run_if(in_state(TurnState::PlayerTurn)),
            ).run_if(in_state(GameState::Battle))
        )
        .add_systems(OnEnter(TurnState::PlayerTurn), draw_cards_system)
        .add_systems(OnExit(TurnState::PlayerTurn), discard_hand_system)
        .add_systems(OnExit(GameState::Battle), (despawn_screen::<BattleEntity>, reset_turn_state))
        .add_systems(OnEnter(GameState::Victory), setup_victory_screen)
        .add_systems(Update, victory_interaction_system.run_if(in_state(GameState::Victory)))
        .add_systems(OnExit(GameState::Victory), despawn_screen::<VictoryUI>)
        .add_systems(OnEnter(GameState::Map), setup_map_screen)
        .add_systems(Update, map_interaction_system.run_if(in_state(GameState::Map)))
        .add_systems(OnExit(GameState::Map), despawn_screen::<MapUI>)
        .add_systems(OnEnter(GameState::Shop), setup_shop_screen)
        .add_systems(Update, (shop_interaction_system, shop_nav_system).run_if(in_state(GameState::Shop)))
        .add_systems(OnExit(GameState::Shop), despawn_screen::<ShopUI>)
        .add_systems(OnEnter(GameState::Rest), setup_rest_screen)
        .add_systems(Update, rest_interaction_system.run_if(in_state(GameState::Rest)))
        .add_systems(OnExit(GameState::Rest), despawn_screen::<RestUI>)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
        .add_systems(Update, game_over_interaction_system.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), (despawn_screen::<GameOverUI>, setup_game))
        .run();
}

use bevy::prelude::*;

mod components;
mod resources;
mod states;
mod common;
mod ui;
mod scene_battle;
mod scene_map;
mod scene_rest;
mod scene_shop;
mod scene_menu;
mod scene_deck;
mod scene_discard;
mod scene_event;
mod item_cards;
mod item_relics;
mod item_potions;
mod scene_rewards;
mod scene_game_over;
mod enemies;

use components::*;
use states::*;
use common::*;
use ui::*;
use scene_battle::*;
use scene_map::*;
use scene_rest::*;
use scene_shop::*;
use scene_menu::*;
use scene_deck::*;
use scene_discard::*;
use scene_event::*;
use scene_rewards::*;
use scene_game_over::*;
use enemies::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds windowing, input, etc.
        .init_state::<GameState>()
        .init_state::<TurnState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Update, menu_interaction_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (despawn_screen::<MainMenuUI>, setup_game))
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
                update_potion_ui,
                potion_interaction_system,
                update_pile_ui,
                update_particles_system,
                update_damage_flash_system,
                update_block_flash_system,
                update_enemy_tooltip_system,
                end_turn_button_system.run_if(in_state(TurnState::PlayerTurn)),
                discard_pile_click_system.run_if(in_state(TurnState::PlayerTurn)),
                resize_background_system,
            ).run_if(in_state(GameState::Battle))
        )
        .add_systems(OnEnter(TurnState::PlayerTurnStart), draw_cards_system)
        .add_systems(OnEnter(TurnState::PlayerTurnEnd), discard_hand_system)
        .add_systems(OnExit(GameState::Battle), (cleanup_battle_deck, despawn_screen::<BattleEntity>, reset_turn_state).chain())
        .add_systems(OnEnter(GameState::Victory), setup_victory_screen)
        .add_systems(Update, reward_interaction_system.run_if(in_state(GameState::Victory)))
        .add_systems(OnExit(GameState::Victory), despawn_screen::<RewardUI>)
        .add_systems(OnEnter(GameState::RewardSelectCard), setup_reward_select_card_screen)
        .add_systems(Update, reward_select_card_interaction_system.run_if(in_state(GameState::RewardSelectCard)))
        .add_systems(OnExit(GameState::RewardSelectCard), despawn_screen::<RewardSelectCardUI>)
        .add_systems(OnEnter(GameState::Map), setup_map_screen)
        .add_systems(Update, (map_interaction_system, tooltip_system).run_if(in_state(GameState::Map)))
        .add_systems(OnExit(GameState::Map), (despawn_screen::<MapUI>, despawn_screen::<TooltipUi>))
        .add_systems(OnEnter(GameState::Shop), setup_shop_screen)
        .add_systems(Update, (shop_interaction_system, shop_nav_system, update_shop_gold_ui, resize_background_system).run_if(in_state(GameState::Shop)))
        .add_systems(OnExit(GameState::Shop), despawn_screen::<ShopUI>)
        .add_systems(OnEnter(GameState::ShopRemoveCard), setup_shop_remove_screen)
        .add_systems(Update, shop_remove_system.run_if(in_state(GameState::ShopRemoveCard)))
        .add_systems(OnExit(GameState::ShopRemoveCard), despawn_screen::<ShopRemoveUI>)
        .add_systems(OnEnter(GameState::Rest), setup_rest_screen)
        .add_systems(Update, (rest_interaction_system, resize_background_system).run_if(in_state(GameState::Rest)))
        .add_systems(OnExit(GameState::Rest), despawn_screen::<RestUI>)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
        .add_systems(Update, game_over_interaction_system.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), (despawn_screen::<GameOverUI>, setup_game))
        .add_systems(OnEnter(GameState::ViewDeck), setup_view_deck_screen)
        .add_systems(Update, view_deck_interaction_system.run_if(in_state(GameState::ViewDeck)))
        .add_systems(OnExit(GameState::ViewDeck), despawn_screen::<ViewDeckUI>)
        .add_systems(OnEnter(TurnState::ViewingDiscard), setup_view_discard_overlay)
        .add_systems(Update, view_discard_interaction_system.run_if(in_state(TurnState::ViewingDiscard)))
        .add_systems(OnExit(TurnState::ViewingDiscard), despawn_screen::<ViewDiscardUI>)
        .add_systems(OnEnter(GameState::Event), setup_event_screen)
        .add_systems(Update, event_interaction_system.run_if(in_state(GameState::Event)))
        .add_systems(OnExit(GameState::Event), despawn_screen::<EventUI>)
        .run();
}

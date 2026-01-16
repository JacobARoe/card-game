use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Battle,
    Victory,
    Shop,
    Rest,
    Map,
    GameOver,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
}
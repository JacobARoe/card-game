use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Battle,
    Victory,
    Shop,
    Rest,
    Map,
    GameOver,
    ViewDeck,
    ShopRemoveCard,
    RewardSelectCard,
    Event,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    #[default]
    Setup,
    PlayerTurnStart,
    PlayerTurn,
    PlayerTurnEnd,
    EnemyTurn,
    ViewingDiscard,
}
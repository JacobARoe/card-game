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
    CharacterSelect,
    BonusSelect,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    #[default]
    Setup,
    PlayerTurnStart,
    PlayerTurn,
    PlayerAttackAnimating,
    PlayerTurnEnd,
    EnemyTurn,
    EnemyAttackAnimating,
    ViewingDiscard,
}

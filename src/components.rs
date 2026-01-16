use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Clone, Copy)]
pub enum EnemyKind {
    Goblin,
    Orc,
}

#[derive(Component)]
pub struct Enemy {
    pub kind: EnemyKind,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Energy {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Default)]
pub struct Block {
    pub value: i32,
}

#[derive(Component, Default)]
pub struct StatusStore {
    pub poison: i32,
    pub weak: i32,
}

#[derive(Component)]
pub struct Gold {
    pub amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relic {
    Vajra,
    BurningBlood,
}

#[derive(Component, Default)]
pub struct RelicStore {
    pub relics: Vec<Relic>,
}

#[derive(Component, Debug, Clone)]
pub struct Card {
    pub name: String,
    pub damage: i32,
    pub block: i32,
    pub cost: i32,
    pub apply_poison: i32,
    pub apply_weak: i32,
    pub upgraded: bool,
}

#[derive(Component)]
pub struct BaseColor(pub Color);

#[derive(Component)]
pub struct BattleEntity;

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct RestUI;

#[derive(Component)]
pub struct VictoryUI;

#[derive(Component)]
pub struct MapUI;

#[derive(Component)]
pub struct PlayerHealthText;

#[derive(Component)]
pub struct EnemyHealthText;

#[derive(Component)]
pub struct PlayerGoldText;

#[derive(Component)]
pub struct PlayerEnergyText;

#[derive(Component)]
pub struct PlayerBlockText;

#[derive(Component)]
pub struct EnemyBlockText;

#[derive(Component)]
pub struct PlayerStatusText;

#[derive(Component)]
pub struct EnemyStatusText;

#[derive(Component)]
pub struct PlayerRelicText;

#[derive(Component)]
pub struct HandContainer;

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct EnemyIntentText;

#[derive(Component)]
pub struct EnterBattleButton;

#[derive(Component)]
pub struct VisitShopButton;

#[derive(Component)]
pub struct VisitRestButton;

#[derive(Component)]
pub struct LeaveShopButton;

#[derive(Component)]
pub struct BuyCardButton {
    pub card: Card,
    pub cost: i32,
}

#[derive(Component)]
pub struct BuyRelicButton {
    pub relic: Relic,
    pub cost: i32,
}

#[derive(Component)]
pub struct HealButton;

#[derive(Component)]
pub struct UpgradeButton;

#[derive(Component)]
pub struct LeaveRestButton;
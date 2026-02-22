use crate::item_potions::Potion;
use crate::item_relics::Relic;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharacterClass {
    #[default]
    Duelist,
    Spellweaver,
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyKind {
    Goblin,
    Orc,
    Dragon,
    DarkKnight,
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
pub struct Mana {
    pub current: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpellElement {
    #[default]
    Neutral,
    Fire,
    Ice,
    Wind,
    Stone,
}

#[derive(Debug, Clone, Default)]
pub struct CastableSpell {
    pub damage: i32,
    pub block: i32,
    pub element: SpellElement,
}

#[derive(Debug, Clone)]
pub struct EssenceInfo {
    pub element: SpellElement,
    pub damage: i32,
    pub block: i32,
}

#[derive(Component, Default)]
pub struct ActiveSpell {
    pub bonus_damage: i32,
    pub bonus_block: i32,
    pub element: SpellElement,
    pub essences: Vec<EssenceInfo>,
    // Store elements cast sequentially for combos
    pub essence_history: Vec<SpellElement>,
}

#[derive(Component, Default)]
pub struct Block {
    pub value: i32,
}

#[derive(Component, Default)]
pub struct PlayerCombo {
    pub current: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflexSuccess {
    Miss,
    Good,
    Perfect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflexVisualType {
    ShrinkingRing,
    LinearSlider,
}

#[derive(Component)]
pub struct ReflexState {
    pub start_time: f32, // Unused conceptually if using timer, but good to have
    pub timer: Timer,
    pub perfect_window_start: f32, // ratio 0.0 to 1.0 of the timer
    pub perfect_window_end: f32,
    pub result: Option<ReflexSuccess>,
    pub is_defensive: bool,
    pub source_entity: Option<Entity>, // For the attacking player/enemy
    pub target_entity: Option<Entity>, // For the enemy being attacked
    pub base_damage: i32,              // Keep track of damage for resolution
    pub visual_type: ReflexVisualType,
}

#[derive(Component)]
pub struct ReflexUI;

#[derive(Component)]
pub struct ReflexMarker;

#[derive(Component, Default)]
pub struct StatusStore {
    pub poison: i32,
    pub weak: i32,
    pub strength: i32,
    pub stun: i32,
    pub burning: i32,
    pub frozen: i32,
}

#[derive(Component)]
pub struct Gold {
    pub amount: i32,
}

#[derive(Component, Default)]
pub struct RelicStore {
    pub relics: Vec<Relic>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[derive(Component, Debug, Clone)]
pub struct Card {
    pub name: String,
    pub damage: i32,
    pub block: i32,
    pub cost: i32,
    pub apply_poison: i32,
    pub apply_weak: i32,
    pub apply_stun: i32,
    pub upgraded: bool,
    pub rarity: Rarity,
    pub is_spell_modifier: bool,
    pub is_spell_source: bool,
    pub element: SpellElement,
    pub combo_points_granted: u32,
    pub finisher_combo_cost: u32,
}

#[derive(Component)]
pub struct PendingPlayerAttack {
    pub targets: Vec<Entity>,
    pub damage: i32,
    pub card: Card,
    pub spell_elements: Vec<SpellElement>,
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
pub struct MapUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

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
pub struct PlayerComboText;

#[derive(Component)]
pub struct PlayerRelicText;

#[derive(Component)]
pub struct HandContainer;

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct EnemyIntentText;

#[derive(Component)]
pub struct LeaveShopButton;

#[derive(Component)]
pub struct BuyCardButton {
    pub card: Card,
    pub cost: i32,
    pub index: usize,
}

#[derive(Component)]
pub struct BuyRelicButton {
    pub relic: Relic,
    pub cost: i32,
    pub index: usize,
}

#[derive(Component)]
pub struct HealButton;

#[derive(Component)]
pub struct UpgradeButton;

#[derive(Component)]
pub struct LeaveRestButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Battle,
    Shop,
    Rest,
    Boss,
    Elite,
    Event,
}

#[derive(Component)]
pub struct MapNodeButton {
    pub level: usize,
    pub index: usize,
    pub node_type: NodeType,
}

#[derive(Component)]
pub struct DeckText;

#[derive(Component)]
pub struct DiscardText;

#[derive(Component)]
pub struct ViewDeckUI;

#[derive(Component)]
pub struct ViewDeckButton;

#[derive(Component)]
pub struct ReturnFromDeckButton;

#[derive(Component)]
pub struct DiscardPileButton;

#[derive(Component)]
pub struct ViewDiscardUI;

#[derive(Component)]
pub struct ReturnFromDiscardButton;

#[derive(Component)]
pub struct Tooltip {
    pub text: String,
}

#[derive(Component)]
pub struct TooltipUi;

#[derive(Component)]
pub struct ShopGoldText;

#[derive(Component)]
pub struct RemoveCardServiceButton {
    pub cost: i32,
}

#[derive(Component)]
pub struct ShopRemoveUI;

#[derive(Component)]
pub struct CardToRemoveButton {
    pub index: usize,
}

#[derive(Component)]
pub struct CancelRemoveButton;

#[derive(Component)]
pub struct RewardUI;

#[derive(Component)]
pub struct RewardGoldButton;

#[derive(Component)]
pub struct RewardCardButton;

#[derive(Component)]
pub struct ProceedButton;

#[derive(Component)]
pub struct RewardSelectCardUI;

#[derive(Component)]
pub struct CardChoiceButton {
    pub card_index: usize,
}

#[derive(Component)]
pub struct SkipCardButton;

#[derive(Component, Default)]
pub struct PotionStore {
    pub potions: Vec<Potion>,
}

#[derive(Component)]
pub struct PotionButton {
    pub index: usize,
}

#[derive(Component)]
pub struct PotionContainer;

#[derive(Component)]
pub struct BuyPotionButton {
    pub potion: Potion,
    pub cost: i32,
    pub index: usize,
}

#[derive(Component)]
pub struct EventUI;

#[derive(Component)]
pub struct EventOptionButton {
    pub effect_id: usize,
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct RelicIcon {
    pub relic: Relic,
}

#[derive(Component)]
pub struct SceneBackground;

#[derive(Component, Debug, Clone)]
pub struct NextEnemyMove {
    pub name: String,
    pub damage: i32,
    pub block: i32,
    pub poison: i32,
    pub weak: i32,
    pub steal_gold: i32,
    pub is_charging: bool,
}

#[derive(Component)]
pub struct DamageFlashUi;

#[derive(Component)]
pub struct BlockFlashUi;

#[derive(Component)]
pub struct EnemyQueue(pub Vec<Entity>); // Queue tracking during EnemyTurn

#[derive(Component)]
pub struct AttackingEnemy;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct CharacterSelectUI;

#[derive(Component)]
pub struct SelectDuelistButton;

#[derive(Component)]
pub struct SelectSpellweaverButton;

#[derive(Component)]
pub struct PlayerSpellContainer;

#[derive(Component)]
pub struct BonusSelectUI;

#[derive(Component)]
pub struct BonusOptionButton {
    pub bonus_type: BonusType,
}

#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Gold,
    Relic,
    Upgrade,
}

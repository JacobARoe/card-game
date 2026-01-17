use bevy::prelude::*;
use crate::components::Card;
use crate::components::NodeType;
use crate::item_relics::Relic;
use crate::item_potions::Potion;

#[derive(Resource, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Resource, Default)]
pub struct DiscardPile {
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub struct MapNodeData {
    pub node_type: NodeType,
    pub next_indices: Vec<usize>,
    pub visible: bool,
}

#[derive(Resource, Default)]
pub struct GameMap {
    pub levels: Vec<Vec<MapNodeData>>,
    pub current_node: Option<(usize, usize)>,
    pub visited_path: Vec<(usize, usize)>,
}

#[derive(Resource, Default)]
pub struct RewardStore {
    pub generated: bool,
    pub gold_reward: Option<i32>,
    pub card_choices: Option<Vec<Card>>,
}

#[derive(Resource, Default)]
pub struct ShopStore {
    pub generated: bool,
    pub cards: Vec<Option<(Card, i32)>>,
    pub relics: Vec<Option<(Relic, i32)>>,
    pub potions: Vec<Option<(Potion, i32)>>,
}

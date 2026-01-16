use bevy::prelude::*;
use crate::components::Card;

#[derive(Resource, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Resource, Default)]
pub struct DiscardPile {
    pub cards: Vec<Card>,
}
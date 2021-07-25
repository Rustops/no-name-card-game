use amethyst::{
    ecs::{Component, DenseVecStorage},
};
use std::convert::TryInto;

use super::card::Card;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CardGroup {
    pub cards_num: u32,
    pub cards: Vec<Card>,
}

impl CardGroup {
    pub fn new() -> Self {
        CardGroup { 
            cards_num: 0,
            cards: Vec::new(),
        }
    }
    pub fn add(&mut self, card: Card) {
        self.cards_num += 1;
        self.cards.push(card);
    }
    pub fn remove(&mut self, index: u32) {
        self.cards_num -= 1;
        self.cards.remove(index.try_into().unwrap());
    }
}
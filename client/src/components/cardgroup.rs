use amethyst::ecs::{Component, DenseVecStorage};

use super::card::Card;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CardGroup {
    pub cards_num: u32,
    pub cards: Vec<Card>,
}

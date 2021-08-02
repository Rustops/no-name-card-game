use amethyst::ecs::{Component, DenseVecStorage};

use super::Card;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Equipment {
    // Different weapons have different effects
    pub weapon: Card,
    // Different armors have different effects
    pub armor: Card,
    // Increases your calculated distance from the enemy
    pub offend_horse: Card,
    // Reduces enemy's calculated distance from you
    pub defend_horse: Card,
}

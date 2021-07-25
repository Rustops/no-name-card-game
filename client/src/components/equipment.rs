use amethyst::{
    ecs::{Component, DenseVecStorage},
};

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

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            weapon: Card::new(),
            armor: Card::new(),
            offend_horse: Card::new(),
            defend_horse: Card::new(),
        }
    }
    pub fn set_weapon(&mut self, weapon: Card) {
        self.weapon = weapon;
    }
    pub fn set_armor(&mut self, armor: Card) {
        self.armor = armor;
    }
    pub fn set_offend_horse(&mut self, offend_horse: Card) {
        self.offend_horse = offend_horse;
    }
    pub fn set_defend_horse(&mut self, defend_horse: Card) {
        self.defend_horse = defend_horse;
    }
    pub fn reset(&mut self) {
        self.weapon = Card::new();
        self.armor = Card::new();
        self.offend_horse = Card::new();
        self.defend_horse = Card::new();
    }
}
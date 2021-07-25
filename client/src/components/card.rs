use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Component, PartialEq, Eq, PartialOrd, Ord)]
#[storage(DenseVecStorage)]
pub struct Card {
    pub name: String,
    pub damage: u32,
    //......
}

impl Card {
    pub fn new() -> Self {
        Card {
            name: String::from("default_card"),
            damage: 0,
        }
    }
}
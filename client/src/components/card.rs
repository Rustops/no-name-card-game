use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component, PartialEq, Eq, PartialOrd, Ord)]
#[storage(DenseVecStorage)]
pub struct Card {
    pub name: String,
    pub damage: u32,
    //......
}

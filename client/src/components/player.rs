use amethyst::ecs::{Component, DenseVecStorage};

use crate::resources::CharacterType;

#[derive(Debug, Clone)]
pub enum PlayerState {
    // The game has not started, players are chatting
    Chatting,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Chatting
    }
}

#[derive(Debug, Component, Clone, Default)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub name: String,
    pub state: PlayerState,
    pub is_playing: bool,
    pub role: CharacterType,
}

impl Player {
    #[allow(dead_code)]
    pub fn new(name: String, state: PlayerState, is_playing: bool, role: CharacterType) -> Self {
        Player {
            name,
            state,
            is_playing,
            role,
        }
    }
}

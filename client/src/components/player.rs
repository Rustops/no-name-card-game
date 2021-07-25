use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Debug)]
pub enum PlayerState {
    // The game has not startedm, players are chatting
    Chatting,
    // Lost all health
    Dying,
    // Executing a round
    Playing,
    // Waiting for the round
    Waiting,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Chatting
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub is_playing: bool,
    // pub role: ,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Chatting,
            is_playing: false,
        }
    }
}
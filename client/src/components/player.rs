use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub enum PlayerState {
    // The game has not startedm, players are chatting
    Chatting,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Chatting
    }
}

#[derive(Debug)]

pub enum PlayerRole {
    Flandre,
}

impl Default for PlayerRole {
    fn default() -> Self {
        Self::Flandre
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub is_playing: bool,
    pub role: PlayerRole,
}

impl Player {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Player {
            state: PlayerState::Chatting,
            is_playing: false,
            role: PlayerRole::Flandre,
        }
    }
}

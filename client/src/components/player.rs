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
    pub name: String,
    pub state: PlayerState,
    pub is_playing: bool,
    pub role: PlayerRole,
}

impl Player {
    #[allow(dead_code)]
    pub fn new(name: String, state: PlayerState, is_playing: bool, role: PlayerRole) -> Self {
        Player {
            name,
            state,
            is_playing,
            role,
        }
    }
}

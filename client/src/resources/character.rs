#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Each character type is unique
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CharacterType {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Alice,
    Cirno,
    Flandre,
    Kanako,
    Kokoro,
}

impl Default for CharacterType {
    fn default() -> Self {
        CharacterType::NotFound
    }
}

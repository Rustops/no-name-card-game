use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Avatar {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Default,
    Alice,
    Cirno,
    Flandre,
    Kanako,
    Kokoro,
}

impl Default for Avatar {
    fn default() -> Self {
        Avatar::Default
    }
}

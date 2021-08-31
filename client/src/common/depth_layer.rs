#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum DepthLayer {
    // todo! implement layer for different type
    Background,
    DebugLines,
    // Leave room to expand background into multiple parallax layers here
    Blocks,
    Selection,
    Cursor,
    FloatingBlocks,
    Enemies,
    Player,
    Particles,
    /// Any UI elements that exist in world space. If we'd want a health bar above an enemy's head,
    /// this is the z-layer we'd use.
    /// Currently, it is used for some debugging elements, such as the player frames.
    /// In the editor, it is used for the cursor and selection.
    UiElements,
    Camera,
}

impl Default for DepthLayer {
    fn default() -> Self {
        DepthLayer::Blocks
    }
}

impl DepthLayer {
    #[must_use]
    pub fn z(self) -> f32 {
        match self {
            DepthLayer::Background => 0.,
            DepthLayer::DebugLines => 1.,
            DepthLayer::Blocks => 100.,
            DepthLayer::Selection => 101.,
            DepthLayer::Cursor => 102.,
            DepthLayer::FloatingBlocks => 110.,
            DepthLayer::Enemies => 120.,
            DepthLayer::Player => 130.,
            DepthLayer::Particles => 140.,
            DepthLayer::UiElements => 200.,
            DepthLayer::Camera => 300.,
        }
    }
}

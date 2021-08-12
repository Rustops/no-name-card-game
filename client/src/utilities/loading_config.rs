use crate::resources::{SoundType, UiType};
use serde::{Deserialize, Serialize};

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct LoadingConfig {
    pub uis: Vec<(UiType, String)>,
    // todo: implement
    // pub characters: ,
    pub sound_effects: Vec<(SoundType, String)>,
    pub music_tracks: Vec<String>,
}

impl Default for LoadingConfig {
    fn default() -> Self {
        LoadingConfig {
            uis: vec![],
            sound_effects: vec![],
            music_tracks: vec![],
        }
    }
}

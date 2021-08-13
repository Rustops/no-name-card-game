use super::{audio::SoundType, CharacterType};
use amethyst::{assets::Handle, audio::SourceHandle, renderer::SpriteSheet};
use log::error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    sounds: HashMap<SoundType, Vec<SourceHandle>>,
    characters: HashMap<CharacterType, Handle<SpriteSheet>>,
}

impl Assets {
    pub fn put_character(mut self, asset_type: CharacterType, asset: Handle<SpriteSheet>) -> Self {
        self.characters.insert(asset_type, asset);
        self
    }
    // todo: applying this
    #[allow(dead_code)]
    pub fn get_character(&self, asset_type: CharacterType) -> Handle<SpriteSheet> {
        (*self
            .characters
            .get(&asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.characters.get(&CharacterType::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_sound(mut self, sound_type: SoundType, asset: SourceHandle) -> Self {
        self.sounds
            .entry(sound_type)
            .or_insert_with(Vec::new)
            .push(asset);
        self
    }

    pub fn get_sound(&self, asset_type: SoundType) -> Option<SourceHandle> {
        self
            .sounds
            .get(&asset_type)
            .or_else(|| {
                error!("There are no sounds of type {:?}. Add them to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*sounds_of_that_type.get(random_index).expect("Should not panic.")).clone()
            })
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AssetType {
    /// A static, non-animated image.
    /// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
    Character(CharacterType, usize),
    // Animated(AnimType),
}

/// Matches a still or animated asset to its dimensions in pixels. Required to calculate the
/// correct scale factor for the entity to make it fit within its in-world bounds.
#[allow(clippy::match_single_binding)]
#[allow(dead_code)]
pub fn get_asset_dimensions(asset: &AssetType) {
    match asset {
        AssetType::Character(character_type, _) => match character_type {
            CharacterType::Alice => todo!(),
            CharacterType::Cirno => todo!(),
            _ => todo!(),
        },
    }
}

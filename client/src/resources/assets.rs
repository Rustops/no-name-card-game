use crate::{common::Pos, resources::Avatar};

use super::{audio::SoundType, CharacterType, SpriteType};
use amethyst::{
    assets::Handle,
    audio::SourceHandle,
    renderer::{SpriteSheet, Texture},
};
use log::error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    sounds: HashMap<SoundType, Vec<SourceHandle>>,
    characters: HashMap<CharacterType, Handle<Texture>>,
    stills: HashMap<SpriteType, Handle<SpriteSheet>>,
}

impl Assets {
    pub fn put_character(mut self, asset_type: CharacterType, asset: Handle<Texture>) -> Self {
        self.characters.insert(asset_type, asset);
        self
    }
    #[allow(dead_code)]
    pub fn get_character(&self, asset_type: CharacterType) -> Handle<Texture> {
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

    pub fn put_still(mut self, asset_type: SpriteType, asset: Handle<SpriteSheet>) -> Self {
        self.stills.insert(asset_type, asset);
        self
    }

    pub fn get_still(&self, asset_type: SpriteType) -> Handle<SpriteSheet> {
        (*self
            .stills
            .get(&asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.stills.get(&SpriteType::NotFound)
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
    Still(SpriteType, usize),
    Character(CharacterType, usize),
    // Animated(AnimType),
}

/// Matches a still or animated asset to its dimensions in pixels. Required to calculate the
/// correct scale factor for the entity to make it fit within its in-world bounds.
pub fn get_asset_dimensions(asset: &AssetType) -> Pos {
    match asset {
        AssetType::Character(character_type, _) => match character_type {
            CharacterType::Alice => Pos::new(50, 50),
            CharacterType::Cirno => todo!(),
            _ => todo!(),
        },
        AssetType::Still(sprite_type, _) => match sprite_type {
            SpriteType::Avatar(Avatar::Default) => Pos::new(50, 50),
            SpriteType::Avatar(_) => Pos::new(50, 50),
            _ => Pos::new(128, 128),
        },
    }
}

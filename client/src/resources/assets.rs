use super::audio::SoundType;
use amethyst::audio::SourceHandle;
use log::error;
use rand::Rng;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    sounds: HashMap<SoundType, Vec<SourceHandle>>,
}

impl Assets {
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

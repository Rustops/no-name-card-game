#![allow(dead_code)]
use std::collections::HashMap;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use log::error;
use serde::{Deserialize, Serialize};

use amethyst::{assets::Handle, ecs::prelude::Entity, prelude::*};

/// This resource stores handles to UI prefabs that were loaded in the `LoadingState`.
#[derive(Default, Debug)]
pub struct CharacterHandle {
    map: HashMap<CharacterType, Handle<UiPrefab>>,
}

impl CharacterHandle {
    #[must_use]
    pub fn put_handle(mut self, key: CharacterType, handle: Handle<UiPrefab>) -> Self {
        self.map.insert(key, handle);
        self
    }

    fn clone_handle(&self, key: CharacterType) -> Option<Handle<UiPrefab>> {
        self.map
            .get(&key)
            .or_else(|| {
                error!("Tried using UI element {:?} but that element was not loaded! To use this element, add it to the LoadingConfig.", key);
                None
            })
            .map(|handle| (*handle).clone())
    }

    /// todo! implement
    /// Convenience method that grabs the correct `UiHandle` and uses it to create an entity.
    /// This is the recommended way to create a GUI.
    pub fn load_character(key: CharacterType, world: &mut World) -> Option<Entity> {
        let handle = world.read_resource::<CharacterHandle>().clone_handle(key);
        handle.map(|handle| world.create_entity().with(handle).build())
    }
}

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
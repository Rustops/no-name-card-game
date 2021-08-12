#![allow(dead_code)]
use std::collections::HashMap;

use amethyst::prelude::WorldExt;

use amethyst::ui::UiPrefab;
use log::error;
use serde::{Deserialize, Serialize};

use amethyst::{assets::Handle, ecs::prelude::Entity, prelude::*};

/// This resource stores handles to UI prefabs that were loaded in the `LoadingState`.
#[derive(Default, Debug)]
pub struct UiHandles {
    map: HashMap<UiType, Handle<UiPrefab>>,
}

impl UiHandles {
    #[must_use]
    pub fn put_handle(mut self, key: UiType, handle: Handle<UiPrefab>) -> Self {
        self.map.insert(key, handle);
        self
    }

    fn clone_handle(&self, key: UiType) -> Option<Handle<UiPrefab>> {
        self.map
            .get(&key)
            .or_else(|| {
                error!("Tried using UI element {:?} but that element was not loaded! To use this element, add it to the LoadingConfig.", key);
                None
            })
            .map(|handle| (*handle).clone())
    }

    /// Convenience method that grabs the correct `UiHandle` and uses it to create an entity.
    /// This is the recommended way to create a GUI.
    pub fn add_ui(key: UiType, world: &mut World) -> Option<Entity> {
        let handle = world.read_resource::<UiHandles>().clone_handle(key);
        handle.map(|handle| world.create_entity().with(handle).build())
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum UiType {
    // As the name implies
    Credits,
    /// Default player
    DefaultPlayer,
    /// Main game interface
    Game,
    /// Te preparation interface before the game
    Lobby,
    /// The main menu.
    MainMenu,
    /// The pause menu in game
    PauseMenu,
    /// For the character selection screen before the game starts
    CharacterSelection,
    /// The welcome screen
    Welcome,
}

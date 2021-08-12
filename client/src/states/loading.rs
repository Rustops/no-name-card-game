use crate::resources::{load_audio_settings, Assets, AudioSettings, Music, UiHandles, UserCache};
use crate::utilities::files::get_user_cache_file;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use log::error;
use log::info;

use amethyst::{
    assets::{Completion, Loader, ProgressCounter},
    ecs::prelude::Entity,
    prelude::*,
    StateData, Trans,
};

use crate::states::welcome::WelcomeScreen;
use crate::utilities::{files::get_config_dir, loading_config::LoadingConfig};
use amethyst::audio::{AudioSink, OggFormat};

/// This state is briefly active when the game is first started up. It loads all assets used in the
/// entire game and then switches to the main menu state.
///
/// If you want to add a new asset that should be loaded, please go to `LoadingConfig` and add it
/// there.
#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        load_configs(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        // Create a LoadingConfig to tell us what assets to actually load.
        let mut loading_config = load_loading_config();

        // Load all UI handles.
        let ui_handles =
            loading_config
                .uis
                .drain(..)
                .fold(UiHandles::default(), |handles, (ui_type, path)| {
                    info!("{:?}, {:?}", ui_type, path);
                    handles.put_handle(
                        ui_type,
                        data.world
                            .exec(|loader: UiLoader<'_>| loader.load(path, &mut self.progress)),
                    )
                });
        data.world.insert(ui_handles);

        // Take the Assets instance we previously filled with stills and animations and
        // add sound effects.
        let assets = loading_config.sound_effects.drain(..).fold(
            Assets::default(),
            |assets, (sound_type, file_path)| {
                let loader = data.world.read_resource::<Loader>();
                assets.put_sound(
                    sound_type,
                    loader.load(
                        file_path,
                        OggFormat,
                        &mut self.progress,
                        &data.world.read_resource(),
                    ),
                )
            },
        );
        data.world.insert(assets);

        let music_resource =
            if let Some(volume) = data.world.read_resource::<AudioSettings>().music_volume {
                // Set music volume:
                data.world.write_resource::<AudioSink>().set_volume(volume);
                // Load music handles.
                let music_handles = loading_config
                    .music_tracks
                    .drain(..)
                    .map(|music_file_path| {
                        let loader = data.world.read_resource::<Loader>();
                        loader.load(
                            music_file_path,
                            OggFormat,
                            &mut self.progress,
                            &data.world.read_resource(),
                        )
                    })
                    .collect();
                Music::new(music_handles)
            } else {
                // Music volume is None, don't load the music tracks at all.
                Music::new(vec![])
            };
        data.world.insert(music_resource);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Failed => {
                error!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                info!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(WelcomeScreen::default()))
            }
            Completion::Loading => Trans::None,
        }
    }
}

/// Load the `LoadingConfig` from file. The `LoadingConfig` contains information on what assets must be
/// loaded by this `LoadingState`.
fn load_loading_config() -> LoadingConfig {
    LoadingConfig::load(&get_config_dir().join("loading.ron")).unwrap_or_else(|error| {
        error!(
            "Failed to load loading config! Falling back to default. Error: {:?}",
            error
        );
        LoadingConfig::default()
    })
}

/// Load various configuration resources from their respective files and insert them into the World
/// as resources.
fn load_configs(world: &mut World) {
    world.insert(load_audio_settings());

    world.insert(if get_user_cache_file().is_file() {
        UserCache::load(get_user_cache_file()).unwrap_or_else(|error| {
            error!(
                "Failed to load user cache! Falling back to default. Error: {:?}",
                error
            );
            UserCache::default()
        })
    } else {
        UserCache::default()
    });
}

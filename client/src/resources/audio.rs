use amethyst::{
    assets::AssetStorage,
    assets::Loader,
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const SOUND_BOOP: &str = "audio/boop.ogg";
const SOUND_CONFIRM: &str = "audio/confirm.ogg";

const BGM_LOBBY: &str = "audio/bgm_lobby.ogg";
const BGM_GAME: &str = "audio/bgm_game.ogg";

const SOUND_TRACKS: &[&str] = &[SOUND_BOOP, SOUND_CONFIRM];
const MUSIC_TRACKS: &[&str] = &[BGM_LOBBY, BGM_GAME];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub struct Sounds {
    pub boop_sound: SourceHandle,
    pub confirm_sound: SourceHandle,
}

#[allow(dead_code)]
/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

#[allow(dead_code)]
pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = SOUND_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, world, file))
            .collect::<Vec<_>>();
        let sound = Sounds {
            boop_sound: sound[0].clone(),
            confirm_sound: sound[1].clone(),
        };

        (sound, music)
    };

    // Add sound effects and music to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}

#[allow(dead_code)]
pub fn play_boop_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.boop_sound) {
            output.play_once(sound, 1.0);
        }
    }
}

#[allow(dead_code)]
pub fn play_confirm_sound(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.confirm_sound) {
            output.play_once(sound, 1.0);
        }
    }
}

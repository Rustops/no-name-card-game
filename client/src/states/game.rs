use amethyst::{
    core::Time,
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
};

use crate::{
    events::state_event::ExtendedStateEvent,
    resources::{UiHandles, UiType},
};

use super::pause::PauseMenuState;
/// Main 'Game' state. Actually, it is mostly similar to the ui/main.rs content-wise.
/// The main differences include the added 'paused' field in the state, which is toggled when
/// 'pausing'.

#[derive(Default)]
pub struct Game {
    // If the Game is paused or not
    paused: bool,
    // If the assets is loaded already
    // progress_counter: Option<ProgressCounter>,
    // The UI root entity. Deleting this should remove the complete UI
    ui_root: Option<Entity>,
    // A reference to the FPS display, which we want to interact with
    fps_display: Option<Entity>,
    player_display: Option<Entity>,
}

impl Game {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::Game, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for Game {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_ui(&mut data);
        // let StateData { mut world, .. } = data;

        // needed for registering audio output.
        // init_output(&mut world);

        // self.player_display = Some(
        //     world.exec(|mut creator: UiCreator<'_>| creator.create("ui/default_player.ron", ())),
        // );

        // load_player(world);
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData>) {
        self.paused = false;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Game Screen");
        }

        self.ui_root = None;
        self.fps_display = None;
        self.player_display = None;
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'a, 'b>>,
        event: ExtendedStateEvent,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        match &event {
            ExtendedStateEvent::Window(event) => {
                if is_close_requested(event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            ExtendedStateEvent::Ui(_ui_event) => {
                // log::info!(
                //     "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                //     ui_event
                // );
                Trans::None
            }
            ExtendedStateEvent::Input(_input) => {
                // log::info!("Input Event detected: {:?}.", input);
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        state_data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        let StateData { world, .. } = state_data;

        // this cannot happen in 'on_start', as the entity might not be fully
        // initialized/registered/created yet.
        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }

        if self.player_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("flandre") {
                    self.player_display = Some(entity);
                }
            });
        }

        // it is important that the 'paused' field is actually pausing your game.
        // Make sure to also pause your running systems.
        if !self.paused {
            let mut ui_text = world.write_storage::<UiText>();

            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 && !self.paused {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        Trans::None
    }
}

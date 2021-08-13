use amethyst::{
    core::Time,
    ecs::{Entity, Read, Write},
    input::{is_close_requested, is_key_down},
    network::simulation::TransportResource,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
};

use super::pause::PauseMenuState;
use crate::{
    resources::{UiHandles, UiType},
    states::select_character::SelectState,
    systems::chat::ChatroomBundle,
};

/// Main 'Game' state. Actually, it is mostly similar to the ui/main.rs content-wise.
/// The main differences include the added 'paused' field in the state, which is toggled when
/// 'pausing'.

#[derive(Default)]
pub struct Lobby {
    // If the Game is paused or not
    paused: bool,
    // The UI root entity. Deleting this should remove the complete UI
    ui_root: Option<Entity>,
    // A reference to the FPS display, which we want to interact with
    fps_display: Option<Entity>,
    // A button to start game
    start_game: Option<Entity>,
}

impl Lobby {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::Lobby, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }

    fn init_connection(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        data.world.exec(
            |(mut net, chatroom_info): (Write<'_, TransportResource>, Read<'_, ChatroomBundle>)| {
                let conn_msg = format!("Connect-{}", chatroom_info.client_info);
                net.send(chatroom_info.server_info.get_addr(), conn_msg.as_bytes());
            },
        );
    }
}

impl SimpleState for Lobby {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // let StateData { mut world, .. } = data;
        self.init_ui(&mut data);
        self.init_connection(&mut data);
        // needed for registering audio output.
        // init_output(&mut world);

        // initialize_audio(world);
        // load_player(world)
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
    }

    fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing in lobby!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.start_game {
                    log::info!("[Trans::Switch] Switching to Select!");
                    return Trans::Push(Box::new(SelectState::default()));
                }
                Trans::None
            }
            StateEvent::Input(_input) => {
                // log::info!("Input Event detected: {:?}.", input);
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {
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

        if self.start_game.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("lobby_start") {
                    self.start_game = Some(entity);
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

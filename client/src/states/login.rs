use amethyst::{
    ecs::{Entity, Read, Write},
    input::{is_close_requested, is_key_down},
    network::simulation::TransportResource,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};
use log::info;

use crate::{
    resources::{UiHandles, UiType},
    states::lobby::Lobby,
    systems::chat::ChatroomBundle,
};

use super::welcome::WelcomeScreen;

#[derive(Default, Debug)]
pub struct Login {
    ui_root: Option<Entity>,
    input_box: Option<Entity>,
    enter_button: Option<Entity>,
}

impl Login {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::MainMenu, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }

    /// The player should connect to the server when he enters the lobby, and
    /// here the player should send his information to the server to facilitate
    /// the server loading the players in the lobby.
    fn init_connection(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        data.world.exec(
            |(mut net, chatroom_info): (Write<'_, TransportResource>, Read<'_, ChatroomBundle>)| {
                log::info!("chatroom_info: server {:?}", chatroom_info.server_info);
                log::info!("chatroom_info: client {:?}", chatroom_info.client_info);
                let conn_msg = format!("{}-Connect", chatroom_info.client_info.name);
                net.send(chatroom_info.server_info.get_addr(), conn_msg.as_bytes());
            },
        );
    }

    fn load_button(&mut self, world: &mut World) {
        world.exec(|ui_finder: UiFinder<'_>| {
            self.enter_button = ui_finder.find("login_button_enter");
        });
    }

    fn load_input_box(&mut self, world: &mut World) {
        world.exec(|ui_finder: UiFinder<'_>| {
            self.input_box = ui_finder.find("login_label_input");
        });
    }
}

impl SimpleState for Login {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        info!("Users are logging in......");
        self.init_ui(&mut data);
        self.init_connection(&mut data);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.enter_button.is_none() {
            self.load_button(world);
        }

        if self.input_box.is_none() {
            self.load_input_box(world);
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _state_data: StateData<'_, GameData>,
        event: StateEvent,
    ) -> SimpleTrans {
        // let StateData { world, .. } = state_data;
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::Switch(Box::new(WelcomeScreen::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.enter_button {
                    log::info!("User login successful!");
                    log::info!("Connection to the server is about to be established！");
                    return Trans::Switch(Box::new(Lobby::default()));
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Login");
        }

        self.ui_root = None;
        self.enter_button = None;
        self.input_box = None;
    }
}

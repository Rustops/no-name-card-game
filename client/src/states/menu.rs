use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    network::simulation::TransportResource,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};
use log::info;

use super::{credits::CreditsScreen, welcome::WelcomeScreen};
use crate::{
    resources::{UiHandles, UiType},
    states::lobby::Lobby,
    systems::message::ServerInfoResource,
};
use shared::{
    clientinfo::ClientInfo,
    utilities::msg::{Message, MessageLayer, TransMessage},
};

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    menu_buttons: MenuButtons,
}

impl MainMenu {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::MainMenu, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
        // look up our buttons
        self.menu_buttons.load_buttons(data.world);
    }

    /// The player should connect to the server when he enters the lobby, and
    /// here the player should send his information to the server to facilitate
    /// the server loading the players in the lobby.
    fn init_connection(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let client = world.fetch_mut::<ClientInfo>().clone();
        let server = world.fetch::<ServerInfoResource>();
        let mut net = world.fetch_mut::<TransportResource>();
        let msg = Message::new(
            client,
            shared::msg::MessageType::EnterLobby,
            "I want to connect to the server".to_owned(),
        );

        let trans_message = TransMessage::construct(MessageLayer::Connection, msg);

        // FIXME: unwrap()
        net.send(
            server.get_addr(),
            trans_message.serialize().unwrap().as_bytes(),
        );
    }
}

#[derive(Default, Debug)]
pub struct MenuButtons {
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl MenuButtons {
    fn is_none(&self) -> bool {
        self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_credits.is_none()
            || self.button_options.is_none()
    }

    fn load_buttons(&mut self, world: &mut World) {
        world.exec(|ui_finder: UiFinder<'_>| {
            self.button_start = ui_finder.find(BUTTON_START);
            self.button_load = ui_finder.find(BUTTON_LOAD);
            self.button_options = ui_finder.find(BUTTON_OPTIONS);
            self.button_credits = ui_finder.find(BUTTON_CREDITS);
        });
    }

    fn set_none(&mut self) {
        self.button_start = None;
        self.button_load = None;
        self.button_options = None;
        self.button_credits = None;
    }
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        info!("MainMenuState on_start");
        self.init_ui(&mut data);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.menu_buttons.is_none() {
            self.menu_buttons.load_buttons(world);
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        state_data: StateData<'_, GameData>,
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
                if Some(target) == self.menu_buttons.button_credits {
                    log::info!("[Trans::Switch] Switching to CreditsScreen!");
                    return Trans::Switch(Box::new(CreditsScreen::default()));
                }
                if Some(target) == self.menu_buttons.button_start {
                    log::info!("[Trans::Switch] Switching to Lobby!");
                    self.init_connection(state_data);
                    return Trans::Switch(Box::new(Lobby::default()));
                }
                if Some(target) == self.menu_buttons.button_load
                    || Some(target) == self.menu_buttons.button_options
                {
                    log::info!("This Buttons functionality is not yet implemented!");
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
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.menu_buttons.set_none()
    }
}

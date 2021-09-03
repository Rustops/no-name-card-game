use amethyst::{
    assets::Handle,
    core::{bundle::SystemBundle, SystemDesc, Time},
    ecs::{
        DispatcherBuilder, Entity, LazyUpdate, Read, System, SystemData, World, Write, WriteStorage,
    },
    network::simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource},
    prelude::WorldExt,
    renderer::SpriteSheet,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText},
    Result,
};
use log::{debug, error, info, warn};
use shared::msg::{MessageLayer, TransMessage};
use std::net::SocketAddr;

use crate::{components::Player, entities::player::load_player, resources::SoundType};

use super::play_sfx::SoundEvent;

const SERVER_ADDRESS: &str = "127.0.0.1:6666";
const CLIENT_NAME: &str = "test";

#[derive(Debug, Default)]
pub struct ChatroomBundle {
    pub server_info: ServerInfoResource,
    pub client_info: ClientInfoResource,
}

impl ChatroomBundle {
    pub fn new(server_info: ServerInfoResource, client_info: ClientInfoResource) -> Self {
        Self {
            server_info,
            client_info,
        }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for ChatroomBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        world.insert(ServerInfoResource::new(self.server_info.addr));
        world.insert(ClientInfoResource::new(self.client_info.name));
        builder.add(ChatroomSystemDesc.build(world), "chat_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ChatroomSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ChatroomSystem> for ChatroomSystemDesc {
    fn build(self, world: &mut World) -> ChatroomSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ChatroomSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let network_reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();

        let ui_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();

        let client = world.fetch::<ClientInfoResource>().name.clone();
        let server = world.fetch::<ServerInfoResource>().get_addr();
        ChatroomSystem::new(network_reader, ui_reader, client, server)
    }
}

/// A simple system that receives a ton of network events.
struct ChatroomSystem {
    network_reader: ReaderId<NetworkSimulationEvent>,
    ui_reader: ReaderId<UiEvent>,
    chat_output: Option<Entity>,
    local_name: String,
    server_addr: SocketAddr,
    players: Vec<String>,
}

impl ChatroomSystem {
    pub fn new(
        network_reader: ReaderId<NetworkSimulationEvent>,
        ui_reader: ReaderId<UiEvent>,
        local_name: String,
        server_addr: SocketAddr,
    ) -> Self {
        Self {
            network_reader,
            ui_reader,
            chat_output: None,
            local_name,
            server_addr,
            players: vec![],
        }
    }

    fn find_ui_elements(&mut self, finder: &UiFinder) {
        self.chat_output = finder.find("lobby_multiline");
    }
}

impl<'a> System<'a> for ChatroomSystem {
    type SystemData = (
        UiFinder<'a>,
        Read<'a, EventChannel<UiEvent>>,
        Read<'a, NetworkSimulationTime>,
        Read<'a, Time>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        WriteStorage<'a, UiText>,
        Write<'a, EventChannel<SoundEvent>>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            ui_finder,
            ui_event,
            _sim_time,
            time,
            mut net,
            event,
            mut ui_text,
            mut sound_channel,
            lazy,
        ): Self::SystemData,
    ) {
        ui_event
            .read(&mut self.ui_reader)
            .filter(|event| event.event_type == UiEventType::ValueCommit)
            .for_each(|event| {
                if let Some(input) = ui_text.get_mut(event.target) {
                    // play sound_effect
                    sound_channel.single_write(SoundEvent::new(SoundType::Confirm));
                    // let msg = format!("{}-Chat-{}", self.local_name, input.text.clone());
                    info!(
                        "[{}][{}] Sending message: {}",
                        time.absolute_time_seconds(),
                        self.local_name,
                        &input.text,
                    );

                    let trans_message = TransMessage::new(
                        MessageLayer::ChatMessage,
                        format!("{}", self.local_name),
                        input.text.clone(),
                    );

                    net.send(
                        self.server_addr,
                        trans_message.serialize().unwrap().as_bytes()
                    );
                    // Reset input text
                    input.text = String::from("");
                }
            });

        for event in event.read(&mut self.network_reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("Client Received from {}: {:?}", addr, payload);
                    if let Ok(resp) = serde_json::from_slice::<TransMessage>(&payload) {
                        info!("msg is {:?}", resp);
                        self.find_ui_elements(&ui_finder);
                        match resp {
                            TransMessage::Default(m) => {
                                info!("Received: [SendToServer]");
                                info!("Unimplemented: {:?}", m);
                            },
                            TransMessage::ResponseImOnline(m) => {
                                info!("Received: [SendToServer]");
                                info!("Unimplemented: {:?}", m);
                            },
                            TransMessage::ForwardChatMessage(m) => {
                                info!("Received: [ForwardChatMessage]");
                                info!("[Chat] Update chatbox output");
                                if let Some(chat_output) = self.chat_output {
                                    info!("[Chat] Getting the interaction ui entity right");
                                    if let Some(output) = ui_text.get_mut(chat_output) {
                                        let total_msg = output.text.clone();
                                        let new_total_msg =
                                            format!("{}[{}]:{} \n", total_msg, m.from, m.msg);
                                        info!("[Chat] Update chatbox content: {}", new_total_msg);
                                        output.text = new_total_msg;
                                    }
                                }
                            },
                            TransMessage::PlayerEnterLobby(m) => {
                                info!("Received: [PlayerEnterLobby]");
                                // TODO: create player entity
                                let num = self.players.len();
                                let name = String::from(m.from);
                                if self.players.contains(&name) {
                                    continue;
                                }
                                self.players.push(name.clone());
                                log::info!("[Chat] Prepare loading player");
                                lazy.exec_mut(move |world| {
                                    load_player(world, name, num);
                                });
                            },
                            TransMessage::Order(m) => {
                                info!("Received: [Order]");
                                info!("Unimplemented: {:?}", m);
                            },
                            _ => debug!("Message is not for me"),
                        }
                    } else {
                        warn!("Received messages that cannot be processed. {:?}", 
                            String::from_utf8(payload.clone().to_vec()).unwrap());
                    };
                }
                NetworkSimulationEvent::Connect(addr) => {
                    info!("New client connection: {}", addr);
                }
                NetworkSimulationEvent::Disconnect(addr) => info!("Server Disconnected: {}", addr),
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                NetworkSimulationEvent::SendError(e, msg) => {
                    error!("Send Error: {:?}, {:?}", e, msg);
                }
                _ => {}
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Player>();
        world.register::<Handle<SpriteSheet>>();
        <Self as System<'_>>::SystemData::setup(world);
    }
}

#[derive(Debug)]
pub struct ServerInfoResource {
    pub addr: String,
}

impl ServerInfoResource {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr.parse().unwrap()
    }
}

impl Default for ServerInfoResource {
    fn default() -> Self {
        Self {
            addr: SERVER_ADDRESS.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ClientInfoResource {
    pub name: String,
}

impl ClientInfoResource {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Default for ClientInfoResource {
    fn default() -> Self {
        Self {
            name: CLIENT_NAME.to_string(),
        }
    }
}

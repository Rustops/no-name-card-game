use amethyst::{
    core::{bundle::SystemBundle, SystemDesc, Time},
    ecs::{DispatcherBuilder, Entity, Read, System, SystemData, World, Write, WriteStorage},
    network::simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText},
    Result,
};
use log::{error, info};
use std::net::SocketAddr;

use crate::resources::SoundType;

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
        ): Self::SystemData,
    ) {
        ui_event
            .read(&mut self.ui_reader)
            .filter(|event| event.event_type == UiEventType::ValueCommit)
            .for_each(|event| {
                if let Some(input) = ui_text.get_mut(event.target) {
                    // play sound_effect
                    sound_channel.single_write(SoundEvent::new(SoundType::Confirm));
                    let msg = format!("{}-Chat-{}", self.local_name, input.text.clone());
                    info!(
                        "[{}][{}] Sending message: {}",
                        time.absolute_time_seconds(),
                        self.local_name,
                        &msg
                    );
                    net.send(self.server_addr, msg.as_bytes());
                    input.text = String::from("");
                }
            });

        for event in event.read(&mut self.network_reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    // Highly centralized
                    if addr != &self.server_addr {
                        continue;
                    }
                    info!("Recv msg: {:?} from Server {}", payload, addr);
                    self.find_ui_elements(&ui_finder);

                    let p = payload.clone().to_vec();
                    let s = String::from_utf8(p).unwrap();
                    let ss: Vec<&str> = s.split("-").collect();

                    if ss[1] == "Chat" {
                        log::info!("[Chat] Update chatbox output");
                        if let Some(chat_output) = self.chat_output {
                            log::info!("[Chat] Getting the interaction ui entity right");
                            if let Some(output) = ui_text.get_mut(chat_output) {
                                let total_msg = output.text.clone();
                                let new_total_msg =
                                    format!("{}[{}]:{} \n", total_msg, ss[0], ss[2]);
                                log::info!("[Chat] Update chatbox content: {}", new_total_msg);
                                output.text = new_total_msg;
                            }
                        }
                    }
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

use amethyst::{
    assets::Handle,
    core::{bundle::SystemBundle, SystemDesc, Time},
    ecs::{
        DispatcherBuilder, Entity, LazyUpdate, Read, System, SystemData, World, Write, WriteStorage,
    },
    network::simulation::{
        tcp::{
            TcpConnectionListenerSystem, TcpNetworkRecvSystem, TcpNetworkResource,
            TcpNetworkSendSystem, TcpStreamManagementSystem,
        },
        udp::{UdpNetworkRecvSystem, UdpNetworkSendSystem, UdpSocketResource},
        NetworkSimulationEvent, NetworkSimulationTime, NetworkSimulationTimeSystem,
        TransportResource,
    },
    prelude::WorldExt,
    renderer::SpriteSheet,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText},
    Result,
};
use log::{error, info, warn};
use shared::{
    clientinfo::ClientInfo,
    msg::{MessageLayer, MessageType, TransMessage},
};
use std::net::{SocketAddr, TcpListener, UdpSocket};

use crate::{
    components::Player, entities::player::load_player, events::connection_event::ConnectionEvent,
    resources::SoundType,
};

use super::play_sfx::SoundEvent;

const SERVER_ADDRESS: &str = "127.0.0.1:6666";

#[derive(Debug, Default)]
pub struct MessageBundle {
    pub server_info: ServerInfoResource,
    pub client_info: ClientInfo,
    pub socket: Option<UdpSocket>,
    pub listener: Option<TcpListener>,
}

impl MessageBundle {
    pub fn new(
        server_info: ServerInfoResource,
        client_info: ClientInfo,
        socket: UdpSocket,
        listener: TcpListener,
    ) -> Self {
        Self {
            server_info,
            client_info,
            socket: Some(socket),
            listener: Some(listener),
        }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for MessageBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(NetworkSimulationTimeSystem, "simulation_time", &[]);

        builder.add(
            TcpConnectionListenerSystem,
            "connection_listener",
            &["simulation_time"],
        );

        builder.add(
            TcpStreamManagementSystem,
            "stream_management",
            &["simulation_time"],
        );

        builder.add(
            TcpNetworkSendSystem,
            "tcp_send",
            &["stream_management", "connection_listener"],
        );

        builder.add(
            TcpNetworkRecvSystem,
            "tcp_recv",
            &["stream_management", "connection_listener"],
        );

        world.insert(TcpNetworkResource::new(self.listener, 2048));

        builder.add(
            UdpNetworkRecvSystem::with_buffer_capacity(2048),
            "udp_recv",
            &["simulation_time"],
        );
        builder.add(UdpNetworkSendSystem, "udp_send", &["simulation_time"]);

        world.insert(UdpSocketResource::new(self.socket));

        world.insert(ServerInfoResource::new(self.server_info.addr));
        world.insert(ClientInfo::new(
            self.client_info.name,
            self.client_info.port,
        ));
        builder.add(MessageSystemDesc.build(world), "message_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct MessageSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, MessageSystem> for MessageSystemDesc {
    fn build(self, world: &mut World) -> MessageSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <MessageSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let network_reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();

        let ui_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();

        let client = world.fetch_mut::<ClientInfo>().clone();
        let server = world.fetch::<ServerInfoResource>().get_addr();
        MessageSystem::new(network_reader, ui_reader, client, server)
    }
}

/// A simple system that receives a ton of network events.
struct MessageSystem {
    network_reader: ReaderId<NetworkSimulationEvent>,
    ui_reader: ReaderId<UiEvent>,
    chat_output: Option<Entity>,
    client_info: ClientInfo,
    server_addr: SocketAddr,
    players: Vec<ClientInfo>,
}

impl MessageSystem {
    pub fn new(
        network_reader: ReaderId<NetworkSimulationEvent>,
        ui_reader: ReaderId<UiEvent>,
        client_info: ClientInfo,
        server_addr: SocketAddr,
    ) -> Self {
        Self {
            network_reader,
            ui_reader,
            chat_output: None,
            client_info,
            server_addr,
            players: vec![],
        }
    }

    fn find_ui_elements(&mut self, finder: &UiFinder) {
        self.chat_output = finder.find("lobby_multiline");
    }
}

impl<'a> System<'a> for MessageSystem {
    type SystemData = (
        UiFinder<'a>,
        Read<'a, EventChannel<UiEvent>>,
        Read<'a, NetworkSimulationTime>,
        Read<'a, Time>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        WriteStorage<'a, UiText>,
        Write<'a, EventChannel<SoundEvent>>,
        Write<'a, EventChannel<ConnectionEvent>>,
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
            mut connection_event,
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
                        "[{}]{:?} Sending message: {}",
                        time.absolute_time_seconds(),
                        &self.client_info,
                        &input.text,
                    );

                    let trans_message = TransMessage::new(
                        MessageLayer::Chat,
                        self.client_info.clone(),
                        shared::msg::MessageType::Chat,
                        input.text.clone(),
                    );

                    net.send(
                        self.server_addr,
                        trans_message.serialize().unwrap().as_bytes(),
                    );
                    // Reset input text
                    input.text = String::from("");
                }
            });

        for event in event.read(&mut self.network_reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("Client Received from {}: {:?}", addr, payload);
                    if let Ok(resp) = serde_json::from_slice::<TransMessage>(payload) {
                        info!("msg is {:?}", resp);
                        self.find_ui_elements(&ui_finder);
                        match resp {
                            TransMessage::Connection(m) => {
                                if m.msg_type == MessageType::EnterLobby {
                                    info!("Received: [PlayerEnterLobby]");
                                    let num = self.players.len();
                                    let client = m.from;
                                    if self.players.contains(&client) {
                                        continue;
                                    }
                                    self.players.push(client.clone());

                                    connection_event
                                        .single_write(ConnectionEvent::EnterLobby(client.clone()));
                                    log::info!("[Chat] Prepare loading player");
                                    lazy.exec_mut(move |world| {
                                        load_player(world, client.name, num);
                                    });
                                } else if m.msg_type == MessageType::Exit {
                                    info!("Received: [PlayerExitGame]");
                                    let client = m.from;
                                    if self.players.contains(&client) {
                                        let x = self
                                            .players
                                            .binary_search(&client)
                                            .unwrap_or_else(|x| x);
                                        self.players.remove(x - 1);
                                        // TODO: Remove player entity
                                        connection_event.single_write(ConnectionEvent::ExitGame(
                                            client.clone(),
                                        ));
                                    }
                                }
                            }
                            TransMessage::System(_) => todo!(),
                            TransMessage::Lobby(_) => todo!(),
                            TransMessage::Chat(m) => {
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
                            }
                            TransMessage::Game(_) => todo!(),
                        }
                    } else {
                        warn!(
                            "Received messages that cannot be processed. {:?}",
                            String::from_utf8(payload.clone().to_vec()).unwrap()
                        );
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

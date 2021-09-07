use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, UdpSocket},
};

use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::simulation::{
        tcp::{
            TcpConnectionListenerSystem, TcpNetworkRecvSystem, TcpNetworkResource,
            TcpNetworkSendSystem, TcpStreamManagementSystem,
        },
        udp::{UdpNetworkRecvSystem, UdpNetworkSendSystem, UdpSocketResource},
        NetworkSimulationEvent, NetworkSimulationTimeSystem, TransportResource,
    },
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{debug, error, info, warn};
use shared::{
    clientinfo::ClientInfo,
    msg::MessageType,
    utilities::msg::{MessageLayer, TransMessage},
};

#[derive(Debug)]
pub struct ServiceBundle {
    listener: Option<TcpListener>,
    socket: Option<UdpSocket>,
    recv_buffer_size_bytes: usize,
}

impl ServiceBundle {
    pub fn new(listener: TcpListener, socket: UdpSocket, recv_buffer_size_bytes: usize) -> Self {
        Self {
            listener: Some(listener),
            socket: Some(socket),
            recv_buffer_size_bytes,
        }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for ServiceBundle {
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

        world.insert(TcpNetworkResource::new(
            self.listener,
            self.recv_buffer_size_bytes,
        ));

        builder.add(
            UdpNetworkRecvSystem::with_buffer_capacity(self.recv_buffer_size_bytes),
            "udp_recv",
            &["simulation_time"],
        );
        builder.add(UdpNetworkSendSystem, "udp_send", &["simulation_time"]);

        world.insert(UdpSocketResource::new(self.socket));

        builder.add(
            ServiceSystemDesc::default().build(world),
            "service_system",
            &[],
        );

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ServiceSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ServiceSystem> for ServiceSystemDesc {
    fn build(self, world: &mut World) -> ServiceSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ServiceSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        ServiceSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct ServiceSystem {
    reader: ReaderId<NetworkSimulationEvent>,
    connection: Vec<SocketAddr>,
    players: HashMap<SocketAddr, ClientInfo>,
    online_num: u32,
}

impl ServiceSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self {
            reader,
            connection: Vec::new(),
            players: HashMap::default(),
            online_num: 0,
        }
    }

    pub fn add_players(&mut self, addr: SocketAddr, c: ClientInfo) {
        self.players.insert(addr, c);
    }

    // Tell the new_player how many players are online right now
    // Target is [new_player]
    pub fn sync_remaining_players(
        &self,
        socket: &mut UdpSocket,
        new_addr: &SocketAddr,
        from: &ClientInfo,
    ) {
        self.players.iter().for_each(|(_, c)| {
            let msg = TransMessage::new(
                MessageLayer::Connection,
                c.clone(),
                MessageType::EnterLobby,
                "enter lobby".to_string(),
            );

            debug!(
                "Tell the player:[{}] that [{}] is in the lobby.",
                from.name, c.name
            );
            send_message(socket, &new_addr, from, &msg);
        });
    }

    pub fn load_player_himself(
        &self,
        socket: &mut UdpSocket,
        new_addr: &SocketAddr,
        from: &ClientInfo,
    ) {
        let msg = TransMessage::new(
            MessageLayer::Connection,
            from.clone(),
            MessageType::EnterLobby,
            "enter lobby".to_string(),
        );

        info!("Load the player himself:[{}]", from.name);
        send_message(socket, &new_addr, from, &msg);
    }

    // Tell all other players that a new player has joined the game
    // Target is [other_players]
    pub fn broadcast_to_others(&self, socket: &mut UdpSocket, from: &ClientInfo) {
        self.players.iter().for_each(|(s, c)| {
            let msg = TransMessage::new(
                MessageLayer::Connection,
                from.clone(),
                MessageType::EnterLobby,
                "enter lobby".to_string(),
            );

            debug!(
                "Tell the player:[{}] that [{}] enter lobby.",
                c.name, from.name
            );
            send_message(socket, &s, c, &msg);
        });
    }
}

impl<'a> System<'a> for ServiceSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Write<'a, UdpSocketResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut _net, mut udp, channel): Self::SystemData) {
        let socket = udp.get_mut().expect("Get socker failed.");
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("[Message] {}: {:?}", addr, payload);
                    info!("players {:?}", self.players);
                    if let Ok(resp) = serde_json::from_slice::<TransMessage>(payload) {
                        match resp {
                            TransMessage::Connection(m) => {
                                info!("Received: [ConnectRequest]");

                                // Tell others the new_player came in
                                self.broadcast_to_others(socket, &m.from);
                                info!("Succeed: [Broadcast2Others]");
                                // Tell the new_player how many players are online right now
                                // NOTE: Include himself right now
                                self.sync_remaining_players(socket, addr, &m.from);
                                self.add_players(*addr, m.from.clone());
                                self.load_player_himself(socket, addr, &m.from);
                                info!("Succeed: [SyncOthers]");
                            }
                            TransMessage::System(_) => todo!(),
                            TransMessage::Lobby(_) => todo!(),
                            TransMessage::Chat(m) => {
                                info!("Received: [ChatMessage]");

                                let trans_message = TransMessage::new(
                                    MessageLayer::Chat,
                                    m.from,
                                    MessageType::Chat,
                                    m.msg,
                                );

                                self.players
                                    .iter()
                                    .for_each(|(s, c)| send_message(socket, s, c, &trans_message));

                                info!("Sent: [ForwardChatMessage] to all clients");
                                debug!("ForwardChatMessage is {:?}", trans_message);
                            }
                            TransMessage::Game(_) => todo!(),
                        }
                    }
                }
                NetworkSimulationEvent::Connect(addr) => {
                    info!("New client connection: {}", addr);
                    self.connection.push(*addr);
                    self.online_num = self.connection.len() as u32;
                    info!("Online player num: {:?}", self.online_num);
                }
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                    let index = self.connection.iter().position(|x| *x == *addr).unwrap();
                    self.connection.remove(index);
                    self.online_num = self.connection.len() as u32;
                    self.players.remove(addr);

                    if !self.players.is_empty() {
                        // tell other players that a player has quit the game
                        self.players.iter().for_each(|(s, c)| {
                            let mut s = *s;
                            s.set_port(c.port);
                            let msg = TransMessage::new(
                                MessageLayer::Connection,
                                c.clone(),
                                MessageType::Exit,
                                "exit game".to_string(),
                            );
                            match socket.connect(s) {
                                Ok(_) => info!("Connecting to the client successfully"),
                                Err(e) => info!("Connecting to the client failed: {}", e),
                            }
                            match socket.send(msg.serialize().unwrap().as_bytes()) {
                                Ok(_) => info!("Send to the client successfully"),
                                Err(e) => info!("Send to the client failed: {}", e),
                            }
                        });
                    }

                    info!("Online player num: {:?}", self.online_num);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}

pub fn send_message(
    socket: &mut UdpSocket,
    addr: &SocketAddr,
    from: &ClientInfo,
    trans_msg: &TransMessage,
) {
    let mut s = *addr;
    s.set_port(from.port);

    match socket.connect(s) {
        Ok(_) => debug!("Connecting to the client[{}] successfully", addr),
        Err(e) => warn!("Connecting to the client failed: {}, add is {:?}", e, addr),
    }
    match socket.send(trans_msg.serialize().unwrap().as_bytes()) {
        Ok(_) => debug!("Send to the client[{}] successfully", addr),
        Err(e) => warn!("Send to the client failed: {}, add is {:?}", e, addr),
    }
}

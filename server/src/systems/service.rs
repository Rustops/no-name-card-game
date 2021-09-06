use std::{
    collections::HashMap,
    convert::TryInto,
    net::{SocketAddr, TcpListener, UdpSocket},
};

use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::{
        simulation::{
            tcp::{
                TcpConnectionListenerSystem, TcpNetworkRecvSystem, TcpNetworkResource,
                TcpNetworkSendSystem, TcpStreamManagementSystem,
            },
            udp::{UdpNetworkRecvSystem, UdpNetworkSendSystem, UdpSocketResource},
            NetworkSimulationEvent, NetworkSimulationTimeSystem, TransportResource,
        },
        Bytes,
    },
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{debug, error, info};
use shared::utilities::msg::{MessageLayer, TransMessage};

const UDP_PORT: u16 = 2000;

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
    players: HashMap<SocketAddr, String>,
    online_num: u32,
    game_room: Vec<SocketAddr>,
}

impl ServiceSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self {
            reader,
            connection: Vec::new(),
            players: HashMap::default(),
            online_num: 0,
            game_room: Vec::new(),
        }
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
                    info!("{}: {:?}", addr, payload);
                    if let Ok(resp) = serde_json::from_slice::<TransMessage>(payload) {
                        match resp {
                            TransMessage::Default(m) => {
                                info!("Received: [Default]");
                                info!("Unimplemented: {:?}", m);
                            }
                            TransMessage::ResponseImOnline(_m) => {
                                info!("Received: [ResponseImOnline]");

                                // let trans_message = TransMessage::new(
                                //     MessageLayer::ResponseImOnline,
                                //     m.from,
                                //     m.msg,
                                // );

                                // let _v: Vec<_> = self
                                //     .connection
                                //     .iter()
                                //     .map(|x| {
                                //         net.send(*x, trans_message.serialize().unwrap().as_bytes())
                                //     })
                                //     .collect();
                            }
                            TransMessage::ConnectRequest(m) => {
                                info!("Received: [ConnectRequest]");
                                self.players.insert(*addr, m.from);
                                self.players.iter().for_each(|(_, name)| {
                                    let trans_message = TransMessage::new(
                                        MessageLayer::PlayerEnterLobby,
                                        name.to_string(),
                                        "enter lobby".to_string(),
                                    );

                                    self.connection.iter().for_each(|s| {
                                        let mut s = *s;
                                        s.set_port(UDP_PORT);
                                        match socket.connect(s) {
                                            Ok(_) => info!("Connecting to the client successfully"),
                                            Err(e) => {
                                                info!("Connecting to the client failed: {}", e)
                                            }
                                        }
                                        match socket
                                            .send(trans_message.serialize().unwrap().as_bytes())
                                        {
                                            Ok(_) => info!("Send to the client successfully"),
                                            Err(e) => info!("Send to the client failed: {}", e),
                                        }
                                    });
                                });
                            }
                            TransMessage::SendToServer(m) => {
                                info!("Received: [SendToServer]");
                                info!("Unimplemented: {:?}", m);
                            }
                            TransMessage::ChatMessage(m) => {
                                info!("Received: [ChatMessage]");

                                let trans_message = TransMessage::new(
                                    MessageLayer::ForwardChatMessage,
                                    m.from,
                                    m.msg,
                                );

                                let _v: Vec<_> = self
                                    .connection
                                    .iter()
                                    .map(|x| {
                                        let mut s = *x;
                                        s.set_port(UDP_PORT);
                                        match socket.connect(s) {
                                            Ok(_) => info!("Connecting to the client successfully"),
                                            Err(e) => {
                                                info!("Connecting to the client failed: {}", e)
                                            }
                                        }
                                        match socket
                                            .send(trans_message.serialize().unwrap().as_bytes())
                                        {
                                            Ok(_) => info!("Send to the client successfully"),
                                            Err(e) => info!("Send to the client failed: {}", e),
                                        }
                                    })
                                    .collect();
                                info!("Sent: [ForwardChatMessage] to all clients");
                                debug!("ForwardChatMessage is {:?}", trans_message);
                            }
                            _ => debug!("Message is not for me"),
                        }
                    }

                    // Check whether the player wants to play the game.
                    if payload.eq(&Bytes::from("Y")) | payload.eq(&Bytes::from("y")) {
                        self.game_room.push(*addr);
                        info!("{} Confirm to play, total: {:?}", addr, self.game_room);

                        // if self.game_room.len() == 2 {
                        //     info!("Players Enough");
                        //     let _v: Vec<_> = self
                        //         .game_room
                        //         .iter()
                        //         .map(|x| net.send(*x, START_GAME.as_bytes()))
                        //         .collect();

                        //     // Rest game_room
                        //     self.game_room.clear();
                        // }
                    }
                }
                NetworkSimulationEvent::Connect(addr) => {
                    info!("New client connection: {}", addr);
                    self.connection.push(*addr);
                    self.online_num = self.connection.len().try_into().unwrap();
                    info!("Online player num: {:?}", self.online_num);

                    if self.online_num >= 2 {
                        info!("Online players >= 2, send msg to players");
                        // let _v: Vec<_> = self
                        //     .connection
                        //     .iter()
                        //     .map(|x| net.send(*x, ENOUGH_PLAYER.as_bytes()))
                        //     .collect();
                    }
                }
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                    let index = self.connection.iter().position(|x| *x == *addr).unwrap();
                    self.connection.remove(index);
                    self.online_num = self.connection.len().try_into().unwrap();
                    self.players.remove(addr);
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

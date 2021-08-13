use std::{convert::TryInto, net::SocketAddr};

use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::{
        simulation::{NetworkSimulationEvent, TransportResource},
        Bytes,
    },
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{error, info};

// const HEARTBEAT_PU: &str = "HEARTBEAT:PU";
// const HEARTBEAT_TONG: &str = "HEARTBEAT:TONG";
const ENOUGH_PLAYER: &str = "There are already enough players in the room, do you want to start the game? please input Y/N.";
const START_GAME: &str = "\nStart Game!";

#[derive(Debug)]
pub struct ChatReceiveBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ChatReceiveBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ChatReceiveSystemDesc::default().build(world),
            "receiving_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ChatReceiveSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ChatReceiveSystem> for ChatReceiveSystemDesc {
    fn build(self, world: &mut World) -> ChatReceiveSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ChatReceiveSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        ChatReceiveSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct ChatReceiveSystem {
    reader: ReaderId<NetworkSimulationEvent>,
    connection: Vec<SocketAddr>,
    online_num: u32,
    game_room: Vec<SocketAddr>,
}

impl ChatReceiveSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self {
            reader,
            connection: Vec::new(),
            online_num: 0,
            game_room: Vec::new(),
        }
    }
}

impl<'a> System<'a> for ChatReceiveSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, channel): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    // In a typical client/server simulation, both the client and the server will
                    // be exchanging messages at a constant rate. Laminar makes use of this by
                    // packaging message acks with the next sent message. Therefore, in order for
                    // reliability to work properly, we'll send a generic "ok" response.
                    let _v: Vec<_> = self
                        .connection
                        .iter()
                        .map(|x| net.send(*x, payload))
                        .collect();

                    // Check whether the player wants to play the game.
                    if payload.eq(&Bytes::from("Y")) | payload.eq(&Bytes::from("y")) {
                        self.game_room.push(*addr);
                        info!("{} Confirm to play, total: {:?}", addr, self.game_room);

                        if self.game_room.len() == 2 {
                            info!("Players Enough");
                            let _v: Vec<_> = self
                                .game_room
                                .iter()
                                .map(|x| net.send(*x, START_GAME.as_bytes()))
                                .collect();

                            // Rest game_room
                            self.game_room.clear();
                        }
                    }
                }
                NetworkSimulationEvent::Connect(addr) => {
                    info!("New client connection: {}", addr);
                    self.connection.push(*addr);
                    self.online_num = self.connection.len().try_into().unwrap();
                    info!("Online player num: {:?}", self.online_num);

                    if self.online_num >= 2 {
                        info!("Online players >= 2, send msg to players");
                        let _v: Vec<_> = self
                            .connection
                            .iter()
                            .map(|x| net.send(*x, ENOUGH_PLAYER.as_bytes()))
                            .collect();
                    }
                }
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                    let index = self.connection.iter().position(|x| *x == *addr).unwrap();
                    self.connection.remove(index);
                    self.online_num = self.connection.len().try_into().unwrap();
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

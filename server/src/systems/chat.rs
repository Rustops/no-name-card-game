use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::simulation::{NetworkSimulationEvent, TransportResource},
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{error, info};

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
}

impl ChatReceiveSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
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

                    net.send(*addr, payload);
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}

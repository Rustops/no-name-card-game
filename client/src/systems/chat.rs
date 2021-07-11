use amethyst::{
    core::{bundle::SystemBundle, SystemDesc, Time},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::{
        simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource},
    },
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{error, info};

#[derive(Debug)]
pub struct ChatroomBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ChatroomBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ChatroomSystemDesc::default().build(world),
            "spam_system",
            &[],
        );
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
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();

        ChatroomSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct ChatroomSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ChatroomSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for ChatroomSystem {
    type SystemData = (
        Read<'a, NetworkSimulationTime>,
        Read<'a, Time>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );
    fn run(&mut self, (sim_time, time, mut net, event /*, tx*/): Self::SystemData) {
        let server_addr = "127.0.0.1:3457".parse().unwrap();
        for frame in sim_time.sim_frames_to_run() {
            info!("Sending message for sim frame {}.", frame);
            let payload = format!(
                "CL: sim_frame:{},abs_time:{}",
                frame,
                time.absolute_time_seconds()
            );
            net.send(server_addr, payload.as_bytes());
        }

        for event in event.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("Resolve msg: {:?} from {}", payload, addr)
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
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

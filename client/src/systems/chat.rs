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

const SERVER_ADDRESS: &str = "127.0.0.1:5566";

#[derive(Debug)]
pub struct ChatroomBundle {
    server_info: ServerInfoResource,
}

impl ChatroomBundle {
    pub fn new(server_info: ServerInfoResource) -> Self {
        Self { server_info }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for ChatroomBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ChatroomSystemDesc::default().build(world),
            "spam_system",
            &[],
        );
        world.insert(ServerInfoResource::new(self.server_info));
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

        ChatroomSystem::new(network_reader, ui_reader)
    }
}

/// A simple system that receives a ton of network events.
struct ChatroomSystem {
    network_reader: ReaderId<NetworkSimulationEvent>,
    ui_reader: ReaderId<UiEvent>,
    chat_input: Option<Entity>,
    chat_output: Option<Entity>,
}

impl ChatroomSystem {
    pub fn new(
        network_reader: ReaderId<NetworkSimulationEvent>,
        ui_reader: ReaderId<UiEvent>,
    ) -> Self {
        Self {
            network_reader,
            ui_reader,
            chat_input: None,
            chat_output: None,
        }
    }

    fn find_ui_elements(&mut self, finder: &UiFinder) {
        if self.chat_input.is_none() {
            self.chat_input = finder.find("editable");
        }
        if self.chat_output.is_none() {
            self.chat_output = finder.find("multiline");
        }
    }
}

impl<'a> System<'a> for ChatroomSystem {
    type SystemData = (
        // ReadStorage<'a, ServerInfoResource>,
        Read<'a, ServerInfoResource>,
        UiFinder<'a>,
        Read<'a, EventChannel<UiEvent>>,
        Read<'a, NetworkSimulationTime>,
        Read<'a, Time>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (server_info, ui_finder, ui_event, _sim_time, time, mut net, event, mut ui_text): Self::SystemData,
    ) {
        self.find_ui_elements(&ui_finder);

        ui_event
            .read(&mut self.ui_reader)
            .filter(|event| event.event_type == UiEventType::ValueCommit)
            .for_each(|event| {
                if self.chat_input == Some(event.target) {
                    if let Some(input) = ui_text.get_mut(event.target) {
                        let msg = input.text.clone();
                        info!(
                            "[{}] Sending message: {}",
                            time.absolute_time_seconds(),
                            &msg
                        );
                        net.send(server_info.get_addr(), msg.as_bytes());
                        input.text = String::from("");
                    }
                }
            });

        for event in event.read(&mut self.network_reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("Recv msg: {:?} from {}", payload, addr);
                    if let Some(chat_output) = self.chat_output {
                        if let Some(output) = ui_text.get_mut(chat_output) {
                            let raw_msg = payload.to_vec();
                            let msg = String::from_utf8_lossy(&raw_msg);

                            let total_msg = output.text.clone();
                            let new_total_msg = format!("{}{} \n", total_msg, msg);

                            output.text = new_total_msg;
                        }
                    }
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

#[derive(Debug)]
pub struct ServerInfoResource {
    pub addr: String,
}

impl ServerInfoResource {
    pub fn new(info: ServerInfoResource) -> Self {
        Self { addr: info.addr }
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

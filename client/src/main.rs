use std::time::Duration;
use amethyst::{
    audio::AudioBundle,
    core::{
        bundle::SystemBundle,
        TransformBundle,
        frame_limiter::FrameRateLimitStrategy, Time
    },
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    network::simulation::{
        tcp::TcpNetworkBundle, NetworkSimulationEvent, NetworkSimulationTime, TransportResource
    },
    shrev::{ EventChannel, ReaderId},
    Result,
};

use log::{error, info};

mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root_path = application_root_dir()?;
    let display_config_path = app_root_path.join("config").join("display.ron");
    let assets_dir = app_root_path.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            systems::events::UiEventHandlerSystemDesc,
            "ui_event_handler",
            &[],
        )
        .with_bundle(FpsCounterBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TcpNetworkBundle::new(None, 2048))?
        .with_bundle(SpamBundle)?;

    let mut game = Application::build(
        assets_dir,
        states::welcome::WelcomeScreen::default()
        )?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            114
        )
        .build(game_data)?;

    log::info!("Starting with WelcomeScreen!");
    game.run();
    Ok(())
}

#[derive(Debug)]
struct SpamBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpamBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(SpamSystemDesc::default().build(world), "spam_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct SpamSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, SpamSystem> for SpamSystemDesc {
    fn build(self, world: &mut World) -> SpamSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <SpamSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();

        SpamSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct SpamSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl SpamSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for SpamSystem {
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
                NetworkSimulationEvent::Message(_addr, payload) => info!("Payload: {:?}", payload),
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

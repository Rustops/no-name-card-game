use amethyst::network::simulation::udp::{UdpNetworkRecvSystem, UdpNetworkSendSystem};
use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, network::simulation::tcp::TcpNetworkBundle,
    prelude::*, utils::application_root_dir, Result,
};
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
use structopt::StructOpt;
use systems::chat::ChatReceiveBundle;

mod systems;

/// Default empty state
pub struct GameState;

impl SimpleState for GameState {}

fn main() -> Result<()> {
    let server = Server::init();
    server.run()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "server", author, about, no_version)]
pub struct Server {
    /// The websocket port of server.
    #[structopt(short, default_value = "6666")]
    pub port: u16,

    #[structopt(long, default_value = "server")]
    pub name: String,
}

impl Server {
    pub fn init() -> Self {
        Server::from_args()
    }
    pub fn run(self) -> Result<()> {
        let listener_addrs = SocketAddr::from(([0, 0, 0, 0], self.port));

        amethyst::start_logger(Default::default());
        let listener = TcpListener::bind(listener_addrs)?;
        listener.set_nonblocking(true)?;

        let assets_dir = application_root_dir()?.join("assets");
        let game_data = GameDataBuilder::default()
            .with_bundle(TcpNetworkBundle::new(Some(listener), 2048))?
            // The tcp bundle and the udp bundle have duplicate parts that will conflict and can only be added to the udp system separately
            .with(
                UdpNetworkRecvSystem::with_buffer_capacity(2048),
                "udp_recv",
                &["simulation_time"],
            )
            .with(UdpNetworkSendSystem, "udp_send", &["simulation_time"])
            .with_bundle(ChatReceiveBundle)?;

        let mut game = Application::build(assets_dir, GameState)?
            .with_frame_limit(
                FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
                60,
            )
            .build(game_data)?;
        game.run();
        Ok(())
    }
}

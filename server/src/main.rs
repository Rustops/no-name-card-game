use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, prelude::*, utils::application_root_dir, Result,
};
use std::net::{SocketAddr, TcpListener, UdpSocket};
use std::time::Duration;
use structopt::StructOpt;
use systems::service::ServiceBundle;

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

        let socket = UdpSocket::bind(listener_addrs)?;
        let assets_dir = application_root_dir()?.join("assets");
        let game_data =
            GameDataBuilder::default().with_bundle(ServiceBundle::new(listener, socket, 2048))?;

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

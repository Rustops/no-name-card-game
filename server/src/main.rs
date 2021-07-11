use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, network::simulation::tcp::TcpNetworkBundle,
    prelude::*, utils::application_root_dir, Result,
};
use std::net::TcpListener;
use std::time::Duration;
use systems::chat::ChatReceiveBundle;

mod systems;

/// Default empty state
pub struct GameState;

impl SimpleState for GameState {}

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    //    // UDP
    //    let socket = UdpSocket::bind("0.0.0.0:3457")?;
    //    socket.set_nonblocking(true)?;

    // TCP
    let listener = TcpListener::bind("0.0.0.0:3457")?;
    listener.set_nonblocking(true)?;

    //    // Laminar
    //    let socket = LaminarSocket::bind("0.0.0.0:3457")?;

    let assets_dir = application_root_dir()?.join("assets");

    let game_data = GameDataBuilder::default()
        //        // UDP
        //        .with_bundle(UdpNetworkBundle::new(Some(socket), 2048))?
        // TCP
        .with_bundle(TcpNetworkBundle::new(Some(listener), 2048))?
        //        // Laminar
        //        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
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

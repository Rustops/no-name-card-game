use std::net::{SocketAddr, TcpListener, UdpSocket};

use crate::{resources::Music, systems::chat::ChatroomBundle};
use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::TransformBundle,
    ecs::{Component, VecStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow, types::DefaultBackend, RenderDebugLines, RenderFlat2D,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::fps_counter::FpsCounterBundle,
    Result,
};
use states::loading::LoadingState;
use structopt::StructOpt;

mod common;
mod components;
mod entities;
mod resources;
mod states;
mod systems;
mod utilities;

use systems::{
    chat::{ClientInfoResource, ServerInfoResource},
    play_sfx::PlaySfxSystem,
};
use utilities::{
    files::{get_assets_dir, get_config_dir},
    startup::start_game,
};

fn main() -> amethyst::Result<()> {
    let client = Client::init();
    client.run()
}

impl Component for ServerInfoResource {
    type Storage = VecStorage<Self>;
}

#[derive(StructOpt, Debug)]
#[structopt(name = "no-name-card-game", author, about, no_version)]
pub struct Client {
    /// The websocket port of server.
    #[structopt(long, default_value = "127.0.0.1:6666")]
    pub url: String,

    #[structopt(long, default_value = "client")]
    pub name: String,
}

impl Client {
    pub fn init() -> Self {
        Client::from_args()
    }

    pub fn run(self) -> Result<()> {
        let server_info = ServerInfoResource { addr: self.url };
        let client_info = ClientInfoResource { name: self.name };
        amethyst::start_logger(Default::default());

        let display_config_path = get_config_dir().join("display.ron");

        let addr = SocketAddr::from(([0, 0, 0, 0], 2000));
        let listener_addr = SocketAddr::from(([0, 0, 0, 0], 2000));

        let listener = TcpListener::bind(listener_addr)?;
        listener.set_nonblocking(true)?;
        let socket = UdpSocket::bind(addr).unwrap();
        let _ = socket.set_nonblocking(true);

        let game_data = GameDataBuilder::default()
            .with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<StringBindings>::new())?
            .with_bundle(UiBundle::<StringBindings>::new())?
            .with_bundle(AudioBundle::default())?
            .with_bundle(FpsCounterBundle)?
            .with_bundle(ChatroomBundle::new(
                server_info,
                client_info,
                socket,
                listener,
            ))?
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config_path(display_config_path)?
                            .with_clear([0.0, 0.0, 0.0, 1.0]),
                    )
                    .with_plugin(RenderFlat2D::default())
                    .with_plugin(RenderUi::default())
                    .with_plugin(RenderDebugLines::default()), // .with_plugin(RenderFlat2D::default()),
            )?
            .with_system_desc(
                DjSystemDesc::new(|music: &mut Music| music.music.next()),
                "dj_system",
                &[],
            )
            .with(PlaySfxSystem::default(), "play_sfx_system", &[])
            .with_system_desc(
                systems::events::UiEventHandlerSystemDesc,
                "ui_event_handler",
                &[],
            );

        start_game(
            get_assets_dir(),
            game_data,
            Some(Box::new(LoadingState::default())),
        );

        Ok(())
    }
}

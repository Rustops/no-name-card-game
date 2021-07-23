use crate::systems::chat::ChatroomBundle;
use amethyst::{
    audio::AudioBundle,
    core::TransformBundle,
    ecs::{Component, VecStorage},
    input::{InputBundle, StringBindings},
    network::simulation::tcp::TcpNetworkBundle,
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    Result,
};
use structopt::StructOpt;
use systems::chat::ServerInfoResource;

mod states;
mod systems;

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
            .with_bundle(ChatroomBundle::new(server_info))?;
        let mut game = Application::new(
            assets_dir,
            states::welcome::WelcomeScreen::default(),
            game_data,
        )?;

        log::info!("Starting with WelcomeScreen!");
        game.run();
        log::info!("Game exit!");
        Ok(())
    }
}

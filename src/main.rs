use amethyst::{
    audio::AudioBundle,
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

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
        )?;

    let mut game = Application::new(
        assets_dir,
        states::welcome::WelcomeScreen::default(),
        game_data,
    )?;
    log::info!("Starting with WelcomeScreen!");
    game.run();
    Ok(())
}
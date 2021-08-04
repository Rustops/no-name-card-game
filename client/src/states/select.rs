use amethyst::{audio::output::init_output, ecs::Entity, ui::UiCreator, SimpleState, StateData};

#[derive(Debug, Default)]
pub struct Select {
    ui_root: Option<Entity>,
}

impl SimpleState for Select {
    fn on_start(&mut self, data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        // needed for registering audio output.
        init_output(&mut world);

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/select.ron", ())));
    }

    fn on_pause(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_stop(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}
}

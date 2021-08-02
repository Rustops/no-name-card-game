use amethyst::{ecs::Entity, SimpleState};

#[derive(Debug)]
pub struct Select {
    ui_root: Option<Entity>,
}

impl SimpleState for Select {
    fn on_start(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_pause(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_stop(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}
}

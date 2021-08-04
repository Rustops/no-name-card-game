use amethyst::{ecs::Entity, ui::UiCreator, SimpleState};

#[derive(Debug, Default)]
pub struct SelectState {
    ui_root: Option<Entity>,
}

impl SimpleState for SelectState {
    fn on_start(&mut self, data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/select.ron", ())));
    }

    fn on_pause(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_stop(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}
}

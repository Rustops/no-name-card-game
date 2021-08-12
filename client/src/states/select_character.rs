use amethyst::{ecs::Entity, GameData, SimpleState, StateData};

use crate::resources::{UiHandles, UiType};

#[derive(Debug, Default)]
pub struct SelectState {
    ui_root: Option<Entity>,
}

impl SelectState {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::CharacterSelection, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }
}

impl SimpleState for SelectState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_ui(&mut data);
    }

    fn on_pause(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}

    fn on_stop(&mut self, _data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}
}

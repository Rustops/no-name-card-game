use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::*,
    winit::{MouseButton, VirtualKeyCode},
};

use crate::{
    events::state_event::ExtendedStateEvent,
    resources::{UiHandles, UiType},
};

use super::menu::MainMenu;
// A simple 'Screen' State, only capable of loading/showing the prefab ui and registering simple
// UI interactions (pressing escape or clicking anywhere).

#[derive(Debug, Default)]
pub struct CreditsScreen {
    ui_root: Option<Entity>,
}

impl CreditsScreen {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_root = UiHandles::add_ui(UiType::Credits, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for CreditsScreen {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_ui(&mut data);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'a, 'b>>,
        event: ExtendedStateEvent,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        match &event {
            ExtendedStateEvent::Window(event) => {
                if is_close_requested(event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(event, VirtualKeyCode::Escape)
                    || is_mouse_button_down(event, MouseButton::Left)
                {
                    log::info!("[Trans::Switch] Switching to MainMenu!");
                    Trans::Switch(Box::new(MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove CreditScreen");
        }

        self.ui_root = None;
    }
}

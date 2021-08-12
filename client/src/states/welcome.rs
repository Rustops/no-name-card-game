use crate::resources::{UiHandles, UiType};
use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::*,
    winit::{MouseButton, VirtualKeyCode},
};

#[derive(Default, Debug)]
pub struct WelcomeScreen {
    ui_handle: Option<Entity>,
}

impl WelcomeScreen {
    fn init_ui(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) {
        self.ui_handle = UiHandles::add_ui(UiType::Welcome, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(data.world);
    }
}

impl SimpleState for WelcomeScreen {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_ui(&mut data);
        // initialize_audio(data.world);
    }

    fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(event) || is_key_down(event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_mouse_button_down(event, MouseButton::Left) {
                    log::info!("[Trans::Switch] Switching to MainMenu!");
                    Trans::Switch(Box::new(super::menu::MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove WelcomeScreen");
        }

        self.ui_handle = None;
    }
}

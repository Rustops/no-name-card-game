use amethyst::{ecs::prelude::World, ui::UiCreator};

use crate::{
    components::{Player, PlayerRole, PlayerState},
    systems::chat::ClientInfoResource,
};

pub fn _load_player(world: &mut World) {
    let client = world.fetch::<ClientInfoResource>();
    let _player = Player::new(
        client.name.clone(),
        PlayerState::Chatting,
        false,
        PlayerRole::default(),
    );

    // _load_player_role(world, player);
    log::info!("[Load::Player] Name: Default_Flandre, Role: Default");
    // world.
}

pub fn _load_player_role(world: &mut World, player: Player) {
    let img_path = match player.role {
        PlayerRole::Flandre => "ui/default_player.ron",
    };
    world.exec(|mut creator: UiCreator<'_>| creator.create(img_path, ()));
}

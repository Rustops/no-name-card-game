use amethyst::{ecs::prelude::World, ui::UiCreator};

use crate::components::{Player, PlayerRole};

pub fn load_player(world: &mut World) {
    let player = Player::new();
    load_player_role(world, player);
    // log::info!("[Load::Player] Name: Default_Flandre, Role: Default");
    // world
    //     .create_entity()
    //     .with(Player::new())
    //     .build();
}

pub fn load_player_role(world: &mut World, player: Player) {
    let img_path = match player.role {
        PlayerRole::Flandre => "ui/default_player.ron",
    };
    world.exec(|mut creator: UiCreator<'_>| creator.create(img_path, ()));
}

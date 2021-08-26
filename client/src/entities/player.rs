use amethyst::{
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    components::{Player, PlayerState},
    resources::{Assets, CharacterType},
    systems::chat::ClientInfoResource,
};

pub fn _load_player(world: &mut World) {
    let client_name = {
        let client = world.fetch::<ClientInfoResource>();
        client.name.clone()
    };

    let character = {
        let assets = world.fetch::<Assets>();
        assets.get_character(CharacterType::Alice)
    };
    log::info!("[Load::Player] Name: {}, CharacterType: Alice", client_name);
    let player = Player::new(
        client_name,
        PlayerState::Chatting,
        false,
        CharacterType::Alice,
    );
    let render = SpriteRender {
        sprite_sheet: character,
        sprite_number: 1,
    };
    let transform = Transform::from(Vector3::new(100.0, 200.0, 300.0));
    world
        .create_entity()
        .with(player)
        .with(render)
        .with(transform)
        .build();
}

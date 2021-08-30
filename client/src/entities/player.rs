use amethyst::{
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
};

use crate::{
    common::{DepthLayer, Pos},
    components::{Player, PlayerState},
    resources::{AssetType, Assets, Avatar, CharacterType},
    systems::chat::ClientInfoResource,
    utilities::load::load_transform,
};

pub fn load_player(world: &mut World) {
    let client_name = {
        let client = world.read_resource::<ClientInfoResource>();
        client.name.clone()
    };

    let avater = {
        let assets = world.read_resource::<Assets>();
        assets.get_avatar(Avatar::Default)
    };
    log::info!("[Load::Avater] {:?}", avater);

    let player = Player::new(
        client_name,
        PlayerState::Chatting,
        false,
        CharacterType::Alice,
    );
    log::info!("[Load::Player] {:?}", player);

    let transform = load_transform(
        Pos::new(0, 0),
        DepthLayer::UiElements,
        Pos::new(1, 1),
        &AssetType::Character(CharacterType::Alice, 3),
    );
    log::info!("[Load::Transform] {:?}", transform);
    
    world
        .create_entity()
        .with(player)
        .with(transform)
        .with(avater)
        .build();
}

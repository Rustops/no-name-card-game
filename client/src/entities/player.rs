use amethyst::{
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    ui::{Anchor, UiImage, UiTransform},
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
        client_name.clone(),
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

    let ui_image = UiImage::Texture(avater);
    let ui_transfrom = UiTransform::new(
        format!("avater_{}", client_name),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        32.,
        200.,
        145.,
        98.,
    );
    log::info!("[Load::UiTransform] {:?}", ui_transfrom);

    world
        .create_entity()
        .with(player)
        .with(ui_image)
        .with(ui_transfrom)
        .build();
}

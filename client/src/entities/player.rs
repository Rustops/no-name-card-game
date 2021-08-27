use amethyst::{
    core::Transform,
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::{
        camera::{ARENA_HEIGHT, ARENA_WIDTH},
        DepthLayer, Pos,
    },
    components::{Player, PlayerState},
    resources::{AssetType, Assets, Avatar, CharacterType, SpriteType},
    systems::chat::ClientInfoResource,
    utilities::load::load_transform,
};

pub fn load_player(world: &mut World) {
    let client_name = {
        let client = world.read_resource::<ClientInfoResource>();
        client.name.clone()
    };

    let character = {
        let assets = world.read_resource::<Assets>();
        assets.get_still(SpriteType::Avatar(Avatar::Default))
    };
    log::info!("[Load::Character] {:?}", character);

    let player = Player::new(
        client_name,
        PlayerState::Chatting,
        false,
        CharacterType::Alice,
    );
    log::info!("[Load::Player] {:?}", player);

    let render = SpriteRender {
        sprite_sheet: character,
        sprite_number: 1,
    };
    log::info!("[Load::Render] {:?}", render);

    // let transform = Transform::from(Vector3::new(100.0, 200.0, 300.0));
    let _transform = load_transform(
        Pos::new(0, 0),
        DepthLayer::UiElements,
        Pos::new(1, 1),
        &AssetType::Character(CharacterType::Alice, 3),
    );
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);
    log::info!("[Load::Transform] {:?}", transform);

    world
        .create_entity()
        .with(player)
        .with(transform)
        .with(render)
        .build();
}

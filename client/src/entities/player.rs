use amethyst::{
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    common::{DepthLayer, Pos},
    components::{Player, PlayerState},
    resources::{AssetType, Assets, Avatar, CharacterType},
    utilities::load::{load_transform, load_ui_text},
};

pub fn load_player(world: &mut World, name: String, num: usize) {
    let avater = {
        let assets = world.read_resource::<Assets>();
        assets.get_avatar(Avatar::Default)
    };
    log::info!("[Load::Avater] {:?}", avater);

    let background_asset = {
        let assets = world.read_resource::<Assets>();
        assets.get_avatar(Avatar::Background)
    };

    let background_image = UiImage::Texture(background_asset);
    let background_transfrom = UiTransform::new(
        format!("player_{}_background", name),
        Anchor::Middle,
        Anchor::Middle,
        -300. + num as f32 * 200.,
        30.,
        130.,
        150.,
        250.,
    );

    world
        .create_entity()
        .with(background_image)
        .with(background_transfrom)
        .build();

    let ui_name = load_ui_text(world, name.clone());
    let ui_name_transfrom = UiTransform::new(
        format!("player_{}_name", name),
        Anchor::Middle,
        Anchor::Middle,
        -300. + num as f32 * 200.,
        140.,
        200.,
        145.,
        18.,
    );

    world
        .create_entity()
        .with(ui_name)
        .with(ui_name_transfrom)
        .build();

    let ui_level = load_ui_text(world, "等级： 菜狗  ".to_owned());
    let ui_level_transfrom = UiTransform::new(
        format!("player_{}_level", name),
        Anchor::Middle,
        Anchor::Middle,
        -300. + num as f32 * 200.,
        -30.,
        200.,
        145.,
        18.,
    );

    world
        .create_entity()
        .with(ui_level)
        .with(ui_level_transfrom)
        .build();

    let ui_record = load_ui_text(world, "战绩： 0胜0败".to_owned());
    let ui_reocrd_transfrom = UiTransform::new(
        format!("player_{}_level", name),
        Anchor::Middle,
        Anchor::Middle,
        -300. + num as f32 * 200.,
        -60.,
        200.,
        145.,
        18.,
    );

    world
        .create_entity()
        .with(ui_record)
        .with(ui_reocrd_transfrom)
        .build();

    let player = Player::new(
        name.clone(),
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
        format!("player_{}_avater", name),
        Anchor::Middle,
        Anchor::Middle,
        -300. + num as f32 * 200.,
        60.,
        200.,
        145.,
        145.,
    );
    log::info!("[Load::UiTransform] {:?}", ui_transfrom);

    world
        .create_entity()
        .with(player)
        .with(ui_image)
        .with(ui_transfrom)
        .build();
}

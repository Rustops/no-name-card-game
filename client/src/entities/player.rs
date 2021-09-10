// use amethyst::{
//     ecs::prelude::World,
//     prelude::{Builder, WorldExt},
//     ui::{Anchor, UiImage, UiTransform},
// };

// use crate::{
//     common::{DepthLayer, Pos},
//     components::{Player, PlayerState},
//     resources::{AssetType, Assets, Avatar, CharacterType},
//     utilities::load::load_transform,
// };

// TODO： Optimize loading player
// This way of loading the player should be used in the state， we need to load it in the system

// pub fn load_player(world: &mut World, name: String, num: usize) {
//     let avater = {
//         let assets = world.read_resource::<Assets>();
//         assets.get_avatar(Avatar::Default)
//     };
//     log::info!("[Load::Avater] {:?}", avater);

//     let player = Player::new(
//         name.clone(),
//         PlayerState::Chatting,
//         false,
//         CharacterType::Alice,
//     );
//     log::info!("[Load::Player] {:?}", player);

//     let transform = load_transform(
//         Pos::new(0, 0),
//         DepthLayer::UiElements,
//         Pos::new(1, 1),
//         &AssetType::Character(CharacterType::Alice, 3),
//     );
//     log::info!("[Load::Transform] {:?}", transform);

//     let ui_image = UiImage::Texture(avater);
//     let ui_transfrom = UiTransform::new(
//         format!("avater_{}", name),
//         Anchor::Middle,
//         Anchor::Middle,
//         -300. + num as f32 * 200.,
//         32.,
//         200.,
//         145.,
//         98.,
//     );
//     log::info!("[Load::UiTransform] {:?}", ui_transfrom);

//     world
//         .create_entity()
//         .with(player)
//         .with(ui_image)
//         .with(ui_transfrom)
//         .build();
// }

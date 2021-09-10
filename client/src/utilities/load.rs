// use crate::{
//     common::{DepthLayer, Pos},
//     resources::{get_asset_dimensions, AssetType},
// };
// use amethyst::core::{math::Vector3, Transform};

// pub fn load_transform(pos: Pos, depth: DepthLayer, dimens: Pos, asset: &AssetType) -> Transform {
//     let asset_dimensions = get_asset_dimensions(asset);
//     let mut transform = Transform::default();
//     transform.set_translation_xyz(
//         pos.x as f32 + dimens.x as f32 * 0.5,
//         pos.y as f32 + dimens.y as f32 * 0.5,
//         depth.z(),
//     );
//     transform.set_scale(Vector3::new(
//         dimens.x as f32 / asset_dimensions.x as f32,
//         dimens.y as f32 / asset_dimensions.y as f32,
//         1.0,
//     ));
//     transform
// }

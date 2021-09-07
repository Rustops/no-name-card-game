use crate::{
    common::{DepthLayer, Pos},
    resources::{get_asset_dimensions, AssetType},
};
use amethyst::{
    assets::Loader,
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::WorldExt,
    ui::{TtfFormat, UiText},
};

pub fn load_transform(pos: Pos, depth: DepthLayer, dimens: Pos, asset: &AssetType) -> Transform {
    let asset_dimensions = get_asset_dimensions(asset);
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        pos.x as f32 + dimens.x as f32 * 0.5,
        pos.y as f32 + dimens.y as f32 * 0.5,
        depth.z(),
    );
    transform.set_scale(Vector3::new(
        dimens.x as f32 / asset_dimensions.x as f32,
        dimens.y as f32 / asset_dimensions.y as f32,
        1.0,
    ));
    transform
}

pub fn load_ui_text(world: &mut World, text: String) -> UiText {
    let font = world.read_resource::<Loader>().load(
        "font/AaWuShiXiaoShenXian.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    UiText::new(
        font,
        text,
        [1., 1., 1., 1.],
        18.,
        amethyst::ui::LineMode::Single,
        amethyst::ui::Anchor::Middle,
    )
}

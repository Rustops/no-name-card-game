#[derive(Clone, Copy, Default)]
pub struct Context {
    pub map_width: f32,
    pub bg_width: f32,
    pub bg_height: f32,
    pub x_correction: f32,
    pub y_correction: f32,
    pub bg_z_translation: f32,
    pub truss_z_translation: f32,
    pub platform_z_translation: f32,
    pub scale: f32,
}

use bevy::{
    asset::Asset,
    color::LinearRgba,
    math::Vec4,
    pbr::Material,
    reflect::TypePath,
    render::{
        alpha::AlphaMode,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct SaturnRingMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(1)]
    // pub ring_params: Vec4, // (inner_radius, outer_radius, fade, gap_factor),
    pub inner_distance_from_body: f32,
    pub outer_distance_from_body: f32,
    pub gradient: Vec4,
    pub alpha_mode: AlphaMode,
}

impl Material for SaturnRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/saturn_rings.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

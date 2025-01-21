use bevy::{color::Color, prelude::*};

use super::settings::{FollowBody, SimulationParameters};

#[derive(Debug, Component, Clone)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>)]
pub struct Body {
    pub data: BodyData,
    pub metadata: BodyMetadata,
}

impl Body {
    pub fn downscale_by_astronomical_units(&self, parameters: &SimulationParameters) -> Self {
        Self {
            data: self.data.downscaled(parameters),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Clone, Component, Copy)]
pub struct BodyData {
    /// Absolute position in world space, in Astronomical Units
    pub position: Vec3,

    /// Astronomical Units per day
    pub velocity: Vec3,

    /// Astronomical Units per day
    pub acceleration: Vec3,
    pub rotation: Quat,
    pub axial_tilt: Vec3,

    /// In Solar Mass
    pub mass: f32,

    /// Mean radius in Astronomical Units
    pub radius: f32,

    /// Surface temperature in Kelvin
    pub temperature: f32,
}

impl Default for BodyData {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            axial_tilt: Vec3::ZERO,
            mass: 0.0,
            radius: 0.0,
            temperature: 0.0,
        }
    }
}

impl BodyData {
    pub fn downscaled(&self, parameters: &SimulationParameters) -> Self {
        Self {
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,
            rotation: self.rotation,
            axial_tilt: self.axial_tilt,
            mass: self.mass / parameters.mass_scale,
            radius: self.radius / parameters.unit_scale,
            temperature: self.temperature,
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct BodyMetadata {
    pub color: Color,
    pub name: Option<&'static str>,
    pub texture: Option<Handle<Image>>,
}

impl Default for BodyMetadata {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            name: None,
            texture: None,
        }
    }
}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Planet {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Moon {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Star {}

#[derive(Resource)]
pub struct Sun(pub Entity);

pub fn follow_body_system(
    mut camera: Query<&mut Transform, With<Camera>>,
    query: Query<(&Transform, &Body), Without<Camera>>,
    sun: Res<Sun>,
    follow: Res<FollowBody>,
) {
    if follow.is_active && follow.entity.is_some() {
        if let Ok((transform, body)) = query.get(follow.entity.unwrap()) {
            let sun = query.get(sun.0);
            camera.single_mut().translation = Vec3 {
                x: transform.translation.x
                    + body.data.radius
                    + match sun {
                        Ok((_, b)) => b.data.radius,
                        Err(_) => 0.2,
                    },
                y: transform.translation.y,
                z: transform.translation.z,
            }
        }
    }
}

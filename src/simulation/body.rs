use std::cell::{Ref, RefCell};
use std::fmt;
use std::sync::{Arc, RwLock};

use bevy::{color::Color, prelude::*};
use serde::Deserialize;

use super::settings::{FollowBody, SimulationParameters};
use super::util::{deserialize_color, deserialize_satellites, deserialize_vec3};

#[derive(Debug, Component, Clone, Deserialize)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>)]
pub struct Body {
    pub data: BodyData,
    pub metadata: BodyMetadata,

    #[serde(default, deserialize_with = "deserialize_satellites")]
    pub satellites: Option<Vec<Arc<RwLock<Body>>>>,
}

#[derive(Debug, Clone, Component, Copy, Deserialize)]
pub struct BodyOrbitalElements {
    /// Tilt of the body's orbit
    pub inclination: f32,
    pub longitude_of_ascending_node: f32,
    pub true_anomaly: f32,

    /// AKA argument of periapsis
    pub argument_of_perifocus: f32,
}

#[derive(Debug, Clone, Component, Copy, Deserialize)]
pub struct BodyData {
    /// Absolute position in world space, in Astronomical Units
    #[serde(deserialize_with = "deserialize_vec3")]
    pub position: Vec3,

    /// Astronomical Units per day
    #[serde(deserialize_with = "deserialize_vec3")]
    pub velocity: Vec3,

    /// Astronomical Units per day
    #[serde(skip)]
    pub acceleration: Vec3,

    /// In Solar Mass
    #[serde(default)]
    pub mass: f32,

    /// Mean radius in Astronomical Units
    #[serde(default)]
    pub radius: f32,

    /// Surface temperature in Kelvin
    #[serde(default)]
    pub temperature: f32,

    /// g/cm^3
    #[serde(default)]
    pub density: f32,

    /// rad/s
    #[serde(default)]
    pub rotation: f32,

    /// AKA axial tilt, relative to orbit
    #[serde(default)]
    pub obliquity: f32,

    #[serde(default)]
    pub orbital_elements: Option<BodyOrbitalElements>,
}

impl Default for BodyData {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            rotation: 0.0,
            obliquity: 0.0,
            mass: 0.0,
            radius: 0.0,
            temperature: 0.0,
            density: 1.0,
            orbital_elements: None,
        }
    }
}

impl BodyData {
    /// No need for it now, but might be useful in the future
    pub fn downscaled(&self, parameters: &SimulationParameters) -> Self {
        Self {
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,
            rotation: self.rotation,
            obliquity: self.obliquity,
            mass: self.mass / parameters.mass_scale,
            radius: self.radius / parameters.unit_scale,
            temperature: self.temperature,
            density: self.density,
            orbital_elements: self.orbital_elements,
        }
    }
}

#[derive(Debug, Component, Clone, Deserialize)]
#[serde(default)]
pub struct BodyMetadata {
    pub name: Option<String>,
    // TODO usize
    pub id: Option<u32>,

    #[serde(default, deserialize_with = "deserialize_color")]
    pub color: Color,

    #[serde(skip)]
    pub texture: Option<Handle<Image>>,

    #[serde(skip)]
    pub body_type: BodyType,
}

impl Default for BodyMetadata {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            name: None,
            id: None,
            texture: None,
            body_type: BodyType::Unknown,
        }
    }
}

/// To be used when loading data.
#[derive(Debug, Clone, Default)]
pub struct MetaLoader {
    pub texture: Option<Handle<Image>>,
    pub body_type: BodyType,
}

impl MetaLoader {
    pub fn new(texture: Option<Handle<Image>>, body_type: BodyType) -> Self {
        Self { texture, body_type }
    }
}

#[derive(Debug, Clone, Default)]
pub enum BodyType {
    Star,
    Planet,
    DwarfPlanet,
    Moon,
    Other,
    #[default]
    Unknown,
}

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            BodyType::Star => "Star",
            BodyType::Planet => "Planet",
            BodyType::DwarfPlanet => "Dwarf Planet",
            BodyType::Moon => "Moon",
            BodyType::Other => "Other",
            BodyType::Unknown => "Unknown",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Star {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Planet {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct DwarfPlanet {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Moon {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Other {}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Unknown {}

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

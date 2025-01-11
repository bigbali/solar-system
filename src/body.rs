use std::f32::{INFINITY, NAN};

use bevy::{
    color::{palettes::css::RED, Color},
    prelude::*,
    render::mesh::Mesh,
};

use crate::{planets::planets, SimulationSpeedMultiplier /* Sun */};

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Planet {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Moon {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Star {}

#[derive(Resource)]
pub struct Sun(pub Entity);

#[derive(Debug, Clone, Component, Copy)]
pub struct BodyData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub rotation: Vec3,
    pub axial_tilt: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub temperature: f32,
}

impl Default for BodyData {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            rotation: Vec3::ZERO,
            axial_tilt: Vec3::ZERO,
            mass: 0.0,
            radius: 0.0,
            temperature: 0.0,
        }
    }
}

impl BodyData {
    pub fn downscaled(&self) -> Self {
        Self {
            position: self.position / SCALE,
            velocity: self.velocity / SCALE,
            acceleration: self.acceleration,
            rotation: self.rotation,
            axial_tilt: self.axial_tilt,
            mass: self.mass / MASS_SCALE,
            radius: self.radius / SCALE,
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

#[derive(Debug, Component, Clone)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>)]
pub struct Body {
    pub data: BodyData,
    pub metadata: BodyMetadata,
}

const SCALE: f32 = 1_000_000.0; // 1 million meters to 1 unit
const MASS_SCALE: f32 = 1_000_000_000_000.0; // 1 trillion kg to 1 unit
const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
const G: f32 = GRAVITATIONAL_CONSTANT / MASS_SCALE;

const SOFTENING_FACTOR: f32 = 0.001;

pub fn bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for planet in planets(&asset_server) {
        commands
            .spawn((
                planet.clone(),
                Mesh3d(meshes.add(Sphere {
                    radius: planet.data.radius,
                })),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: planet.metadata.color,
                    base_color_texture: planet.metadata.texture,
                    ..default()
                })),
                Transform::from_translation(planet.data.position),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2d::new(planet.metadata.name.unwrap()),
                    TextColor::from(RED),
                    Transform::from_translation(Vec3::new(0.0, planet.data.radius + 50.0, 0.0)),
                ));
            });
    }

    let sun_texture: Handle<Image> = asset_server.load("sun.jpg");
    let sun_radius = 696_300.0;

    let sun = commands
        .spawn((
            Body {
                data: BodyData {
                    position: Vec3::ZERO,
                    velocity: Vec3::ZERO,
                    acceleration: Vec3::ZERO,
                    rotation: Vec3::new(0.0, 0.0, 0.0),
                    axial_tilt: Vec3::new(0.0, 0.0, 0.0),
                    mass: 1.9891e+30,
                    radius: sun_radius,
                    temperature: 5778.0,
                }
                .downscaled(),
                metadata: BodyMetadata {
                    color: Color::linear_rgb(0.5, 0.5, 0.0),
                    name: Some("Sun"),
                    texture: Some(sun_texture.clone()),
                },
            },
            Mesh3d(meshes.add(Sphere {
                radius: sun_radius / SCALE,
            })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive_texture: Some(sun_texture),
                emissive: LinearRgba::WHITE,
                ..default()
            })),
            Star {},
        ))
        .with_child(PointLight {
            shadows_enabled: true,
            color: Color::WHITE,
            range: f32::MAX,
            radius: sun_radius / SCALE * 1.05,
            intensity: 1_000_000_000_000.0,
            shadow_depth_bias: 0.0,
            shadow_map_near_z: 0.0,
            shadow_normal_bias: 0.0,
            ..default()
        })
        .id();

    commands.insert_resource(Sun(sun));
}

pub fn planets_update_system(
    time: Res<Time>,
    mut body_query: Query<(&mut Transform, &mut Body)>,
    speed: Res<SimulationSpeedMultiplier>,
) {
    let planet_data: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    for (mut transform_outer, mut body_outer) in body_query.iter_mut() {
        transform_outer.rotate_x(body_outer.data.rotation.x);
        transform_outer.rotate_y(body_outer.data.rotation.y);
        transform_outer.rotate_z(body_outer.data.rotation.z);

        for (transform_inner, body_inner) in &planet_data {
            if body_inner.metadata.name == body_outer.metadata.name {
                continue;
            }

            let distance_squared = transform_inner
                .translation
                .distance_squared(transform_outer.translation)
                + SOFTENING_FACTOR;
            let force_direction =
                (transform_inner.translation - transform_outer.translation).normalize();

            let f = force_direction * G * body_outer.data.mass * body_inner.data.mass
                / distance_squared;

            let acceleration = f / body_outer.data.mass;

            let delta = time.delta_secs() * speed.0;

            body_outer.data.velocity += acceleration * delta;

            transform_outer.translation += body_outer.data.velocity * speed.0;

            assert_ne!(distance_squared, INFINITY);
            assert_ne!(force_direction, Vec3::INFINITY);
            assert_ne!(f, Vec3::INFINITY);
            assert_ne!(acceleration, Vec3::INFINITY);
            assert_ne!(distance_squared, NAN);
            assert_ne!(force_direction, Vec3::NAN);
            assert_ne!(f, Vec3::NAN);
            assert_ne!(acceleration, Vec3::NAN);
            assert_ne!(body_outer.data.velocity, Vec3::NAN);
            assert_ne!(transform_outer.translation, Vec3::NAN);
        }
    }
}

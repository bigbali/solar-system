use std::f32::{INFINITY, NAN};

use bevy::{
    color::{palettes::tailwind, Color},
    math::VectorSpace,
    prelude::*,
    render::{mesh::Mesh, texture},
};

use crate::{planets::planets, SimulationSpeedMultiplier /* Sun */};

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Planet {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Moon {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Star {}

// #[derive(Debug, Clone, Component, Copy)]
// pub struct BodyData {
//     pub position: Vec3,
//     pub velocity: Vec3,
//     pub acceleration: Vec3,
//     pub rotation: Vec3,
//     // pub rotation_velocity: Vec3,
//     pub axial_tilt: Vec3,
//     pub mass: f32,
//     pub radius: f32,
//     pub temperature: f32,
//     pub color: Color,
//     pub name: Option<&'static str>,
//     pub texture: Option<&'static str>,
// }

// impl Default for BodyData {
//     fn default() -> Self {
//         Self {
//             position: Vec3::ZERO,
//             velocity: Vec3::ZERO,
//             acceleration: Vec3::ZERO,
//             rotation: Vec3::ZERO,
//             // rotation_velocity: Vec3::ZERO,
//             axial_tilt: Vec3::ZERO,
//             mass: 0.0,
//             radius: 0.0,
//             temperature: 0.0,
//             color: Color::WHITE,
//             name: None,
//             texture: None,
//         }
//     }
// }

// #[derive(Bundle)]
// pub struct ObjectBundle {
//     pub body: Body,
//     pub pbr: PbrBundle,
// }

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
// const G: f32 = GRAVITATIONAL_CONSTANT / (SCALE * SCALE * MASS_SCALE);
const G: f32 = GRAVITATIONAL_CONSTANT / MASS_SCALE;

const SOFTENING_FACTOR: f32 = 0.001;

pub fn bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let sun_body = BodyBundle::new(
    //     "Sun",
    //     Color::from(tailwind::YELLOW_500),
    //     696_300.0,
    //     Vec3::new(0.01, 0.01, 0.01),
    //     1.9891e+30,
    //     // 100000000000.0,
    //     Vec3::new(0.01, 0.01, 0.01),
    //     Vec3::new(0.0, 0.017, 0.0),
    //     Vec3::new(0.0, 17.0, 0.0),
    //     "sun.jpg",
    // );

    // let planets = vec![
    //     BodyBundle::new(
    //         "Mercury",
    //         Color::from(tailwind::GRAY_500),
    //         2_439.7,
    //         Vec3::new(69_820_000.0, 0.1, 0.1),
    //         // 69_820_000.0,
    //         // 1000.0,
    //         3.3011e23,
    //         // 1000.0,
    //         Vec3::new(0.001, 0.001, 47.36),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "mercury.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Venus",
    //         Color::from(tailwind::AMBER_500),
    //         6_051.8,
    //         Vec3::new(108_940_000.0, 0.1, 0.1),
    //         // 108_940_000.0,,
    //         // 2000.0,
    //         4.8675e24,
    //         // 10000.0,
    //         Vec3::new(0.001, 0.0001, 35.02),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "venus.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Earth",
    //         Color::from(tailwind::BLUE_500),
    //         6_371.0,
    //         // 152_097_597.0,
    //         Vec3::new(152_097_597.0, 0.1, 0.1),
    //         // 10000.0,
    //         5.97237e24,
    //         // 10000.0,
    //         Vec3::new(0.01, 0.01, 29.7827),
    //         Vec3::new(0.0, 0.008, 0.0),
    //         Vec3::new(0.0, 27.0, 0.0),
    //         "earth.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Mars",
    //         Color::from(tailwind::RED_500),
    //         3_389.5,
    //         // 249_261_000.0,
    //         Vec3::new(249_261_000.0, 0.1, 0.1),
    //         6.4171e23,
    //         // 0.04,
    //         Vec3::new(0.01, 0.01, 24.07),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "mars.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Jupiter",
    //         Color::from(tailwind::YELLOW_500),
    //         142_984.0,
    //         // 816_081_455.0,
    //         Vec3::new(816_081_455.0, 0.1, 0.1),
    //         1.8982E27,
    //         // 0.1,
    //         Vec3::new(0.01, 0.01, 13.07),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "jupiter.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Saturn",
    //         Color::from(tailwind::YELLOW_300),
    //         58_232.0,
    //         // 1_514_500_000.0,
    //         Vec3::new(1_514_500_000.0, 0.1, 0.1),
    //         // 5.6834e26,
    //         0.08,
    //         Vec3::new(0.1, 0.1, 30.01),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "saturn.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Uranus",
    //         Color::from(tailwind::TEAL_500),
    //         25_362.0,
    //         // 3_006_390_000.0,
    //         Vec3::new(3_006_390_000.0, 0.1, 0.1),
    //         // 8.6810e25,
    //         0.06,
    //         Vec3::new(0.1, 0.1, 50.0),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "uranus.jpg",
    //     ),
    //     BodyBundle::new(
    //         "Neptune",
    //         Color::from(tailwind::BLUE_700),
    //         24_622.0,
    //         // 4_540_000_000.0,
    //         Vec3::new(4_540_000_000.0, 0.1, 0.1),
    //         // 1.02413e26,
    //         0.07,
    //         Vec3::new(0.1, 0.1, 80.0),
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 7.0, 0.0),
    //         "neptune.jpg",
    //     ),
    // ];

    for planet in planets() {
        commands.spawn((
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
        ));
    }

    // commands.insert_resource(Sun(sun));

    let sun_texture: Handle<Image> = asset_server.load("sun.jpg");

    commands.spawn((
        Body {
            data: BodyData {
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                rotation: Vec3::new(0.0, 0.0, 0.0),
                axial_tilt: Vec3::new(0.0, 0.0, 0.0),
                mass: 1.9891e+30,
                radius: 696_300.0,
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
            radius: 696_300.0 / SCALE,
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            // base_color_texture: Some(custom_texture_handle.clone()),
            emissive_texture: Some(sun_texture),
            emissive: LinearRgba::WHITE,
            ..default()
        })),
        PointLight {
            shadows_enabled: true,
            color: Color::WHITE,
            range: f32::MAX,
            radius: 10.0,
            intensity: 1_000_000_000_000_000.0 / SCALE,
            ..default()
        },
        Star {},
    ));
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
        // transform_outer.rotate_x(body_outer.data.rotation.x);
        // transform_outer.rotate_y(body_outer.data.rotation.y);
        // transform_outer.rotate_z(body_outer.data.rotation.z);

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

            // body_outer.data.velocity += acceleration * time.delta_seconds();
            // transform_outer.translation += body_outer.data.velocity * speed.0;

            let delta = time.delta_secs() * speed.0;

            body_outer.data.velocity += acceleration * delta; // Adjusted for simulation speed
                                                              // println!(
                                                              //     "{:?} {:?}",
                                                              //     body_outer.metadata.name, body_outer.data.velocity
                                                              // );
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

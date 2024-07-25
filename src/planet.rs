use std::f32::{INFINITY, NAN};

use bevy::{
    color::{palettes::tailwind, Color},
    prelude::*,
    render::mesh::Mesh,
};

use crate::SimulationSpeedMultiplier;

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Object {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Planet {}
#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Star {}

#[derive(Debug, Clone, Component, Copy)]
pub struct BodyData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub rotation: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub temperature: f32,
    pub color: Color,
    pub name: Option<&'static str>,
}

impl Default for BodyData {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(1.0, 1.0, -10.0),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            mass: 0.0,
            radius: 0.0,
            temperature: 0.0,
            color: Color::WHITE,
            name: None,
        }
    }
}

#[derive(Bundle)]
pub struct ObjectBundle {
    pub body: Body,
    pub pbr: PbrBundle,
}

#[derive(Bundle)]
pub struct BodyBundle {}

impl BodyBundle {
    pub fn new(
        name: &'static str,
        color: Color,
        radius: f32,
        position: Vec3,
        mass: f32,
        velocity: Vec3,
    ) -> Body {
        Body {
            data: BodyData {
                position: position / SCALE,
                radius: radius / SCALE,
                mass: mass / MASS_SCALE,
                color,
                name: Some(name),
                velocity: velocity / SCALE,
                ..default()
            },
        }
    }
}

#[derive(Debug, Component, Clone, Default, Copy)]
pub struct Body {
    pub data: BodyData,
}

// const SCALE: f32 = 100_000_000.0;
// const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
// const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11 * SCALE;
// const GRAVITATIONAL_CONSTANT: f32 = 0.001;

const SCALE: f32 = 1_000_000.0; // 1 million meters to 1 unit
const MASS_SCALE: f32 = 1_000_000_000_000.0; // 1 trillion kg to 1 unit
const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
// const G: f32 = GRAVITATIONAL_CONSTANT / (SCALE * SCALE * MASS_SCALE);
const G: f32 = GRAVITATIONAL_CONSTANT / MASS_SCALE;

const SOFTENING_FACTOR: f32 = 0.001;

pub fn planets_create_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun = BodyBundle::new(
        "Sun",
        Color::from(tailwind::YELLOW_500),
        696_300.0,
        Vec3::new(0.01, 0.01, 0.01),
        1.9891e+30,
        // 100000000000.0,
        Vec3::new(0.01, 0.01, 0.01),
    );

    let planets = vec![
        BodyBundle::new(
            "Mercury",
            Color::from(tailwind::GRAY_500),
            2_439.7,
            Vec3::new(69_820_000.0, 0.1, 0.1),
            // 69_820_000.0,
            // 1000.0,
            3.3011e23,
            // 1000.0,
            Vec3::new(0.001, 0.001, 47.36),
        ),
        BodyBundle::new(
            "Venus",
            Color::from(tailwind::AMBER_500),
            6_051.8,
            Vec3::new(108_940_000.0, 0.1, 0.1),
            // 108_940_000.0,,
            // 2000.0,
            4.8675e24,
            // 10000.0,
            Vec3::new(0.001, 0.0001, 35.02),
        ),
        BodyBundle::new(
            "Earth",
            Color::from(tailwind::BLUE_500),
            6_371.0,
            // 152_097_597.0,
            Vec3::new(152_097_597.0, 0.1, 0.1),
            // 10000.0,
            5.97237e24,
            // 10000.0,
            Vec3::new(0.01, 0.01, 29.7827),
        ),
        BodyBundle::new(
            "Mars",
            Color::from(tailwind::RED_500),
            3_389.5,
            // 249_261_000.0,
            Vec3::new(249_261_000.0, 0.1, 0.1),
            6.4171e23,
            // 0.04,
            Vec3::new(0.01, 0.01, 24.07),
        ),
        BodyBundle::new(
            "Jupiter",
            Color::from(tailwind::YELLOW_500),
            142_984.0,
            // 816_081_455.0,
            Vec3::new(816_081_455.0, 0.1, 0.1),
            1.8982E27,
            // 0.1,
            Vec3::new(0.01, 0.01, 13.07),
        ),
        BodyBundle::new(
            "Saturn",
            Color::from(tailwind::YELLOW_300),
            58_232.0,
            // 1_514_500_000.0,
            Vec3::new(1_514_500_000.0, 0.1, 0.1),
            // 5.6834e26,
            0.08,
            Vec3::new(0.1, 0.1, 30.01),
        ),
        BodyBundle::new(
            "Uranus",
            Color::from(tailwind::TEAL_500),
            25_362.0,
            // 3_006_390_000.0,
            Vec3::new(3_006_390_000.0, 0.1, 0.1),
            // 8.6810e25,
            0.06,
            Vec3::new(0.1, 0.1, 50.0),
        ),
        BodyBundle::new(
            "Neptune",
            Color::from(tailwind::BLUE_700),
            24_622.0,
            // 4_540_000_000.0,
            Vec3::new(4_540_000_000.0, 0.1, 0.1),
            // 1.02413e26,
            0.07,
            Vec3::new(0.1, 0.1, 80.0),
        ),
    ];

    for planet in planets {
        commands.spawn((
            ObjectBundle {
                body: planet,
                pbr: PbrBundle {
                    mesh: meshes.add(Sphere {
                        radius: planet.data.radius,
                    }),
                    material: materials.add(StandardMaterial {
                        base_color: planet.data.color,
                        ..default()
                    }),
                    transform: Transform::from_translation(planet.data.position),
                    ..default()
                },
            },
            Object {},
            Planet {},
        ));
    }

    commands
        .spawn((
            ObjectBundle {
                body: sun,
                pbr: PbrBundle {
                    mesh: meshes.add(Sphere {
                        radius: sun.data.radius,
                    }),
                    material: materials.add(StandardMaterial {
                        base_color: Color::from(tailwind::YELLOW_500),
                        emissive: Color::from(tailwind::YELLOW_500).into(),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_translation(sun.data.position),
                    ..default()
                },
            },
            Object {},
            Star {},
        ))
        .with_children(|p| {
            p.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    color: Color::WHITE,
                    range: f32::MAX,
                    radius: sun.data.radius,
                    intensity: 1_000_000_000_000_000.0,
                    ..default()
                },
                ..default()
            });
        });
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
        for (transform_inner, body_inner) in &planet_data {
            if body_inner.data.name == body_outer.data.name {
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

            let delta = time.delta_seconds() * speed.0;

            body_outer.data.velocity += acceleration * delta; // Adjusted for simulation speed
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

            println!(
                "{:?}, velocity: {:?}",
                body_outer.data.name, body_outer.data.velocity
            );
        }
    }
}

use bevy::{
    color::{palettes::tailwind, Color},
    prelude::*,
    render::mesh::Mesh,
};

#[derive(Debug, Clone, Default)]
pub struct SpaceObjectData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub rotation: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub temperature: f32,
}

#[derive(Debug, Component, Clone, Default)]
pub struct Planet {
    pub name: &'static str,
    pub color: Color,
    pub data: SpaceObjectData,
}

impl Planet {
    pub fn new(name: &'static str, radius: f32, distance: f32, color: Color) -> Self {
        static mut PREVIOUS_POSITION: f32 = 0.0;
        let position;

        unsafe {
            position = Vec3::new(distance, 0.0, 0.0); // Adding 1.0 for spacing between planets
            PREVIOUS_POSITION = position.x + radius;
        }

        Self {
            name,
            color,
            data: SpaceObjectData {
                position,
                radius,
                ..default()
            },
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Star {
    pub name: &'static str,
    pub data: SpaceObjectData,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    planet: Planet,
    pbr: PbrBundle,
}

#[derive(Bundle)]
pub struct StarBundle {
    star: Star,
    pbr: PbrBundle,
}

pub const SUN: Star = Star {
    name: "Sun",
    data: SpaceObjectData {
        position: Vec3::new(0.0, 0.0, 0.0),
        velocity: Vec3::new(0.0, 0.0, 0.0),
        acceleration: Vec3::new(0.0, 0.0, 0.0),
        rotation: Vec3::new(0.0, 0.0, 0.0),
        mass: 332_950.0,
        radius: 696_300.0 / SCALE,
        temperature: 5772.0,
    },
};

const SCALE: f32 = 100.0;

fn scale(x: f32) -> f32 {
    x / SCALE
}

fn scale2(x: f32) -> f32 {
    x / (SCALE * 10.0)
}

pub fn planets_create_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = vec![
        Planet::new(
            "Mercury",
            scale(2_439.7),
            scale2(69_820_000.0),
            Color::from(tailwind::GRAY_500),
        ),
        Planet::new(
            "Venus",
            scale(6_051.8),
            scale2(108_940_000.0),
            Color::from(tailwind::AMBER_500),
        ),
        Planet::new(
            "Earth",
            scale(6_371.0),
            scale2(152_097_597.0),
            Color::from(tailwind::BLUE_500),
        ),
        Planet::new(
            "Mars",
            scale(3_389.5),
            scale2(249_261_000.0),
            Color::from(tailwind::RED_500),
        ),
        Planet::new(
            "Jupiter",
            scale(142_984.0),
            scale2(816_081_455.0),
            Color::from(tailwind::YELLOW_500),
        ),
        Planet::new(
            "Saturn",
            scale(58_232.0),
            scale2(1_514_500_000.0),
            Color::from(tailwind::YELLOW_300),
        ),
        Planet::new(
            "Uranus",
            scale(25_362.0),
            scale2(3_006_390_000.0),
            Color::from(tailwind::TEAL_500),
        ),
        Planet::new(
            "Neptune",
            scale(24_622.0),
            scale2(4_540_000_000.0),
            Color::from(tailwind::BLUE_700),
        ),
    ];

    for planet in planets {
        commands.spawn(PlanetBundle {
            planet: planet.clone(),
            pbr: PbrBundle {
                mesh: meshes.add(Sphere {
                    radius: planet.data.radius,
                }),
                material: materials.add(StandardMaterial {
                    base_color: planet.color,
                    ..default()
                }),
                transform: Transform::from_translation(planet.data.position),
                ..default()
            },
        });
    }

    commands
        .spawn(StarBundle {
            star: SUN.clone(),
            pbr: PbrBundle {
                mesh: meshes.add(Sphere {
                    radius: SUN.data.radius,
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::from(tailwind::YELLOW_500),
                    emissive: Color::from(tailwind::YELLOW_500).into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(SUN.data.position),
                ..default()
            },
        })
        .with_children(|p| {
            p.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    color: Color::WHITE,
                    range: f32::MAX,
                    radius: SUN.data.radius,
                    intensity: 1_000_000_000_000_000.0,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn planets_update_system(mut query: Query<(&mut Planet, &mut Transform)>, time: Res<Time>) {}

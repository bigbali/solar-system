use bevy::prelude::*;

use crate::{
    simulation::body::*, simulation::data::planets, simulation::player::Player,
    simulation::settings::SimulationParameters,
};

pub fn initialize_bodies_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    parameters: Res<SimulationParameters>,
    asset_server: Res<AssetServer>,
) {
    let sun_texture: Handle<Image> = asset_server.load("sun.jpg");
    let sun_radius = 0.00465047;

    let sun = commands
        .spawn((
            Body {
                data: BodyData {
                    position: Vec3::new(
                        0.00450250878464055477,
                        0.00076707642709100705,
                        0.00026605791776697764,
                    ),
                    velocity: Vec3::new(
                        -0.00000035174953607552,
                        0.00000517762640983341,
                        0.00000222910217891203,
                    ),
                    acceleration: Vec3::ZERO,
                    rotation: Vec3::new(0.0, 0.0, 0.0),
                    axial_tilt: Vec3::new(0.0, 0.0, 0.0),
                    mass: 1.0,
                    radius: sun_radius,
                    temperature: 5778.0,
                },
                metadata: BodyMetadata {
                    color: Color::linear_rgb(0.5, 0.5, 0.0),
                    name: Some("Sun"),
                    texture: Some(sun_texture.clone()),
                },
            },
            Mesh3d(meshes.add(Sphere {
                radius: sun_radius / parameters.unit_scale,
            })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive_texture: Some(sun_texture),
                emissive: LinearRgba::WHITE,
                ..default()
            })),
            Star {},
        ))
        .with_children(|p| {
            p.spawn(PointLight {
                shadows_enabled: true,
                color: Color::WHITE,
                range: f32::MAX,
                radius: sun_radius / parameters.unit_scale * 1.05,
                intensity: 1_000_000_000_000.0,
                shadow_depth_bias: 0.0,
                shadow_map_near_z: 0.0,
                shadow_normal_bias: 0.0,
                ..default()
            });
            p.spawn((
                bevy_mod_billboard::BillboardText::default(),
                bevy_mod_billboard::BillboardDepth(false),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_translation(Vec3::new(
                    0.0,
                    sun_radius / parameters.unit_scale * 2.0,
                    0.0,
                ))
                .with_scale(Vec3::splat(0.0001)),
            ))
            .with_child((
                TextSpan::new("Sun"),
                TextFont::default().with_font_size(60.0),
                TextColor::from(Color::WHITE),
            ));
        })
        .id();

    commands.insert_resource(Sun(sun));

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
                if planet.metadata.name.is_none() {
                    return;
                }

                parent
                    .spawn((
                        bevy_mod_billboard::BillboardText::default(),
                        bevy_mod_billboard::BillboardDepth(false),
                        TextLayout::new_with_justify(JustifyText::Left),
                        Transform::from_translation(Vec3::new(
                            0.0,
                            planet.data.radius + sun_radius / parameters.unit_scale,
                            0.0,
                        ))
                        .with_scale(Vec3::splat(0.0001)),
                    ))
                    .with_child((
                        TextSpan::new(planet.metadata.name.unwrap()),
                        TextFont::default().with_font_size(60.0),
                        TextColor::from(Color::WHITE),
                    ));
            });
    }
}

pub fn spawn_player_system(mut commands: Commands, parameters: Res<SimulationParameters>) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(
                0.00450250878464055477,
                0.00076707642709100705 * 10.0,
                0.00026605791776697764,
            ),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                bevy::core_pipeline::bloom::Bloom::NATURAL,
                Camera3d::default(),
                Camera {
                    hdr: true,
                    ..default()
                },
                PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    near: 0.000000001,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.00465047 / parameters.unit_scale)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                bevy_flycam::FlyCam,
            ));
        });
}

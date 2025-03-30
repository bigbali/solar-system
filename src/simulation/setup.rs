use bevy::{color, prelude::*, render::camera};

use crate::{
    material::saturn_rings::SaturnRingMaterial,
    simulation::{
        body::*, data::initialize_bodies, player::Player, settings::SimulationParameters,
    },
};

pub fn initialize_bodies_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut _ring_material: ResMut<Assets<SaturnRingMaterial>>,
    _parameters: Res<SimulationParameters>,
    asset_server: Res<AssetServer>,
) {
    let bodies = initialize_bodies(&asset_server);

    if let Some(bodies) = bodies {
        for body in bodies {
            if body.metadata.name.as_ref().is_some_and(|n| n == "Sun") {
                let sun = commands
                    .spawn((
                        body.clone(),
                        Mesh3d(meshes.add(Sphere {
                            radius: body.data.radius,
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: body.metadata.texture.clone(),
                            emissive_texture: body.metadata.texture.clone(),
                            emissive: LinearRgba::new(2.0, 1.0, 1.0, 1.0),
                            ..default()
                        })),
                        Transform::from_translation(body.data.position),
                        Star {},
                    ))
                    .with_children(|p| {
                        p.spawn(PointLight {
                            shadows_enabled: true,
                            color: Color::WHITE,
                            range: f32::MAX,
                            radius: body.data.radius * 1.05,
                            intensity: 1_000_000.0,
                            shadow_depth_bias: 0.0,
                            shadow_map_near_z: 0.0,
                            shadow_normal_bias: 0.0,
                            ..default()
                        });
                    })
                    .id();

                commands.insert_resource(Sun(sun));
            } else {
                let mut entity = commands.spawn((
                    body.clone(),
                    Mesh3d(meshes.add(Sphere {
                        radius: body.data.radius,
                    })),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: match &body.metadata.texture {
                            Some(_) => Color::WHITE,
                            None => body.metadata.color,
                        },
                        base_color_texture: body.metadata.texture.clone(),
                        ..default()
                    })),
                    Transform::from_translation(body.data.position),
                ));

                // Saturn's rings
                entity.with_children(|parent| {
                    if body.metadata.name == Some("Saturn".to_string()) {
                        parent.spawn((
                            // TODO too faint, should probably use shader as initially planned
                            Mesh3d(meshes.add(Mesh::from(Plane3d {
                                half_size: Vec2::new(
                                    body.data.radius * 5.0,
                                    body.data.radius * 5.0,
                                ),
                                ..Default::default()
                            }))),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(asset_server.load("saturn_rings.png")),
                                emissive_texture: Some(asset_server.load("saturn_rings.png")),
                                emissive: Color::WHITE.into(),
                                ..default()
                            })),
                        ));
                    }
                });

                insert_type_marker(&mut entity, &body);

                if let Some(satellites) = body.satellites {
                    for satellite in satellites {
                        let satellite = satellite.read().unwrap();

                        let mut satellite_entity = commands.spawn((
                            satellite.clone(),
                            Mesh3d(meshes.add(Sphere {
                                radius: satellite.data.radius,
                            })),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color: satellite.metadata.color,
                                base_color_texture: satellite.metadata.texture.clone(),
                                ..default()
                            })),
                            Transform::from_translation(satellite.data.position),
                        ));

                        insert_type_marker(&mut satellite_entity, &satellite);
                    }
                }
            }
        }
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
                Projection::Perspective(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    near: 1e-5,
                    ..default()
                }),
                Transform::from_xyz(0.0, 0.0, 0.00465047 / parameters.unit_scale)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                bevy_flycam::FlyCam,
            ));
        });
}

fn insert_type_marker(entity: &mut EntityCommands, body: &Body) {
    match body.metadata.body_type {
        BodyType::Star => {
            entity.insert(Star {});
        }
        BodyType::Planet => {
            entity.insert(Planet {});
        }
        BodyType::DwarfPlanet => {
            entity.insert(DwarfPlanet {});
        }
        BodyType::Moon => {
            entity.insert(Moon {});
        }
        BodyType::Other => {
            entity.insert(Other {});
        }
        _ => {
            entity.insert(Unknown {});
        }
    }
}

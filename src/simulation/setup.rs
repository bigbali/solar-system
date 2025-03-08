use bevy::prelude::*;

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
    mut ring_material: ResMut<Assets<SaturnRingMaterial>>,
    parameters: Res<SimulationParameters>,
    asset_server: Res<AssetServer>,
) {
    let bodies = initialize_bodies(&asset_server);

    let name_offset_au = 0.005;

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
                        p.spawn((
                            bevy_mod_billboard::BillboardText::default(),
                            bevy_mod_billboard::BillboardDepth(false),
                            TextLayout::new_with_justify(JustifyText::Left),
                            Transform::from_translation(Vec3::new(
                                0.0,
                                body.data.radius / 2.0 + name_offset_au,
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
                        base_color_texture: body.metadata.texture,
                        ..default()
                    })),
                    Transform::from_translation(body.data.position),
                ));

                entity.with_children(|parent| {
                    if body.metadata.name == Some("Saturn".to_string()) {
                        parent.spawn((
                            /// TODO too faint, should probably use shader as initially planned
                            Mesh3d(meshes.add(Mesh::from(Plane3d {
                                half_size: Vec2::new(
                                    body.data.radius * 5.0,
                                    body.data.radius * 5.0,
                                ),
                                ..Default::default()
                            }))),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(asset_server.load("saturn_rings.png")),
                                ..default()
                            })),
                        ));
                    }

                    if body.metadata.name.is_none() {
                        return;
                    }

                    parent
                        .spawn((
                            bevy_mod_billboard::BillboardText::default(),
                            bevy_mod_billboard::BillboardDepth(false),
                            TextLayout::new_with_justify(JustifyText::Left),
                            Transform::from_translation(Vec3::new(
                                0.0,
                                body.data.radius / 2.0 + name_offset_au,
                                0.0,
                            ))
                            .with_scale(Vec3::splat(0.0001)),
                        ))
                        .with_child((
                            TextSpan::new(body.metadata.name.unwrap()),
                            TextFont::default().with_font_size(60.0),
                            TextColor::from(Color::WHITE),
                        ));
                });

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

                if let Some(satellites) = body.satellites {
                    for satellite in satellites {
                        let satellite = satellite.read().unwrap();

                        let mut entity = commands.spawn((
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

                        entity.with_children(|parent| {
                            if satellite.metadata.name.is_none() {
                                return;
                            }

                            parent
                                .spawn((
                                    bevy_mod_billboard::BillboardText::default(),
                                    bevy_mod_billboard::BillboardDepth(false),
                                    TextLayout::new_with_justify(JustifyText::Left),
                                    Transform::from_translation(Vec3::new(
                                        0.0,
                                        body.data.radius / 2.0 + name_offset_au,
                                        0.0,
                                    ))
                                    .with_scale(Vec3::splat(0.0001)),
                                ))
                                .with_child((
                                    TextSpan::new(
                                        satellite
                                            .metadata
                                            .name
                                            .clone()
                                            .unwrap_or("<unknown>".to_string()),
                                    ),
                                    TextFont::default().with_font_size(60.0),
                                    TextColor::from(Color::WHITE),
                                ));
                        });

                        match satellite.metadata.body_type {
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

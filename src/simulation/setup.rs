use bevy::{color, prelude::*};
use bevy_ui_anchor::{AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor};

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
        let mut bodies_to_show_name_for: Vec<(Option<String>, Color, f32, Entity)> = Vec::new();

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

                bodies_to_show_name_for.push((
                    body.metadata.name.clone(),
                    body.metadata.color.clone(),
                    body.data.radius,
                    sun,
                ));

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
                                emissive_texture: Some(asset_server.load("saturn_rings.png")),
                                emissive: Color::WHITE.into(),
                                ..default()
                            })),
                        ));
                    }
                });

                insert_type_marker(&mut entity, &body);

                bodies_to_show_name_for.push((
                    body.metadata.name.clone(),
                    body.metadata.color.clone(),
                    body.data.radius,
                    entity.id(),
                ));

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

                        bodies_to_show_name_for.push((
                            satellite.metadata.name.clone(),
                            satellite.metadata.color.clone(),
                            satellite.data.radius,
                            satellite_entity.id(),
                        ));
                    }
                }
            }
        }

        for (name, color, radius, id) in bodies_to_show_name_for {
            create_body_name(&mut commands, name, color, radius, id);
        }
    }
}

#[derive(Component)]
pub struct CameraMarker {}

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
                CameraMarker {},
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

fn create_body_name(
    commands: &mut Commands,
    name: Option<String>,
    color: Color,
    radius: f32,
    id: Entity,
) {
    // TODO system: get ca,era distance from body, and use that distance to increase vertical offset
    const NAME_OFFSET_AU: f32 = 0.005;
    const BEVY_DEFAULT_FONT_SIZE: f32 = 20.0;

    commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            BorderColor(color),
            AnchorUiNode {
                target: AnchorTarget::Entity(id),
                offset: Some(Vec3::new(0.0, radius / 2.0 + NAME_OFFSET_AU, 0.0)),
                anchorwidth: HorizontalAnchor::Mid,
                anchorheight: VerticalAnchor::Mid,
            },
        ))
        .with_children(|node| {
            node.spawn((
                Text(name.clone().unwrap_or("<unknown>".to_string())),
                TextColor::from(Color::WHITE),
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

use std::{env, f32::consts::PI};

use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use body::{bodies, planets_update_system, Body};
use iyes_perf_ui::{
    prelude::{PerfUiEntryFPS, PerfUiPosition, PerfUiRoot},
    PerfUiPlugin,
};

use ui::data_window;

mod body;
mod planets;
mod ui;

#[derive(Resource)]
pub struct SimulationSpeedMultiplier(f32);

#[derive(Resource)]
pub struct Follow {
    entity: Option<Entity>,
    active: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(bevy_mod_imgui::ImguiPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, (setup, bodies, spawn_player))
        .add_systems(PostStartup, log_system)
        .add_systems(
            Update,
            (
                planets_update_system,
                planet_gizmos,
                data_window,
                follow_object,
            ),
        )
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 50.0,          // default: 12.0
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SimulationSpeedMultiplier(1.0))
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
        })
        .insert_resource(Follow {
            entity: None,
            active: false,
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::TopRight,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));
}

#[derive(Debug, Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Bloom::NATURAL,
                Camera3d::default(),
                Camera {
                    hdr: true,
                    ..default()
                },
                PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 696_300.0 / 999_999.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                FlyCam,
            ));
        });
}

fn log_system(bodies: Query<&Body>) {
    println!("Debug data");
    println!("===============================================================");

    for body in bodies.iter() {
        println!("{:?}", body);
    }
}

fn planet_gizmos(mut gizmos: Gizmos, query: Query<(&Body, &Transform)>) {
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        UVec2::splat(50),
        Vec2::new(1_000.0, 1_000.0),
        LinearRgba::new(0.0, 0.0, 1.0, 1.0),
    );

    for (body, transform) in query.iter() {
        gizmos.sphere(
            Isometry3d {
                rotation: Quat::IDENTITY,
                translation: transform.translation.into(),
            },
            body.data.radius * 100.0,
            body.metadata.color,
        );

        // force direction and intensity
        gizmos.arrow(
            transform.translation,
            transform.translation + body.data.velocity * 100000.0,
            body.metadata.color,
        );
    }
}

fn follow_object(
    mut camera: Query<&mut Transform, With<Camera>>,
    query: Query<(&Transform, &Body), Without<Camera>>,
    follow: Res<Follow>,
) {
    if follow.active && follow.entity.is_some() {
        if let Ok((transform, body)) = query.get(follow.entity.unwrap()) {
            camera.single_mut().translation = Vec3 {
                x: transform.translation.x + body.data.radius * 2.0,
                y: transform.translation.y,
                z: transform.translation.z,
            }
        }
    }
}

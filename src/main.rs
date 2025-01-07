use std::f32::consts::PI;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use iyes_perf_ui::{
    entries::PerfUiBundle,
    prelude::{PerfUiEntryFPS, PerfUiPosition, PerfUiRoot},
    PerfUiPlugin,
};
use planet::{planets_create_system, planets_update_system, Body};

use bevy_mod_imgui::prelude::*;

mod planet;
#[derive(Resource)]
pub struct SimulationSpeedMultiplier(f32);

#[derive(Resource)]
pub struct Follow {
    entity: Option<Entity>,
    active: bool,
}

#[derive(Resource)]
pub struct Sun(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(bevy_mod_imgui::ImguiPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, (setup, planets_create_system, spawn_player))
        .add_systems(PostStartup, log_system)
        .add_systems(
            Update,
            (planets_update_system, planet_gizmos, ui, follow_object),
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

fn ui(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), Without<Camera>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut camera_speed: ResMut<MovementSettings>,
    mut follow: ResMut<Follow>,
    mut speed: ResMut<SimulationSpeedMultiplier>,
    sun: Res<Sun>,
) {
    let ui = context.ui();
    let window = ui.window("Solar System");

    let sun = query.get(sun.0);

    let bevy_window = windows.single();

    window
        .size(
            [500.0, bevy_window.resolution.physical_height() as f32],
            imgui::Condition::Always,
        )
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Objects");
            ui.separator();

            let mut camera_transform = camera.single_mut();

            for (transform, body, entity) in query.iter() {
                if ui.button_with_size(body.data.name.unwrap_or("unnamed"), [250.0, 60.0]) {
                    camera_transform.translation = Vec3 {
                        x: transform.translation.x,
                        y: transform.translation.y,
                        z: transform.translation.z + body.data.radius * 2.0,
                    };

                    if follow.entity != Some(entity) {
                        follow.entity = Some(entity);
                    } else {
                        follow.entity = None;
                    }
                }

                ui.same_line();

                ui.group(|| {
                    if let Ok((sun_transform, _, _)) = sun {
                        ui.text(format!(
                            "d from sun {}",
                            transform.translation.distance(sun_transform.translation)
                        ));
                    }

                    ui.text(format!("velocity x {}", body.data.velocity.x));
                    ui.text(format!("velocity y {}", body.data.velocity.y));
                    ui.text(format!("velocity z {}", body.data.velocity.z));
                    ui.text(format!(
                        "mass: {}, radius:{}",
                        body.data.mass, body.data.radius
                    ));
                    ui.text(format!("rot: {}", transform.rotation));
                });
            }

            ui.checkbox("Follow Planet", &mut follow.active);

            ui.input_float("Camera Speed", &mut camera_speed.speed)
                .step(100.0)
                .build();

            ui.input_float("Simulation Speed", &mut speed.0)
                .step(0.1)
                .build();

            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));

            ui.text(format!(
                "Camera Position: (x: {:.1}, y: {:.1}, z: {:.1})",
                camera_transform.translation.x,
                camera_transform.translation.y,
                camera_transform.translation.z
            ));
        });
}

#[derive(Debug, Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true, // 1. HDR is required for bloom
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    // tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
                    transform: Transform::from_xyz(0.0, 0.0, 696_300.0 / 999_999.0)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                BloomSettings::NATURAL,
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
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(50),
        Vec2::new(1_000.0, 1_000.0),
        LinearRgba::BLUE,
    );

    for (body, transform) in query.iter() {
        gizmos.sphere(
            transform.translation,
            Quat::IDENTITY,
            body.data.radius * 100.0,
            body.data.color,
        );

        gizmos.arrow(
            transform.translation,
            transform.translation + body.data.velocity * 100000.0,
            Color::WHITE,
        );

        let forward = transform.rotation * Vec3::Z;

        // Define the start and end points of the line
        let start = transform.translation;
        let end = transform.translation + forward * 10.0;

        // let tilted_direction = tilt_quaternion * Vec3::Z;

        gizmos.line(start, end, Color::linear_rgb(200.0, 100.0, 20.0));
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

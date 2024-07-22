use std::f32::consts::PI;

use bevy::{color::palettes::tailwind, core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use planet::{planets_create_system, planets_update_system, Planet, Star, SUN};

use bevy_mod_imgui::prelude::*;

mod planet;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(bevy_mod_imgui::ImguiPlugin::default())
        .add_systems(Startup, (setup, planets_create_system, spawn_player))
        .add_systems(PostStartup, log_system)
        .add_systems(Update, (planets_update_system, planet_gizmos, ui))
        .init_gizmo_group::<MyRoundGizmos>()
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 500.0,         // default: 12.0
        })
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn gizmo_color_text(config: &LightGizmoConfigGroup) -> String {
    match config.color {
        LightGizmoColor::Manual(color) => format!("Manual {}", Srgba::from(color).to_hex()),
        LightGizmoColor::Varied => "Random from entity".to_owned(),
        LightGizmoColor::MatchLightColor => "Match light color".to_owned(),
        LightGizmoColor::ByLightType => {
            format!(
                "Point {}, Spot {}, Directional {}",
                Srgba::from(config.point_light_color).to_hex(),
                Srgba::from(config.spot_light_color).to_hex(),
                Srgba::from(config.directional_light_color).to_hex()
            )
        }
    }
}

fn ui(
    mut context: NonSendMut<ImguiContext>,
    query: Query<&Planet>,
    mut camera: Query<(&mut Camera, &mut Transform)>,
    mut camera_speed: ResMut<MovementSettings>,
) {
    let ui = context.ui();
    let window = ui.window("Solar System");

    window
        .size([300.0, 700.0], imgui::Condition::FirstUseEver)
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Planets");
            ui.separator();

            let mut camera_transform = camera.single_mut().1;

            for planet in query.iter() {
                if ui.button_with_size(planet.name, [290.0, 60.0]) {
                    camera_transform.translation = Vec3 {
                        x: planet.data.position.x,
                        y: planet.data.position.y,
                        z: planet.data.position.z + planet.data.radius * 2.0,
                    };
                }
            }

            ui.input_float("Camera Speed", &mut camera_speed.speed)
                .step(100.0)
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

#[derive(Component)]
struct GizmoColorText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let text_style = TextStyle::default();
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Gizmo color mode: ", text_style.clone()),
            TextSection::new(gizmo_color_text(light_config), text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
        GizmoColorText,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::BLACK,
        brightness: 0.0,
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
                    transform: Transform::from_xyz(0.0, 0.0, SUN.data.radius * 1.2)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                BloomSettings::NATURAL,
                FlyCam,
            ));
        });
}

fn log_system(
    planets: Query<&Planet>,
    stars: Query<&Star>,
    lights: Query<(&PointLight, &Transform)>,
) {
    println!("Debug data");

    for planet in planets.iter() {
        println!("{:?}", planet);
    }

    for star in stars.iter() {
        println!("{:?}", star);
    }

    for light in lights.iter() {
        println!("{:?}, {:?}", light.0, light.1);
    }
}

fn planet_gizmos(
    mut gizmos: Gizmos,
    // mut my_gizmos: Gizmos<MyRoundGizmos>,
    query: Query<&Planet>,
    // time: Res<Time>,
) {
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(500),
        Vec2::new(100_000.0, 100_000.0),
        LinearRgba::BLUE,
    );

    // println!("Creating gizmos");

    for planet in query.iter() {
        gizmos.sphere(
            planet.data.position,
            Quat::IDENTITY,
            planet.data.radius * 10.0,
            Color::from(tailwind::PINK_700),
        );
    }
}

use bevy::prelude::*;

use crate::simulation::body::{Body, BodyData, BodyMetadata, Planet};
pub struct SpawnBodyPlugin;

impl Plugin for SpawnBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_preview_system, spawn_system))
            .insert_resource(SpawnBodyPreview(None))
            .add_event::<CreateSpawnBodyPreviewEvent>()
            .add_event::<UpdateSpawnBodyPreviewEvent>()
            .add_event::<DeleteSpawnBodyPreviewEvent>()
            .add_event::<SpawnBodyEvent>();
    }
}

pub struct SpawnBody {
    pub body: crate::simulation::body::Body,
}

#[derive(Event, Debug, Clone)]
pub struct CreateSpawnBodyPreviewEvent {}

#[derive(Event, Debug, Clone)]
pub struct UpdateSpawnBodyPreviewEvent {}

#[derive(Event, Debug, Clone)]
pub struct DeleteSpawnBodyPreviewEvent {}

#[derive(Event, Debug, Clone)]
pub struct SpawnBodyEvent {}

#[derive(Resource, Debug, Clone)]
pub struct SpawnBodyPreview(pub Option<SpawnBodyPreviewData>);

impl SpawnBodyPreview {
    pub fn new() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpawnBodyPreviewData {
    pub velocity: f32,
    pub from: Vec3,
    pub to: Vec3,
}

pub fn create_preview_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut preview: ResMut<SpawnBodyPreview>,
    mut events: EventReader<CreateSpawnBodyPreviewEvent>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if events.is_empty() {
        return;
    };

    events.clear();

    preview.0 = Some(SpawnBodyPreviewData {
        velocity: 0.0001,
        from: camera.single().translation,
        to: camera.single().translation / 2.0,
    });
}

pub fn spawn_system(
    mut commands: Commands,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut events: EventReader<SpawnBodyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    c: Query<&Projection>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    let mut entity = commands.spawn((
        Body {
            data: BodyData {
                velocity: camera.single().forward().as_vec3() / 100.0,
                radius: 0.05,
                mass: 0.2,
                ..default()
            },
            metadata: BodyMetadata {
                name: Some("test body".to_string()),
                color: Color::linear_rgb(1.0, 0.1, 0.1),
                id: Some(u32::MAX),
                ..default()
            },
            satellites: None,
        },
        Mesh3d(meshes.add(Sphere { radius: 0.1 })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgb(1.0, 0.1, 0.1),
            emissive: LinearRgba::rgb(100.0, 0.0, 0.0),
            ..default()
        })),
        Transform::from_translation(camera.single().translation + camera.single().forward() * 0.01),
        Planet {},
    ));
}

/// If the player has opted to specify custom coordinates as the target for the spawned body, visualize the body's direction vector.
pub fn render_preview_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut preview: Res<SpawnBodyPreview>,
    mut camera: Query<&mut Transform, With<Camera>>,
    c: Query<&Projection>,
) {
    if let Some(p) = &preview.0 {
        // let start = camera.single().rotation.to_euler(EulerRot::YZX);
        // let end = camera.single().translation - p.to * 0.1;

        let forward = camera.single().forward().as_vec3();

        println!("{}", forward);

        // let pos = camera.single().translation + forward;
        let pos = camera.single().translation + camera.single().back() * -0.01;

        gizmos.sphere(
            Isometry3d {
                rotation: Quat::IDENTITY,
                translation: pos.into(),
            },
            0.01,
            Color::linear_rgb(1.0, 0.0, 0.0),
        );

        gizmos.arrow(
            pos + forward,
            pos + forward + forward,
            Color::linear_rgb(1.0, 1.0, 0.0),
        );
        // gizmos.arrow(
        //     camera.single().translation + camera.single().back() * -0.01,
        //     camera.single().translation + camera.single().back() * -0.01 + forward,
        //     Color::linear_rgb(1.0, 0.0, 1.0),
        // );
    }
}

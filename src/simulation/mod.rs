use bevy::prelude::*;
use bevy_ui_anchor::AnchorUiPlugin;
use setup::CameraMarker;

pub mod body;
pub mod data;
mod gizmo;
pub mod physics;
pub mod player;
pub mod settings;
mod setup;
pub mod trajectory;
mod util;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, settings::params_override_system)
            .add_systems(
                Startup,
                (setup::initialize_bodies_system, setup::spawn_player_system),
            )
            .add_systems(
                Update,
                (
                    body::follow_body_system,
                    gizmo::body_gizmo_system,
                    setup::update_body_name_system,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    physics::gravity_system,
                    trajectory::precalculate_trajectory_system,
                    settings::elapsed_time_update_system,
                ),
            )
            .insert_resource(settings::SimulationParameters::default())
            .insert_resource(settings::FollowBody::default())
            .insert_resource(settings::SelectedBody::default())
            .insert_resource(settings::ElapsedTime::default())
            .insert_resource(trajectory::Trajectories::default())
            .insert_resource(trajectory::CalculateTrajectory::default())
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(bevy_flycam::MovementSettings {
                sensitivity: 0.00012,
                speed: 1.0,
            })
            .add_plugins(AnchorUiPlugin::<CameraMarker>::new());
    }
}

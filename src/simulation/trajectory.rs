use bevy::prelude::*;

use super::{body::Body, settings::SimulationParameters};

#[derive(Debug)]
pub struct TrajectoryPosition {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Debug)]
pub struct Trajectory {
    pub positions: Vec<TrajectoryPosition>,
    pub color: Color,
}

#[derive(Resource, Default, Debug)]
pub struct Trajectories(pub Vec<Trajectory>);

#[derive(Resource)]
pub struct CalculateTrajectory {
    pub calculated: bool,
    pub steps: i32,
}

impl Default for CalculateTrajectory {
    fn default() -> Self {
        Self {
            calculated: false,
            steps: 100,
        }
    }
}

pub fn precalculate_trajectory_system(
    body_query: Query<(&Transform, &Body)>,
    trajectory: Res<CalculateTrajectory>,
    mut trajectories: ResMut<Trajectories>,
    parameters: Res<SimulationParameters>,
) {
    if !trajectory.calculated {
        return;
    }

    let planet_data_inner: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    let planet_data_outer: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    let mut planet_data_outer_copy = planet_data_outer.clone();

    for step in 0..trajectory.steps {
        let delta = /* time.delta_secs() *  */parameters.speed_multiplier / parameters.time_step /* * 1000.0 */;

        for (index, (transform_outer, body_outer)) in
            &mut planet_data_outer_copy.iter_mut().enumerate()
        {
            let start = transform_outer.translation.clone();

            for (transform_inner, body_inner) in &planet_data_inner {
                if body_inner.metadata.name == body_outer.metadata.name {
                    continue;
                }

                let distance_squared = transform_inner
                    .translation
                    .distance_squared(transform_outer.translation)
                    .max(parameters.softening_factor);

                let force_direction =
                    (transform_inner.translation - transform_outer.translation).normalize();

                let f = force_direction
                    * parameters.gravitational_constant
                    * body_outer.data.mass
                    * body_inner.data.mass
                    / distance_squared;

                let acceleration = f / body_outer.data.mass;

                body_outer.data.velocity += 0.5 * acceleration * delta / parameters.velocity_scale;
            }

            transform_outer.translation += body_outer.data.velocity * delta;

            for (transform_inner, body_inner) in &planet_data_inner {
                if body_inner.metadata.name == body_outer.metadata.name {
                    continue;
                }

                let distance_squared = transform_inner
                    .translation
                    .distance_squared(transform_outer.translation)
                    .max(parameters.softening_factor);

                let force_direction =
                    (transform_inner.translation - transform_outer.translation).normalize();

                let f = force_direction
                    * parameters.gravitational_constant
                    * body_outer.data.mass
                    * body_inner.data.mass
                    / distance_squared;

                let acceleration = f / body_outer.data.mass;

                body_outer.data.velocity += 0.5 * acceleration * delta / parameters.velocity_scale;
            }

            if index >= trajectories.0.len() {
                trajectories.0.push(Trajectory {
                    color: body_outer.metadata.color,
                    positions: vec![TrajectoryPosition {
                        start,
                        end: transform_outer.translation.clone(),
                    }],
                });
            } else {
                trajectories.0[index].positions.push(TrajectoryPosition {
                    start,
                    end: transform_outer.translation.clone(),
                });
            }
        }
    }
}

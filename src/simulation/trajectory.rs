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
    body_query: Query<(&Transform, &Body, Entity)>,
    trajectory: Res<CalculateTrajectory>,
    mut trajectories: ResMut<Trajectories>,
    parameters: Res<SimulationParameters>,
) {
    if !trajectory.calculated {
        return;
    }

    let bodies_inner: Vec<_> = body_query
        .iter()
        .map(|(t, p, e)| (t.clone(), p.clone(), e.clone()))
        .collect();

    let mut bodies_outer = bodies_inner.clone();

    for step in 0..trajectory.steps {
        for (index, (transform_outer, body_outer, entity_outer)) in
            bodies_outer.iter_mut().enumerate()
        {
            body_outer.data.acceleration = Vec3::ZERO;

            let start = transform_outer.translation.clone();

            for (transform_inner, body_inner, entity_inner) in &bodies_inner {
                if entity_inner == entity_outer {
                    continue;
                }

                let distance_squared = transform_inner
                    .translation
                    .distance_squared(transform_outer.translation)
                    .max(parameters.softening_factor);

                let force_direction =
                    (transform_inner.translation - transform_outer.translation).normalize();

                body_outer.data.acceleration += force_direction
                    * (parameters.gravitational_constant * body_inner.data.mass / distance_squared);
            }

            body_outer.data.velocity += body_outer.data.acceleration * parameters.time_step;
            transform_outer.translation += body_outer.data.velocity * parameters.time_step;

            // if there are less body trajectories than we need, add a new one,
            // otherwise push to existing
            if index >= trajectories.0.len() {
                trajectories.0.push(Trajectory {
                    color: body_outer.metadata.color,
                    positions: vec![TrajectoryPosition {
                        start,
                        end: transform_outer.translation,
                    }],
                });
            } else {
                trajectories.0[index].positions.push(TrajectoryPosition {
                    start,
                    end: transform_outer.translation,
                });
            }
        }
    }
}

pub fn trajectory_projection_system(
    body_query: Query<(&Transform, &Body, Entity)>,
    trajectory: Res<CalculateTrajectory>,
    mut trajectories: ResMut<Trajectories>,
    parameters: Res<SimulationParameters>,
) {
}

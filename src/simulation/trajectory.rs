use bevy::{prelude::*, transform};

use super::{
    body::Body,
    physics::compute_acceleration,
    settings::{Integrator, SimulationParameters, UPDATE_FREQUENCY},
};

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

#[derive(Resource, Debug, Clone)]
pub struct LiveTrajectoryPreview {
    pub values: Vec<LiveTrajectoryPreviewPositions>,
    pub steps: usize,
}

#[derive(Debug, Clone)]
pub struct LiveTrajectoryPreviewPositions {
    pub points: Vec<Vec3>,
    pub color: Color,
    pub body_id: Entity,
}

const TRAJECTORY_POINTS: usize = 120;
impl Default for LiveTrajectoryPreview {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            steps: TRAJECTORY_POINTS,
        }
    }
}

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

// TODO update, but UI first
// TODO rk4 and leapfrog so we can use bigger multipliers without losing as much accuracy = better performance
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

            // if there are less body trajectories than bodies, add a new one,
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

pub fn live_trajectory_projection_system(
    mut body_query: Query<(&Transform, &Body, Entity)>,
    mut t: ResMut<LiveTrajectoryPreview>,
    parameters: Res<SimulationParameters>,
) {
    let p = &parameters;

    let total_updates =
        (p.time_step.abs() * p.updates_per_step / UPDATE_FREQUENCY as f32).round() as i32;

    // We want to do multiple updates per step to improve accuracy.
    // Do note that this will run each physics update, thus (time_step / 60 * updates_per_step) times / second.
    // That would be 10 updates per frame with 10.0 updates_per_step and 60.0 time_step.
    // We also make sure that we run at least 1 update per frame.
    for _ in 0..total_updates.max(1) {
        match p.integrator {
            Integrator::Euler => live_projection_euler(body_query.reborrow(), p, &mut t),
            Integrator::Leapfrog => live_projection_euler(body_query.reborrow(), p, &mut t),
            Integrator::RK4 => live_projection_euler(body_query.reborrow(), p, &mut t),
        }
    }
}

fn live_projection_euler(
    body_query: Query<(&Transform, &Body, Entity)>,
    parameters: &Res<SimulationParameters>,
    trajectories: &mut ResMut<LiveTrajectoryPreview>,
) {
    let mut bodies_outer: Vec<_> = body_query
        .iter()
        .map(|(t, p, e)| (t.clone(), p.clone(), e.clone()))
        .collect();

    let multiplier = parameters.time_step / UPDATE_FREQUENCY as f32 / parameters.updates_per_step;
    let mut accelerations = vec![Vec3::ZERO; bodies_outer.len()];

    for step in 0..trajectories.steps {
        for (i, (transform_outer, _body_outer, entity_outer)) in bodies_outer.iter().enumerate() {
            accelerations[i] = compute_acceleration(
                transform_outer.translation,
                &bodies_outer,
                parameters,
                *entity_outer,
            );
        }

        for (i, (transform_outer, body_outer, entity_outer)) in bodies_outer.iter_mut().enumerate()
        {
            body_outer.data.acceleration = accelerations[i];
            body_outer.data.velocity += accelerations[i] * multiplier;
            transform_outer.translation += body_outer.data.velocity * multiplier;

            if let Some(value) = trajectories.values.get_mut(i) {
                value.points[step] = transform_outer.translation;
            } else {
                trajectories.values.push(LiveTrajectoryPreviewPositions {
                    points: vec![transform_outer.translation; TRAJECTORY_POINTS],
                    color: body_outer.metadata.color,
                    body_id: *entity_outer,
                });
            }
        }
    }
}

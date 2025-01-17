use std::{
    f32::{INFINITY, NAN},
    rc::Weak,
};

use bevy::prelude::*;

use crate::{
    body::{Body, G, SOFTENING_FACTOR},
    CalculateTrajectory, SimulationSpeedMultiplier,
};

#[derive(Debug)]
pub struct TrajectoryPosition {
    pub start: Vec3,
    pub end: Vec3,
    pub color: Color,
    pub name: &'static str,
}

// #[derive(Debug, Default)]
// pub struct TrajectoryPositionLL {
//     pub p: Vec3,
//     pub next: Option<Box<TrajectoryPositionLL>>,
//     pub color: Color,
// }

#[derive(Resource, Default, Debug)]
pub struct Trajectories(pub Vec<TrajectoryPosition>);

// #[derive(Resource, Default, Debug)]
// pub struct Trajectories(pub TrajectoryPositionLL);

pub fn calculate_trajectory(
    body_query: Query<(&Transform, &Body)>,
    trajectory: Res<CalculateTrajectory>,
    mut trajectories: ResMut<Trajectories>,
    mut gizmos: Gizmos,
    speed: Res<SimulationSpeedMultiplier>,
    time: Res<Time>,
) {
    if !trajectory.calculated {
        return;
    }

    println!("Calculating trajectory");

    let planet_data_inner: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    let mut planet_data_outer: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    let mut planet_data_outer_copy = planet_data_outer.clone();

    for step in 0..trajectory.steps {
        for (transform_outer, body_outer) in &mut planet_data_outer_copy {
            let start = transform_outer.translation.clone();
            // let end: Vec3 = Vec3::ZERO;

            for (transform_inner, body_inner) in &planet_data_inner {
                if body_inner.metadata.name == body_outer.metadata.name {
                    continue;
                }

                let distance_squared = transform_inner
                    .translation
                    .distance_squared(transform_outer.translation)
                    + SOFTENING_FACTOR;
                let force_direction =
                    (transform_inner.translation - transform_outer.translation).normalize();

                let f = force_direction * G * body_outer.data.mass * body_inner.data.mass
                    / distance_squared;

                let acceleration = f / body_outer.data.mass;

                let delta = time.delta_secs() * speed.0;

                body_outer.data.velocity += acceleration * delta;

                transform_outer.translation += body_outer.data.velocity * speed.0;

                assert_ne!(distance_squared, INFINITY);
                assert_ne!(force_direction, Vec3::INFINITY);
                assert_ne!(f, Vec3::INFINITY);
                assert_ne!(acceleration, Vec3::INFINITY);
                assert_ne!(distance_squared, NAN);
                assert_ne!(force_direction, Vec3::NAN);
                assert_ne!(f, Vec3::NAN);
                assert_ne!(acceleration, Vec3::NAN);
                assert_ne!(body_outer.data.velocity, Vec3::NAN);
                assert_ne!(transform_outer.translation, Vec3::NAN);
            }

            let c = step as f32 / trajectory.steps as f32;

            let color = Color::linear_rgba(c, 1.0 - c, 1.0 - c, 1.0);

            println!("{:?} {:?}", step, color);

            trajectories.0.push({
                TrajectoryPosition {
                    start,
                    end: transform_outer.translation.clone(),
                    color,
                    name: body_outer.metadata.name.unwrap_or("unnamed"),
                }
            });

            // if step == 0 {
            //     trajectories.0.p = transform_outer.translation;
            //     trajectories.0.color = color;
            //     trajectories.0.next = None;
            // } else {
            //     let abc = trajectories.0;

            //     while abc.next
            // }
        }
    }
}

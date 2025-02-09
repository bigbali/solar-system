use bevy::prelude::*;

use super::{
    body::Body,
    settings::{Integrator, SimulationParameters, UPDATE_FREQUENCY},
};

pub fn gravity_system(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: Res<SimulationParameters>,
) {
    let p = &parameters;

    let total_updates =
        (p.time_step.abs() * p.updates_per_step / UPDATE_FREQUENCY as f32).round() as i32;

    // We want to do multiple updates per step to improve accuracy.
    // Do note that this will run each physics update, thus (time_step / 60 * updates_per_step) times / second.
    // That would be 10 updates per frame with 10.0 updates_per_step and 60.0 time_scaling.
    // We also make sure that we run at least 1 update per frame.
    for _ in 0..total_updates.max(1) {
        match p.integrator {
            Integrator::Euler => euler(body_query.reborrow(), p),
            Integrator::Leapfrog => leapfrog(body_query.reborrow(), p),
            Integrator::RK4 => rk4(body_query.reborrow(), p),
        }
    }
}

pub fn compute_acceleration(
    translation: Vec3,
    bodies: &[(Transform, Body, Entity)],
    parameters: &SimulationParameters,
    entity: Entity,
) -> Vec3 {
    let mut acceleration = Vec3::ZERO;

    for (other_transform, other_body, other_entity) in bodies {
        if entity == *other_entity {
            continue;
        }

        let distance_squared = other_transform
            .translation
            .distance_squared(translation)
            .max(parameters.softening_factor);

        let force_direction = (other_transform.translation - translation).normalize();

        acceleration += force_direction
            * (parameters.gravitational_constant * other_body.data.mass / distance_squared);
    }

    acceleration
}

fn euler(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: &Res<SimulationParameters>,
) {
    let bodies: Vec<_> = body_query
        .iter()
        .map(|(t, p, e)| (t.clone(), p.clone(), e))
        .collect();

    // Make sure that we account for the total updates per frame.
    let multiplier = parameters.time_step / UPDATE_FREQUENCY as f32 / parameters.updates_per_step;

    for (mut transform_outer, mut body_outer, entity_outer) in body_query.iter_mut() {
        let acceleration = compute_acceleration(
            transform_outer.translation,
            &bodies,
            parameters,
            entity_outer,
        );

        body_outer.data.acceleration = acceleration;
        body_outer.data.velocity += acceleration * multiplier;
        transform_outer.translation += body_outer.data.velocity * multiplier;
    }
}

fn leapfrog(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: &Res<SimulationParameters>,
) {
    euler(body_query, parameters);
}

fn rk4(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: &Res<SimulationParameters>,
) {
    euler(body_query, parameters);
}

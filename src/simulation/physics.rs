use bevy::prelude::*;

use super::{
    body::Body,
    settings::{Integrator, SimulationParameters},
};

pub fn gravity_system(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: Res<SimulationParameters>,
) {
    let p = &parameters;

    // We want to do multiple updates per step to improve accuracy.
    // Do note that this will run each physics update, thus (60 * time_step * updates_per_step) times / second.
    // That would make it 360 updates per second with the default 0.1 updates_per_step.
    for _ in 0..(p.time_step * p.updates_per_step) as i32 {
        match p.integrator {
            Integrator::Euler => euler(body_query.reborrow(), p),
            Integrator::Leapfrog => leapfrog(body_query.reborrow(), p),
            Integrator::RK4 => rk4(body_query.reborrow(), p),
        }
    }
}

fn compute_acceleration(
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

    for (mut transform_outer, mut body_outer, entity_outer) in body_query.iter_mut() {
        let acceleration = compute_acceleration(
            transform_outer.translation,
            &bodies,
            parameters,
            entity_outer,
        );

        body_outer.data.acceleration = acceleration;
        body_outer.data.velocity += acceleration * parameters.updates_per_step;
        transform_outer.translation += body_outer.data.velocity * parameters.updates_per_step;
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

use bevy::prelude::*;

use super::{body::Body, settings::SimulationParameters};

pub fn gravity_system(
    mut body_query: Query<(&mut Transform, &mut Body, Entity)>,
    parameters: Res<SimulationParameters>,
) {
    let bodies: Vec<_> = body_query
        .iter()
        .map(|(t, p, e)| (t.clone(), p.clone(), e))
        .collect();

    for (mut transform_outer, mut body_outer, entity_outer) in body_query.iter_mut() {
        // transform_outer.rotate(body_outer.data.rotation);
        body_outer.data.acceleration = Vec3::ZERO;

        for (transform_inner, body_inner, entity_inner) in &bodies {
            if *entity_inner == entity_outer {
                continue;
            }

            let distance_squared = transform_inner
                .translation
                .distance_squared(transform_outer.translation)
                .max(parameters.softening_factor);

            let force_direction =
                (transform_inner.translation - transform_outer.translation).normalize();

            let acceleration = force_direction
                * (parameters.gravitational_constant * body_inner.data.mass / distance_squared);

            body_outer.data.acceleration += acceleration;
        }

        let acceleration = body_outer.data.acceleration;

        body_outer.data.velocity += acceleration * parameters.time_step;
        transform_outer.translation += body_outer.data.velocity * parameters.time_step;
    }
}

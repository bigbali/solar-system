use bevy::prelude::*;

use super::{body::Body, settings::SimulationParameters};

pub fn gravity_system(
    mut body_query: Query<(&mut Transform, &mut Body)>,
    parameters: Res<SimulationParameters>,
) {
    let planet_data: Vec<_> = body_query
        .iter()
        .map(|(t, p)| (t.clone(), p.clone()))
        .collect();

    // todo: should be separated into function

    let delta = /* time.delta_secs() *  */parameters.speed_multiplier / parameters.time_step;

    for (mut transform_outer, mut body_outer) in body_query.iter_mut() {
        transform_outer.rotate_x(body_outer.data.rotation.x);
        transform_outer.rotate_y(body_outer.data.rotation.y);
        transform_outer.rotate_z(body_outer.data.rotation.z);

        for (transform_inner, body_inner) in &planet_data {
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

            let acceleration = f;

            body_outer.data.velocity += /* 0.5 *  */acceleration * delta;
        }

        transform_outer.translation += body_outer.data.velocity * delta;

        // for (transform_inner, body_inner) in &planet_data {
        //     if body_inner.metadata.name == body_outer.metadata.name {
        //         continue;
        //     }

        //     let distance_squared = transform_inner
        //         .translation
        //         .distance_squared(transform_outer.translation)
        //         .max(SOFTENING_FACTOR);

        //     let force_direction =
        //         (transform_inner.translation - transform_outer.translation).normalize();

        //     let f = force_direction * G * body_outer.data.mass * body_inner.data.mass
        //         / distance_squared;

        //     let acceleration = f;

        //     body_outer.data.velocity += 0.5 * acceleration * delta;
        // }
    }
}

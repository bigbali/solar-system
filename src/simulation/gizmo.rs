use std::f32::consts::PI;

use bevy::prelude::*;

use super::{body::Body, trajectory::LiveTrajectoryPreview};

pub fn body_gizmo_system(
    mut gizmos: Gizmos,
    query: Query<(&Body, &Transform)>,
    trajectory: ResMut<super::trajectory::Trajectories>,
    t: Res<LiveTrajectoryPreview>,
) {
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        UVec2::splat(50),
        Vec2::new(1.0, 1.0),
        LinearRgba::new(0.15, 0.15, 0.15, 0.2),
    );

    for (body, transform) in query.iter() {
        gizmos.sphere(
            Isometry3d {
                rotation: Quat::IDENTITY,
                translation: transform.translation.into(),
            },
            body.data.radius * 1000.0,
            body.metadata.color,
        );

        // force direction and velocity
        gizmos.arrow(
            transform.translation,
            transform.translation + body.data.velocity * 10.0,
            body.metadata.color,
        );
    }

    for t in trajectory.0.iter() {
        let points: Vec<Vec3> = t.positions.iter().map(|p| p.end).collect();

        gizmos.linestrip(points, t.color);
    }

    for trajectory in t.values.iter() {
        gizmos.linestrip(trajectory.points.clone(), LinearRgba::RED);
    }
}

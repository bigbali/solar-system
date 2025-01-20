use std::f32::consts::PI;

use bevy::prelude::*;

use super::body::Body;

pub fn body_gizmo_system(
    mut gizmos: Gizmos,
    query: Query<(&Body, &Transform)>,
    trajectory: ResMut<super::trajectory::Trajectories>,
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

    // gizmos.curve_3d(
    //     points,
    //     (0..=100).map(|n| n as f32 / 100.0),
    //     Color::linear_rgb(1.0, 0.0, 0.0),
    // );

    // let a = Curve3d

    for t in trajectory.0.iter() {
        // gizmos.line(t.start, t.end, t.color);
        let points: Vec<Vec3> = t.positions.iter().map(|p| p.end).collect();

        // let curve = ConstantCurve::new(Interval::EVERYWHERE, points);

        // gizmos.curve_3d(
        //     curve,
        //     (0..=100).map(|n| n as f32 / 100.0),
        //     Color::linear_rgb(1.0, 0.0, 0.0),
        // );

        gizmos.linestrip(points, t.color);

        // let mut control_points: Vec<Vec3> = Vec::new();

        // for p in &t.positions {
        //     // Always add the end point
        //     control_points.push(p.end);
        // }

        // gizmos.curve_3d(
        //     control_points,
        //     (0..=100).map(|n| n as f32 / 100.0),
        //     Color::linear_rgb(1.0, 0.0, 0.0),
        // );
    }
    // gizmos.line(t.start, t.end, t.color);
    // if trajectory.0.len() > 0 {
    //     let points: Vec<Vec3> = trajectory.0[0].positions.iter().map(|p| p.end).collect();
    //     gizmos.linestrip(points, Color::linear_rgb(1.0, 0.0, 0.0));
    // }

    // let curve = ConstantCurve::new(Interval::EVERYWHERE, points);

    // gizmos.curve_3d(
    //     curve,
    //     (0..=100).map(|n| n as f32 / 100.0),
    //     Color::linear_rgb(1.0, 0.0, 0.0),
    // );

    // let mut control_points: Vec<Vec3> = Vec::new();

    // for p in &t.positions {
    //     // Always add the end point
    //     control_points.push(p.end);
    // }

    // gizmos.curve_3d(
    //     control_points,
    //     (0..=100).map(|n| n as f32 / 100.0),
    //     Color::linear_rgb(1.0, 0.0, 0.0),
    // );

    // let range = 20;
    // let frange = range as f32;
    // for n in (0..range).step_by(2) {
    //     let norm = n as f32 / frange;

    //     gizmos.line(
    //         Vec3::splat(n as f32),
    //         Vec3::splat((n + 1) as f32),
    //         Color::linear_rgb(norm, 1.0 - norm, 1.0 - norm),
    //         // Color::linear_rgba(1.0, 0.0, 0.0, 1.0),
    //     );

    //     gizmos.sphere(
    //         Isometry3d {
    //             rotation: Quat::IDENTITY,
    //             translation: Vec3::splat((n + 1) as f32).into(),
    //         },
    //         0.1,
    //         Color::linear_rgb(norm, 1.0 - norm, 1.0 - norm),
    //     );
    // }
}

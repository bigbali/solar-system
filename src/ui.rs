use bevy::prelude::*;
use bevy_flycam::MovementSettings;
use bevy_mod_imgui::ImguiContext;

use crate::{
    body::{Body, Sun},
    Follow, SimulationSpeedMultiplier,
};

pub fn data_window(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), Without<Camera>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut camera_speed: ResMut<MovementSettings>,
    mut follow: ResMut<Follow>,
    mut speed: ResMut<SimulationSpeedMultiplier>,
    sun: Res<Sun>,
) {
    let ui = context.ui();
    let window = ui.window("Solar System");

    let sun = query.get(sun.0);

    let bevy_window = windows.single();

    window
        .size(
            [500.0, bevy_window.resolution.physical_height() as f32],
            imgui::Condition::Always,
        )
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Objects");
            ui.separator();

            let mut camera_transform = camera.single_mut();

            for (transform, body, entity) in query.iter() {
                if ui.button_with_size(body.metadata.name.unwrap_or("unnamed"), [250.0, 60.0]) {
                    camera_transform.translation = Vec3 {
                        x: transform.translation.x,
                        y: transform.translation.y,
                        z: transform.translation.z + body.data.radius * 2.0,
                    };

                    if follow.entity != Some(entity) {
                        follow.entity = Some(entity);
                    } else {
                        follow.entity = None;
                    }
                }

                ui.same_line();

                ui.group(|| {
                    if let Ok((sun_transform, _, _)) = sun {
                        ui.text(format!(
                            "d from sun {}",
                            transform.translation.distance(sun_transform.translation)
                        ));
                    }

                    ui.text(format!("velocity x {}", body.data.velocity.x));
                    ui.text(format!("velocity y {}", body.data.velocity.y));
                    ui.text(format!("velocity z {}", body.data.velocity.z));
                    ui.text(format!(
                        "mass: {}, radius:{}",
                        body.data.mass, body.data.radius
                    ));
                    ui.text(format!("rot: {}", transform.rotation));
                });
            }

            ui.checkbox("Follow Planet", &mut follow.active);

            ui.input_float("Camera Speed", &mut camera_speed.speed)
                .step(100.0)
                .build();

            ui.input_float("Simulation Speed", &mut speed.0)
                .step(0.1)
                .build();

            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));

            ui.text(format!(
                "Camera Position: (x: {:.1}, y: {:.1}, z: {:.1})",
                camera_transform.translation.x,
                camera_transform.translation.y,
                camera_transform.translation.z
            ));
        });
}

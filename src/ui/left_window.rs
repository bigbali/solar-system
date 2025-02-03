use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::simulation::body::*;

use super::{apply_button_color, clear_button_color};

pub fn left_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), (Without<Camera>, With<Planet>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut camera_speed: ResMut<bevy_flycam::MovementSettings>,
    mut follow: ResMut<crate::simulation::settings::FollowBody>,
    mut trajectory: ResMut<crate::simulation::trajectory::CalculateTrajectory>,
    mut trajectories: ResMut<crate::simulation::trajectory::Trajectories>,
    mut selected_body: ResMut<crate::simulation::settings::SelectedBody>,
    mut parameters: ResMut<crate::simulation::settings::SimulationParameters>,
    mut elapsed_time: ResMut<crate::simulation::settings::ElapsedTime>,
    sun: Res<Sun>,
) {
    let bevy_window = windows.single();
    let sun = query.get(sun.0);

    let ui = context.ui();
    let action_window = ui.window("Actions");

    action_window
        .size(
            [
                (bevy_window.resolution.physical_width() / 4) as f32,
                bevy_window.resolution.physical_height() as f32,
            ],
            imgui::Condition::Always,
        )
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.separator();
            ui.text("Bodies");
            ui.separator();
            ui.dummy([0.0, 4.0]);

            let mut camera_transform = camera.single_mut();

            for (_, body, entity) in query.iter() {
                let color_stack = apply_button_color(ui, body.metadata.color.to_linear());

                let button_text = match follow.entity {
                    Some(u_entity)
                        if entity == u_entity
                            && follow.is_active
                            && body.metadata.name.is_some() =>
                    {
                        format!("{} (following)", body.metadata.name.clone().unwrap())
                    }
                    _ => body
                        .metadata
                        .name
                        .clone()
                        .unwrap_or("<unknown body>".to_string())
                        .to_string(),
                };

                if ui.button_with_size(button_text, [164.0, 48.0]) {
                    match selected_body.entity {
                        Some(selected_entity) => {
                            if entity != selected_entity {
                                selected_body.entity = Some(entity);
                            } else {
                                selected_body.entity = None;
                            }
                        }
                        None => {
                            selected_body.entity = Some(entity);
                        }
                    };
                }

                clear_button_color(color_stack);

                ui.dummy([0.0, 8.0]);
            }

            ui.group(|| {
                ui.separator();
                ui.text("Simulation Parameters");
                ui.separator();
                ui.dummy([0.0, 4.0]);

                ui.input_float(
                    "Gravitational Constant",
                    &mut parameters.gravitational_constant,
                )
                .step(1e-4)
                .build();
                ui.input_float("Time Scaling (days/second)", &mut parameters.time_step)
                    .step(1.0 / 60.0) // step by 1 day/second
                    .build();
                ui.input_float(
                    "Simulation Speed Multiplier",
                    &mut parameters.updates_per_step,
                )
                .step(1.0)
                .build();

                ui.dummy([0.0, 8.0]);
            });

            ui.group(|| {
                ui.separator();
                ui.text("Trajectory");
                ui.separator();
                ui.dummy([0.0, 4.0]);

                ui.input_int("Steps", &mut trajectory.steps)
                    .step(100)
                    .build();

                if ui.button_with_size("Calculate Trajectories", [200.0, 48.0]) {
                    trajectory.calculated = true;
                } else {
                    trajectory.calculated = false;
                }

                ui.same_line_with_spacing(0.0, 4.0);

                if ui.button_with_size("Clear Trajectories", [200.0, 48.0]) {
                    trajectories.0.clear();
                }

                ui.dummy([0.0, 8.0]);
            });

            ui.group(|| {
                ui.separator();
                ui.text("Selected Body");
                ui.separator();
                ui.dummy([0.0, 4.0]);

                match selected_body.entity {
                    Some(entity) => {
                        let selected_body = query.get(entity);

                        if let Ok((_, body, _)) = selected_body {
                            ui.text(format!(
                                "Selected: {:?}",
                                body.metadata
                                    .name
                                    .clone()
                                    .unwrap_or("<unknown>".to_string())
                            ));
                        }

                        if ui.button_with_size("Visit", [200.0, 48.0]) {
                            if let Ok((transform, body, _)) = selected_body {
                                camera_transform.translation = Vec3 {
                                    x: transform.translation.x,
                                    y: transform.translation.y,
                                    z: transform.translation.z
                                        + body.data.radius
                                        + match sun {
                                            Ok((_, b, _)) => b.data.radius,
                                            Err(_) => 0.2,
                                        },
                                };
                            }
                        }

                        if ui.button_with_size("Follow", [200.0, 48.0]) {
                            if follow.entity != Some(entity) {
                                follow.entity = Some(entity);
                            } else {
                                follow.entity = None;
                            }
                        }
                    }
                    None => {
                        ui.text("No body selected");
                    }
                }
                ui.dummy([0.0, 8.0]);
            });

            ui.checkbox("Follow Planet", &mut follow.is_active);

            ui.input_float("Camera Speed", &mut camera_speed.speed)
                .step(100.0)
                .build();

            ui.input_float("Simulation Speed", &mut parameters.updates_per_step)
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

            ui.text(format!("Time Passed: {:.2} days", elapsed_time.0));
        });
}

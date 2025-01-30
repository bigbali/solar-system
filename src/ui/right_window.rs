use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;
use imgui::CollapsingHeader;

use crate::simulation::body::*;

pub fn right_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), Without<Camera>>,
    trajectories: ResMut<crate::simulation::trajectory::Trajectories>,
    selected_body: ResMut<crate::simulation::settings::SelectedBody>,
    sun: Res<Sun>,
) {
    let bevy_window = windows.single();
    let sun = query.get(sun.0);

    let ui = context.ui();
    let information_window = ui.window("Information");

    information_window
        .size(
            [
                (bevy_window.resolution.physical_width() / 5) as f32,
                bevy_window.resolution.physical_height() as f32,
            ],
            imgui::Condition::Always,
        )
        .position(
            [
                (bevy_window.resolution.physical_width()
                    - bevy_window.resolution.physical_width() / 5) as f32,
                0.0,
            ],
            imgui::Condition::Always,
        )
        .build(|| {
            if CollapsingHeader::new("Bodies").build(ui) {
                ui.group(|| {
                    for (transform, body, entity) in query.iter() {
                        let color = body.metadata.color.to_linear();
                        let token1 = ui.push_style_color(
                            imgui::StyleColor::Header,
                            [color.red, color.green, color.blue, color.alpha],
                        );
                        let token2 = ui.push_style_color(
                            imgui::StyleColor::HeaderActive,
                            [color.red, color.green, color.blue, color.alpha],
                        );

                        ui.dummy([0.0, 0.0]);
                        ui.same_line_with_spacing(0.0, 8.0);

                        if CollapsingHeader::new(format!(
                            "{}: {} {}",
                            body.metadata
                                .name
                                .clone()
                                .unwrap_or("<unknown body>".to_string()),
                            entity,
                            match selected_body.entity {
                                Some(se) =>
                                    if se == entity {
                                        "(selected)"
                                    } else {
                                        ""
                                    },
                                None => "",
                            }
                        ))
                        .build(ui)
                        {
                            ui.text(format!("{:#?}", transform));
                            ui.text(format!("{:#?}", body));

                            if let Ok((sun_transform, _, _)) = sun {
                                ui.text(format!(
                                    "Distance from Sun {} AU",
                                    transform.translation.distance(sun_transform.translation)
                                ));
                            }
                        }

                        token1.pop();
                        token2.pop();
                    }
                });
            }
            if CollapsingHeader::new("Raw Trajectory Data").build(ui) {
                ui.text(format!("{:#?}", trajectories));
            }
        });
}

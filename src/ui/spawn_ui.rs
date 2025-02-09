use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::{
    simulation::body::*,
    spawn::{CreateSpawnBodyPreviewEvent, SpawnBodyEvent, SpawnBodyPreview},
};

use super::{apply_button_color, clear_button_color};

pub fn spawn_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), (Without<Camera>, With<Planet>)>,
    mut e: EventWriter<CreateSpawnBodyPreviewEvent>,
    mut e2: EventWriter<SpawnBodyEvent>,
    mut s: ResMut<SpawnBodyPreview>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let bevy_window = windows.single();

    let ui = context.ui();
    let spawn_window = ui.window("Spawn");

    let w = bevy_window.resolution.width() / 6.0;
    let h = bevy_window.resolution.height() / 9.0;

    spawn_window
        .size([w, h], imgui::Condition::Always)
        .position([w * 3.0 - (w / 2.0), h * 9.0 - h], imgui::Condition::Always)
        .build(|| {
            ui.separator();
            ui.text("Spawn");
            ui.separator();
            ui.dummy([0.0, 4.0]);

            if ui.button_with_size("preview", [164.0, 48.0]) {
                e.send(CreateSpawnBodyPreviewEvent {});
            }

            ui.same_line_with_spacing(0.0, 16.0);

            if ui.button_with_size("spawn", [164.0, 48.0]) {
                e2.send(SpawnBodyEvent {});
            }

            if let Some(p) = &mut s.0 {
                ui.input_float("x", &mut p.from.x)
                    .step(1.0 / 60.0) // step by 1 day/second
                    .build();
                ui.input_float("y", &mut p.from.y)
                    .step(1.0 / 60.0) // step by 1 day/second
                    .build();
                ui.input_float("z", &mut p.from.z)
                    .step(1.0 / 60.0) // step by 1 day/second
                    .build();
            }
        });
}

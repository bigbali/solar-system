use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

pub fn test_window_system(mut context: NonSendMut<ImguiContext>, windows: Query<&Window>) {
    let bevy_window = windows.single();

    let ui = context.ui();
    let window = ui.window("##Test Window");

    let size = [
        bevy_window.resolution.physical_width() as f32,
        bevy_window.resolution.physical_height() as f32,
    ];
}

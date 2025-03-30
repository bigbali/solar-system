use bevy::prelude::*;

use material::saturn_rings::SaturnRingMaterial;
use simulation::SimulationPlugin;
use spawn::SpawnBodyPlugin;
use ui::SimulationUiPlugin;

mod material;
mod simulation;
mod spawn;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_flycam::NoCameraPlayerPlugin)
        .add_plugins(bevy_mod_imgui::ImguiPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(iyes_perf_ui::PerfUiPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(SimulationUiPlugin)
        .add_plugins(SpawnBodyPlugin)
        .add_plugins(MaterialPlugin::<SaturnRingMaterial>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
        })
        .run();
}

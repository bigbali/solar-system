use bevy::prelude::*;
// use flex::FlexAxisAlign;
use left_window::left_window_system;
use right_window::right_window_system;
use spawn_ui::spawn_window_system;
use util::{active, hover, rgba};

mod element;
// mod flex;
mod left_window;
mod right_window;
mod spawn_ui;
mod util;
mod window;

pub struct SimulationUiPlugin;

impl Plugin for SimulationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, performance_metrics_system)
            .add_systems(
                Update,
                (left_window_system, right_window_system, spawn_window_system),
            );
    }
}

fn performance_metrics_system(mut commands: Commands) {
    commands.spawn((
        iyes_perf_ui::prelude::PerfUiRoot {
            position: iyes_perf_ui::prelude::PerfUiPosition::TopRight,
            ..default()
        },
        iyes_perf_ui::prelude::PerfUiEntryFPS::default(),
    ));
}

// TODO util
pub fn apply_button_color<'a>(
    ui: &'a imgui::Ui,
    color: LinearRgba,
) -> Vec<imgui::ColorStackToken<'a>> {
    let mut color_stack: Vec<imgui::ColorStackToken> = Vec::new();

    let color_hover = color.mix(&LinearRgba::BLACK, 0.1);
    let color_active = color.mix(&LinearRgba::BLACK, 0.2);

    color_stack.push(ui.push_style_color(
        imgui::StyleColor::Button,
        [color.red, color.green, color.blue, color.alpha],
    ));

    color_stack.push(ui.push_style_color(
        imgui::StyleColor::ButtonHovered,
        [
            color_hover.red,
            color_hover.green,
            color_hover.blue,
            color_hover.alpha,
        ],
    ));

    color_stack.push(ui.push_style_color(
        imgui::StyleColor::ButtonActive,
        [
            color_active.red,
            color_active.green,
            color_active.blue,
            color_active.alpha,
        ],
    ));

    return color_stack;
}

// TODO util
pub fn clear_button_color(stack: Vec<imgui::ColorStackToken>) {
    for color in stack {
        color.pop();
    }
}

#[non_exhaustive]
pub struct Color;

impl Color {
    pub const Text: [f32; 4] = rgba([255.0, 255.0, 255.0, 1.0]);
    pub const Background: [f32; 4] = rgba([13.0, 13.0, 13.0, 1.0]);
    pub const Button: [f32; 4] = rgba([26.0, 26.0, 26.0, 1.0]);
    pub const ButtonHover: [f32; 4] = hover(Self::Button);
    pub const ButtonActive: [f32; 4] = active(Self::Button);
    pub const Input: [f32; 4] = Self::Button;
    pub const InputBorder: [f32; 4] = rgba([38.0, 38.0, 38.0, 1.0]);
    pub const Border: [f32; 4] = rgba([26.0, 26.0, 26.0, 1.0]);
}

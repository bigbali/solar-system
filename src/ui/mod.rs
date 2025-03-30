use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use imgui::ImColor32;
use mint::Vector4;
use name_tag::{name_tag_setup_system, name_tag_update_system};
use util::{active, hover, rgba};
use window::{spawn_window::spawn_window_system, test_window::test_window_system};

pub mod element;
pub mod name_tag;
// mod flex;
mod left_window;
mod right_window;
// mod spawn_ui;
mod util;
mod window;

pub struct SimulationUiPlugin;

impl Plugin for SimulationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, performance_metrics_system)
            .add_systems(PostStartup, name_tag_setup_system)
            .add_systems(
                Update,
                (
                    // left_window_system,
                    // right_window_system,
                    spawn_window_system,
                    test_window_system,
                    name_tag_update_system,
                ),
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
pub struct DefaultColor;

impl DefaultColor {
    pub const TEXT: [f32; 4] = rgba([255.0, 255.0, 255.0, 1.0]);
    pub const BACKGROUND: [f32; 4] = rgba([13.0, 13.0, 13.0, 1.0]);
    pub const BUTTON: [f32; 4] = rgba([26.0, 26.0, 26.0, 1.0]);
    pub const BUTTON_HOVER: [f32; 4] = hover(Self::BUTTON);
    pub const BUTTON_ACTIVE: [f32; 4] = active(Self::BUTTON);
    pub const INPUT: [f32; 4] = Self::BUTTON;
    pub const INPUT_BORDER: [f32; 4] = rgba([38.0, 38.0, 38.0, 1.0]);
    pub const BORDER: [f32; 4] = rgba([26.0, 26.0, 26.0, 1.0]);
}

#[derive(Debug, Clone, Copy)]
pub struct UiColor(Color);

impl UiColor {
    pub const fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Default for UiColor {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

impl From<Color> for UiColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<LinearRgba> for UiColor {
    fn from(color: LinearRgba) -> Self {
        Self(Color::from(color))
    }
}

impl Into<LinearRgba> for UiColor {
    fn into(self) -> LinearRgba {
        self.to_linear()
    }
}

impl Into<Vector4<f32>> for UiColor {
    fn into(self) -> Vector4<f32> {
        let c = self.to_linear();

        Vector4 {
            x: c.red,
            y: c.green,
            z: c.blue,
            w: c.alpha,
        }
    }
}

impl Deref for UiColor {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UiColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<ImColor32> for UiColor {
    fn into(self) -> ImColor32 {
        let c = self.to_linear().to_f32_array();
        ImColor32::from_rgba_f32s(c[0], c[1], c[2], c[3])
    }
}

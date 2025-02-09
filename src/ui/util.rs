use bevy::winit::cursor;
use imgui::*;
use mint;

use super::Color;

static mut IMGUI_INCR_ID: u32 = 0;

fn id() -> String {
    unsafe {
        IMGUI_INCR_ID += 1;
        format!("##IMGUI_GROUP_{}", IMGUI_INCR_ID)
    }
}

pub fn with_color_scheme(ui: &imgui::Ui, f: impl FnOnce()) {
    let mut color_stack = Vec::new();

    color_stack.push(ui.push_style_color(StyleColor::WindowBg, Color::Background));
    color_stack.push(ui.push_style_color(StyleColor::Button, Color::Button));
    color_stack.push(ui.push_style_color(StyleColor::ButtonHovered, Color::ButtonHover));
    color_stack.push(ui.push_style_color(StyleColor::ButtonActive, Color::ButtonActive));
    color_stack.push(ui.push_style_color(StyleColor::Border, Color::Border));
    color_stack.push(ui.push_style_color(StyleColor::Text, Color::Text));
    color_stack.push(ui.push_style_color(StyleColor::FrameBg, Color::Input));

    f();

    for color in color_stack {
        color.pop();
    }
}

pub const fn hover(color: [f32; 4]) -> [f32; 4] {
    [color[0] * 1.1, color[1] * 1.1, color[2] * 1.1, color[3]]
}

pub const fn active(color: [f32; 4]) -> [f32; 4] {
    [color[0] * 1.2, color[1] * 1.2, color[2] * 1.2, color[3]]
}

pub const fn rgba(c: [f32; 4]) -> [f32; 4] {
    [c[0] / 255.0, c[1] / 255.0, c[2] / 255.0, c[3]]
}

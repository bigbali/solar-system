use std::sync::atomic::{AtomicU32, Ordering};

use bevy::{color::LinearRgba, winit::cursor};
use imgui::*;
use mint;

use super::DefaultColor;

static INCR_ID: AtomicU32 = AtomicU32::new(0);

pub fn id() -> String {
    format!("##{}", INCR_ID.fetch_add(1, Ordering::SeqCst).to_string())
}

pub fn with_color_scheme(ui: &imgui::Ui, f: impl FnOnce()) {
    let mut color_stack = Vec::new();

    color_stack.push(ui.push_style_color(StyleColor::WindowBg, DefaultColor::Background));
    color_stack.push(ui.push_style_color(StyleColor::Button, DefaultColor::Button));
    color_stack.push(ui.push_style_color(StyleColor::ButtonHovered, DefaultColor::ButtonHover));
    color_stack.push(ui.push_style_color(StyleColor::ButtonActive, DefaultColor::ButtonActive));
    color_stack.push(ui.push_style_color(StyleColor::Border, DefaultColor::Border));
    color_stack.push(ui.push_style_color(StyleColor::Text, DefaultColor::Text));
    color_stack.push(ui.push_style_color(StyleColor::FrameBg, DefaultColor::Input));

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

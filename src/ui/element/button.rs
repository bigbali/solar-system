use std::{fmt, sync::Arc};

use bevy::color::LinearRgba;

use crate::ui::{apply_button_color, clear_button_color, UiColor};

use super::{Border, Override, UiNode};

#[derive(Debug, Clone)]
pub struct Button {
    pub width: f32,
    pub height: f32,
    pub border: Border,
    pub background: UiColor,
    pub label: String,
    pub on_click: OnClickCallback,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            width: 120.0,
            height: 48.0,
            border: Border {
                size: 0.0,
                color: UiColor::from(LinearRgba::BLACK),
            },
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_click: OnClickCallback(None),
        }
    }
}

impl UiNode for Button {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_border(&self) -> Border {
        self.border
    }

    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        cascading_override: Override,
    ) {
        let width = cascading_override.width.unwrap_or(self.width);
        let height = cascading_override.width.unwrap_or(self.height);

        let cursor = context.cursor_screen_pos();

        let color_stack = apply_button_color(context, self.background.into());

        let a = context.push_style_var(imgui::StyleVar::FrameBorderSize(self.border.size));
        let b = context.push_style_color(imgui::StyleColor::Border, self.border.color);

        if context.button_with_size(self.label.clone(), [width, height]) {
            if let Some(callback) = &self.on_click.0 {
                callback();
            }
        }

        clear_button_color(color_stack);

        a.pop();
        b.pop();

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

pub trait ButtonChild {
    fn button(&mut self, button: Button) -> &mut Button;
}

#[derive(Clone)]
pub struct OnClickCallback(pub Option<Arc<dyn Fn()>>);

impl fmt::Debug for OnClickCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OnClick Closure")
    }
}

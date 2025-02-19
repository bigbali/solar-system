use bevy::color::LinearRgba;

use crate::ui::{apply_button_color, clear_button_color};

use super::{Override, UiNode};

pub struct Button {
    pub width: f32,
    pub height: f32,
    pub border: f32,
    pub background: LinearRgba,
    pub label: String,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            width: 120.0,
            height: 48.0,
            border: 0.0,
            background: LinearRgba::BLACK,
            label: "Button".to_string(),
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

    fn get_border(&self) -> f32 {
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

        let color_stack = apply_button_color(context, self.background);

        context.button_with_size(self.label.clone(), [width, height]);

        clear_button_color(color_stack);

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

pub trait ButtonChild {
    fn button(&mut self, button: Button) -> &mut Button;
}

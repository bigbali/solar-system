use bevy::color::LinearRgba;

use crate::ui::{apply_button_color, clear_button_color, UiColor};

use super::{Border, Override, UiNode};

pub struct Button {
    pub width: f32,
    pub height: f32,
    pub border: Border,
    pub background: UiColor,
    pub label: String,
    pub on_click: Option<Box<dyn Fn()>>,
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
            on_click: None,
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
            self.on_click.as_ref().unwrap()();
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

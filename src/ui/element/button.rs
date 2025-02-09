use std::{fmt, sync::Arc};

use bevy::color::LinearRgba;
use imgui::{ColorStackToken, StyleStackToken};

use crate::ui::{apply_button_color, clear_button_color, UiColor};

use super::{Border, Computed, ParentProperties, Size, UiElement, UiElementType, UiNode};

#[derive(Debug, Clone)]
pub struct Button {
    pub width: Size,
    pub height: Size,
    pub border: Option<Border>,
    pub background: UiColor,
    pub label: String,
    pub on_click: OnClickCallback,
    pub computed_width: Option<f32>,
    pub computed_height: Option<f32>,
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            width: Size::Pixels(120.0),
            height: Size::Pixels(120.0),
            border: None,
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_click: OnClickCallback(None),
            computed_width: None,
            computed_height: None,
        }
    }
}

impl Computed for Button {
    fn get_computed_width(&self) -> Option<f32> {
        self.computed_width
    }

    fn set_computed_width(&mut self, new_width: f32) {
        self.computed_width = Some(new_width);
    }

    fn get_computed_height(&self) -> Option<f32> {
        self.computed_height
    }

    fn set_computed_height(&mut self, new_height: f32) {
        self.computed_height = Some(new_height);
    }

    // Button can't have children,
    fn compute_children_size(&mut self, _parent_properties: &ParentProperties) {
        return;
    }
}

impl UiNode for Button {
    fn get_width(&self) -> &Size {
        &self.width
    }

    fn get_height(&self) -> &Size {
        &self.height
    }

    fn get_border(&self) -> Option<Border> {
        self.border
    }

    fn get_children(&self) -> Option<&Vec<UiElement>> {
        None
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>> {
        None
    }

    fn get_type(&self) -> UiElementType {
        UiElementType::Button
    }

    fn build(
        &self,
        context: &imgui::Ui,
        _draw_list: &imgui::DrawListMut,
        // cascading_override: Override,
    ) {
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        let cursor = context.cursor_screen_pos();

        let button_color_stack = apply_button_color(context, self.background.into());

        let mut style_stack: Vec<StyleStackToken> = Vec::new();
        let mut color_stack: Vec<ColorStackToken> = Vec::new();

        if self.border.is_some() {
            style_stack.push(
                context.push_style_var(imgui::StyleVar::FrameBorderSize(self.border.unwrap().size)),
            );

            color_stack.push(
                context.push_style_color(imgui::StyleColor::Border, self.border.unwrap().color),
            );
        }

        if context.button_with_size(self.label.clone(), [width, height]) {
            if let Some(callback) = &self.on_click.0 {
                callback();
            }
        }

        clear_button_color(button_color_stack);

        if self.border.is_some() {
            for t in style_stack {
                t.pop();
            }

            for t in color_stack {
                t.pop();
            }
        }

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

pub trait ButtonChild {
    fn button(&mut self) -> &mut Button;
}

#[derive(Clone)]
pub struct OnClickCallback(pub Option<Arc<dyn Fn()>>);

impl fmt::Debug for OnClickCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OnClick Closure")
    }
}

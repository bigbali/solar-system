use std::{fmt, sync::Arc};

use bevy::{
    color::{Alpha, LinearRgba},
    text::{self, cosmic_text::Color},
};

use crate::ui::{apply_button_color, clear_button_color, element::rect::Rect, UiColor};

use super::{
    flex::{FlexAxisAlign, FlexCrossAxisAlign},
    Border, Computed, ParentProperties, Size, UiElement, UiElementType, UiNode,
};

// NOTE: this seems like a place where Size::FitContent would be great

#[derive(Debug, Clone)]
pub struct Text {
    width: Size,
    height: Size,
    align_x: FlexCrossAxisAlign,
    align_y: FlexCrossAxisAlign,
    background: UiColor,
    foreground: UiColor,
    padding: f32,
    text: String,
    computed_width: Option<f32>,
    computed_height: Option<f32>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self::default().text(text.to_string()).to_owned()
    }

    pub fn text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    pub fn width(&mut self, v: Size) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: Size) -> &mut Self {
        self.height = v;
        self
    }

    pub fn align_x(&mut self, v: FlexCrossAxisAlign) -> &mut Self {
        self.align_x = v;
        self
    }

    pub fn align_y(&mut self, v: FlexCrossAxisAlign) -> &mut Self {
        self.align_y = v;
        self
    }

    pub fn background(&mut self, v: UiColor) -> &mut Self {
        self.background = v;
        self
    }

    pub fn foreground(&mut self, v: UiColor) -> &mut Self {
        self.foreground = v;
        self
    }

    pub fn padding(&mut self, v: f32) -> &mut Self {
        self.padding = v;
        self
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            align_x: FlexCrossAxisAlign::Start,
            align_y: FlexCrossAxisAlign::Center,
            background: UiColor::from(LinearRgba::BLACK.with_alpha(0.0)),
            foreground: UiColor::from(LinearRgba::WHITE),
            padding: 0.0,
            text: "Text".to_string(),
            computed_width: None,
            computed_height: None,
        }
    }
}

impl Computed for Text {
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

    fn compute_children_size(&mut self, _parent_properties: &ParentProperties) {
        return;
    }
}

impl UiNode for Text {
    fn get_width(&self) -> &Size {
        &self.width
    }

    fn get_height(&self) -> &Size {
        &self.height
    }

    fn get_border(&self) -> Option<Border> {
        None
    }

    fn get_children(&self) -> Option<&Vec<UiElement>> {
        None
    }

    fn get_type(&self) -> UiElementType {
        UiElementType::Button
    }

    fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut) {
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        // Rect::new(Size::Pixels(width), Size::Pixels(height))
        //     .background(self.background)
        //     .build(context, draw_list);

        let text_size = context.calc_text_size(&self.text);

        let text_width = text_size[0];
        let text_height = text_size[1];

        let cursor = context.cursor_screen_pos();

        Rect::draw(
            context,
            draw_list,
            cursor,
            [cursor[0] + width, cursor[1] + height],
            Some(self.background),
            None,
        );

        let offset_x = match self.align_x {
            FlexCrossAxisAlign::Start => 0.0,
            FlexCrossAxisAlign::Center => width / 2.0 - text_width / 2.0,
            FlexCrossAxisAlign::End => width - text_width,
        };

        let offset_y = match self.align_y {
            FlexCrossAxisAlign::Start => 0.0,
            FlexCrossAxisAlign::Center => height / 2.0 - text_height / 2.0,
            FlexCrossAxisAlign::End => height - text_height,
        };

        context.set_cursor_screen_pos([cursor[0] + offset_x, cursor[1] + offset_y]);

        // Draw background
        // Padding
        // Border
        // Font size

        context.text_colored(self.foreground, &self.text);
        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

pub trait TextChild {
    fn text(&mut self, text: &str) -> &mut Text;
}

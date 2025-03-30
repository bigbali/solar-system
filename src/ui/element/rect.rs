use bevy::prelude::*;

use crate::ui::UiColor;

use super::{Border, Computed, ParentProperties, Size, UiElement, UiElementType, UiNode};

#[derive(Debug, Clone)]
pub struct Rect {
    width: Size,
    height: Size,
    background: Option<UiColor>,
    border: Option<Border>,
    computed_width: Option<f32>,
    computed_height: Option<f32>,
}

impl Rect {
    pub fn new(width: Size, height: Size) -> Self {
        Self {
            width,
            height,
            ..Self::default()
        }
    }

    pub fn width(&mut self, v: Size) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: Size) -> &mut Self {
        self.height = v;
        self
    }

    pub fn background(&mut self, v: UiColor) -> &mut Self {
        self.background = Some(v);
        self
    }

    pub fn border(&mut self, v: Border) -> &mut Self {
        self.border = Some(v);
        self
    }

    pub fn draw(
        _context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        from: [f32; 2],
        to: [f32; 2],
        color: Option<UiColor>,
        border: Option<Border>,
    ) {
        if let Some(c) = color {
            draw_list.add_rect(from, to, c).filled(true).build();
        }

        if let Some(b) = border {
            draw_list
                .add_rect(from, to, b.color)
                .thickness(b.size)
                .build();
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            background: Some(UiColor::from(LinearRgba::BLACK.with_alpha(0.0))),
            border: Some(Border::default()),
            computed_width: None,
            computed_height: None,
        }
    }
}

impl Computed for Rect {
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
}

impl UiNode for Rect {
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
        UiElementType::Rect
    }

    fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut) {
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let border = self.border.and_then(|b| Some(b.size)).unwrap_or(0.0);
        let halfborder = border / 2.0;

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        let cursor = context.cursor_screen_pos();

        let starting_position = [(cursor[0] + halfborder), (cursor[1] + halfborder)];
        let ending_position = [
            (starting_position[0] + width - halfborder),
            (starting_position[1] + height - halfborder),
        ];

        Self::draw(
            context,
            draw_list,
            starting_position,
            ending_position,
            self.background,
            self.border,
        );

        // let starting_position_outer_edge = [
        //     (starting_position[0] - halfborder),
        //     (starting_position[1] - halfborder),
        // ];
        // let ending_position_outer_edge = [
        //     (ending_position[0] + halfborder),
        //     (ending_position[1] + halfborder),
        // ];

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

use std::{collections::HashMap, rc::Weak};

use bevy::color::{Color, LinearRgba};
use imgui::DrawListMut;

use crate::ui::UiColor;

use super::{
    button::{Button, ButtonChild},
    dropdown::{Dropdown, DropdownBox, DropdownChild},
    text::{Text, TextChild},
    Border, Computed, ParentProperties, Size, UiElement, UiElementType, UiNode,
};

#[derive(Debug, Clone, Copy)]
pub enum FlexAxisAlign {
    Start,
    End,
    Center,
    Between,
}

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy)]
pub enum FlexCrossAxisAlign {
    Start,
    End,
    Center,
}

pub struct FlexBuilder<'a> {
    parent: &'a mut Flex,
}

#[derive(Debug, Clone)]
pub struct Flex {
    axis_align_items: FlexAxisAlign,
    cross_axis_align_items: FlexCrossAxisAlign,
    direction: FlexDirection,
    gap: f32,
    width: Size,
    height: Size,
    border: Option<Border>,
    padding: f32,
    fill: Option<UiColor>,
    children: Vec<UiElement>,
    computed_width: Option<f32>,
    computed_height: Option<f32>,
    computed_axis_available_space: Option<f32>,
}

impl Default for Flex {
    fn default() -> Self {
        Self {
            axis_align_items: FlexAxisAlign::Start,
            cross_axis_align_items: FlexCrossAxisAlign::Start,
            direction: FlexDirection::Row,
            gap: 0.0,
            width: Size::Pixels(320.0),
            height: Size::Pixels(60.0),
            border: None,
            padding: 0.0,
            fill: None,
            children: Vec::new(),
            computed_width: None,
            computed_height: None,
            computed_axis_available_space: None,
        }
    }
}

impl UiNode for Flex {
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
        Some(&self.children)
    }

    fn get_type(&self) -> UiElementType {
        UiElementType::Flex
    }

    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &DrawListMut, /* cascading_override: Override */
    ) {
        match self.direction {
            FlexDirection::Row => self.build_row(context, draw_list /* cascading_override */),
            FlexDirection::Column => {
                // self.build_experimental(context, draw_list /* cascading_override */)
                self.build_column(context, draw_list)
            }
        }
    }
}

impl Computed for Flex {
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
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        if self.children.is_empty() {
            return;
        }

        let border = self.border.and_then(|b| Some(b.size)).unwrap_or(0.0);

        let total_width = self.computed_width.unwrap();
        let available_width = total_width - border - (self.padding * 2.0);
        let total_height = self.computed_height.unwrap();
        let available_height = total_height - border - (self.padding * 2.0);

        let mut child_axis_pixels = 0.0;
        let mut child_axis_percentage = 0.0;
        let mut child_axis_auto_count = 0_u8;

        // Calculate space distribution along main axis,
        // and set the cross-axis size for each child.
        for child in &mut self.children {
            match self.direction {
                FlexDirection::Row => {
                    // Calculate axis space distribution
                    match child.get_width() {
                        Size::Pixels(p) => child_axis_pixels += p,
                        Size::Percentage(p) => child_axis_percentage += p,
                        Size::Auto => child_axis_auto_count += 1,
                    }

                    // Set cross-axis sizes
                    child.set_computed_height(match child.get_height() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_height * p / 100.0,
                        Size::Auto => available_height,
                    });
                }
                FlexDirection::Column => {
                    // Calculate axis space distribution
                    match child.get_height() {
                        Size::Pixels(p) => child_axis_pixels += p,
                        Size::Percentage(p) => child_axis_percentage += p,
                        Size::Auto => child_axis_auto_count += 1,
                    }

                    // Set cross-axis sizes
                    child.set_computed_width(match child.get_width() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_width * p / 100.0,
                        Size::Auto => available_width,
                    });
                }
            }
        }

        let gap_count = self.children.len() - 1;
        let total_gap_space = self.gap * gap_count as f32;

        // Calculate the total unfilled space, taking into account all the Pixel and Percentage values
        // (this space will later be distributed among Auto-sized children, if there are any)
        let axis_available_space = match self.direction {
            FlexDirection::Row => {
                available_width
                    - (child_axis_pixels
                        + (child_axis_percentage * available_width / 100.0)
                        + total_gap_space)
            }
            FlexDirection::Column => {
                available_height
                    - (child_axis_pixels
                        + (child_axis_percentage * available_height / 100.0)
                        + total_gap_space)
            }
        };

        let mut contains_auto = false;

        let self_properties = ParentProperties {
            computed_width: self.computed_width,
            computed_height: self.computed_height,
            width_sizing: &self.width,
            height_sizing: &self.height,
            padding: self.padding,
        };

        for child in &mut self.children {
            // The cross-axis size has already been set by the previous loop
            match self.direction {
                FlexDirection::Row => {
                    let axis_size = match child.get_width() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_width * p / 100.0,
                        Size::Auto => {
                            contains_auto = true;
                            axis_available_space / child_axis_auto_count as f32
                        }
                    };
                    child.set_computed_width(axis_size);
                }
                FlexDirection::Column => {
                    let axis_size = match child.get_height() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_height * p / 100.0,
                        Size::Auto => {
                            contains_auto = true;
                            axis_available_space / child_axis_auto_count as f32
                        }
                    };

                    child.set_computed_height(axis_size);
                }
            }

            child.compute_children_size(&self_properties);
        }

        // If there are any Auto-sized children, they fill all the available space, thus there is None left
        self.computed_axis_available_space = match contains_auto {
            true => None,
            false => Some(axis_available_space),
        };
    }
}

impl Flex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align_axis(&mut self, spacing: FlexAxisAlign) -> &mut Self {
        self.axis_align_items = spacing;
        self
    }

    pub fn align_cross_axis(&mut self, spacing: FlexCrossAxisAlign) -> &mut Self {
        self.cross_axis_align_items = spacing;
        self
    }

    pub fn flex_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn width(&mut self, width: Size) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: Size) -> &mut Self {
        self.height = height;
        self
    }

    pub fn border(&mut self, border: Option<Border>) -> &mut Self {
        self.border = border;
        self
    }

    pub fn fill(&mut self, fill: UiColor) -> &mut Self {
        self.fill = Some(fill);
        self
    }

    pub fn children(&mut self, f: impl FnOnce(&mut FlexBuilder)) -> &mut Self {
        let mut builder = FlexBuilder { parent: self };
        f(&mut builder);
        self
    }

    fn build_row(&self, context: &imgui::Ui, draw_list: &DrawListMut) {
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

        let starting_position_outer_edge = [
            (starting_position[0] - halfborder),
            (starting_position[1] - halfborder),
        ];
        let ending_position_outer_edge = [
            (ending_position[0] + halfborder),
            (ending_position[1] + halfborder),
        ];

        if let Some(fill) = self.fill {
            draw_list
                .add_rect(starting_position, ending_position, fill)
                .filled(true)
                .build();
        }

        // NOTE if let some self.border???
        if self.border.is_some() {
            draw_list
                .add_rect(
                    starting_position,
                    ending_position,
                    self.border.unwrap().color,
                )
                .thickness(border)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let axis_empty_space = self.computed_axis_available_space.unwrap_or(0.0);

            let number_of_children = self.children.len();
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            // Used for AxisAlign::Between
            let calculated_gap = ((axis_empty_space - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                assert!(
                    child.get_computed_height().is_some(),
                    "Computed height for child {} is unset.",
                    i
                );

                let child_height = child.get_computed_height().unwrap();

                let vertical_empty_space = height - child_height - border;

                let inner_cursor = context.cursor_screen_pos();

                let vertical_adjusted_start = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Start => starting_position[1],
                    FlexCrossAxisAlign::End => starting_position[1] + vertical_empty_space,
                    FlexCrossAxisAlign::Center => {
                        starting_position[1] + (height / 2.0 - child_height / 2.0)
                    }
                };

                if i == 0 {
                    match self.axis_align_items {
                        FlexAxisAlign::End => {
                            context.set_cursor_screen_pos([
                                inner_cursor[0] + axis_empty_space,
                                vertical_adjusted_start,
                            ]);
                        }
                        _ => context
                            .set_cursor_screen_pos([inner_cursor[0], vertical_adjusted_start]),
                    }
                } else {
                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            context.set_cursor_screen_pos([
                                inner_cursor[0] + calculated_gap,
                                vertical_adjusted_start,
                            ]);
                        }
                        _ => context.set_cursor_screen_pos([
                            inner_cursor[0] + self.gap,
                            vertical_adjusted_start,
                        ]),
                    }
                }

                child.build(context, &draw_list);

                // Tell ImGui that we intend to render the next child on the same line
                context.same_line_with_spacing(0.0, 0.0);
            }
        }

        context.set_cursor_screen_pos([
            ending_position_outer_edge[0],
            starting_position_outer_edge[1],
        ]);
    }

    fn build_column(&self, context: &imgui::Ui, draw_list: &DrawListMut) {
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        let border = self.border.and_then(|b| Some(b.size)).unwrap_or(0.0);
        let halfborder = border / 2.0;

        let cursor = context.cursor_screen_pos();
        let starting_position = [
            (cursor[0] + halfborder).floor(),
            (cursor[1] + halfborder).floor(),
        ];
        let ending_position = [
            (starting_position[0] + width - halfborder).floor(),
            (starting_position[1] + height - halfborder).floor(),
        ];

        if let Some(fill) = self.fill {
            draw_list
                .add_rect(starting_position, ending_position, fill)
                .filled(true)
                .build();
        }

        if self.border.is_some() {
            draw_list
                .add_rect(
                    starting_position,
                    ending_position,
                    self.border.unwrap().color,
                )
                .thickness(border)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let axis_empty_space = self.computed_axis_available_space.unwrap_or(0.0);

            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            // Used for AxisAlign::Between
            let calculated_gap = ((axis_empty_space - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                assert!(
                    child.get_computed_width().is_some(),
                    "Computed width for child {} is unset.",
                    i
                );

                let child_width = child.get_computed_width().unwrap();

                let horizontal_empty_space = width - child_width - border;

                let inner_cursor = context.cursor_screen_pos();

                // This is the alignment on the cross-axis, which in this case is the horizontal alignment
                let cross_axis_adjusted_start = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Start => starting_position[0],
                    FlexCrossAxisAlign::End => starting_position[0] + horizontal_empty_space,
                    FlexCrossAxisAlign::Center => {
                        starting_position[0] + (width / 2.0 - child_width / 2.0)
                    }
                };

                if i == 0 {
                    match self.axis_align_items {
                        FlexAxisAlign::End => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                inner_cursor[1] + axis_empty_space,
                            ]);
                        }
                        FlexAxisAlign::Center => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                inner_cursor[1] + axis_empty_space / 2.0,
                            ]);
                        }
                        _ => context
                            .set_cursor_screen_pos([cross_axis_adjusted_start, inner_cursor[1]]),
                    }
                } else {
                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                inner_cursor[1]
                                    + calculated_gap
                                    + self.children[i - 1].get_computed_height().unwrap(),
                            ]);
                        }
                        FlexAxisAlign::End => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1]
                                + self.gap
                                + self.children[i - 1].get_computed_height().unwrap(),
                        ]),
                        _ => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1]
                                + self.gap
                                + self.children[i - 1].get_computed_height().unwrap(),
                        ]),
                    }
                }

                child.build(context, &draw_list);
            }
        }

        context.set_cursor_screen_pos([starting_position[0], ending_position[1]]);
    }
}

pub trait FlexChild {
    fn flex(&mut self) -> &mut Flex;
}

impl<'a> FlexChild for FlexBuilder<'a> {
    fn flex(&mut self) -> &mut Flex {
        self.parent.children.push(UiElement::Flex(Flex::new()));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

impl<'a> TextChild for FlexBuilder<'a> {
    fn text(&mut self, text: &str) -> &mut Text {
        self.parent.children.push(UiElement::Text(Text::new(text)));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Text(text) => text,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

impl<'a> ButtonChild for FlexBuilder<'a> {
    fn button(&mut self) -> &mut Button {
        self.parent.children.push(UiElement::Button(Button::new()));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Button(button) => button,
            _ => unreachable!("Button is not buttoning :("),
        }
    }
}

impl<'a> DropdownChild for FlexBuilder<'a> {
    // fn dropdown<T: 'static + PartialEq + Clone>(
    //     &mut self,
    //     dropdown: Dropdown<T>,
    // ) -> &mut Dropdown<T> {
    //     let dropdown_box = DropdownBox::new(dropdown);
    //     self.parent.children.push(UiElement::Dropdown(dropdown_box));

    //     match self.parent.children.last_mut().unwrap() {
    //         UiElement::Dropdown(dropdown_box) => dropdown_box.downcast_mut::<T>().unwrap(),
    //         _ => unreachable!("Dropdown not dropdowning :("),
    //     }
    // }
    fn dropdown<T: 'static + PartialEq + Clone>(&mut self) -> &mut Dropdown<T> {
        let dropdown_box = DropdownBox::new(Dropdown::<T>::new());
        self.parent.children.push(UiElement::Dropdown(dropdown_box));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Dropdown(dropdown_box) => dropdown_box.downcast_mut::<T>().unwrap(),
            _ => unreachable!("Dropdown not dropdowning :("),
        }
    }
}

use std::{panic::Location, sync::atomic};

use imgui::DrawListMut;

use mint::Vector2 as MintVec2;

use crate::ui::{element::rect::Rect, window::test_window::IS_SHOW_UNNAMED_ELEMENTS, UiColor};

use super::{
    button::ButtonChild, dropdown::DropdownChild, input::InputI32Child, text::TextChild, Border,
    Builder, Computed, ParentProperties, Size, UiElement, UiElementType, UiNode,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct ComputedOverflow {
    pub x: Option<u32>,
    pub y: Option<u32>,
}

pub struct FlexBuilder<'a> {
    parent: &'a mut Flex,
}

impl<'a> Builder for FlexBuilder<'a> {
    #[allow(refining_impl_trait)]
    fn parent(&mut self) -> &mut Flex {
        self.parent
    }
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
    computed_overflow: ComputedOverflow,
    debug_id: Option<&'static str>,
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
            computed_overflow: ComputedOverflow::default(),
            debug_id: None,
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

    fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>> {
        Some(&mut self.children)
    }

    fn get_type(&self) -> UiElementType {
        UiElementType::Flex
    }

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut) {
        match self.direction {
            FlexDirection::Row => self.build_row(context, draw_list),
            FlexDirection::Column => self.build_column(context, draw_list),
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

    fn compute_children_size(
        &mut self,
        context: &imgui::Ui,
        _parent_properties: &ParentProperties,
    ) {
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
                    let min_height = child.calculate_min_height(context);

                    // Calculate axis space distribution (NOTE: the actual sizes are calculated a bit later, a couple of lines below)
                    match child.get_width() {
                        Size::Pixels(p) => child_axis_pixels += p,
                        Size::Percentage(p) => child_axis_percentage += p,
                        Size::Auto => child_axis_auto_count += 1,
                    }

                    let height = match child.get_height() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_height * p / 100.0,
                        Size::Auto => available_height,
                    };

                    // If the minimum height is larger than the actual height, use the minimum height
                    let applied_height = min_height
                        .and_then(|min| height.lt(&min).then_some(min))
                        .unwrap_or(height);

                    // Set cross-axis sizes
                    child.set_computed_height(applied_height);
                }
                FlexDirection::Column => {
                    let min_width = child.calculate_min_width(context);

                    match child.get_height() {
                        Size::Pixels(p) => child_axis_pixels += p,
                        Size::Percentage(p) => child_axis_percentage += p,
                        Size::Auto => child_axis_auto_count += 1,
                    }

                    let width = match child.get_width() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_width * p / 100.0,
                        Size::Auto => available_width,
                    };

                    let applied_width = min_width
                        .and_then(|min| width.lt(&min).then_some(min))
                        .unwrap_or(width);

                    // Set cross-axis sizes
                    child.set_computed_width(applied_width);
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

        // TODO: uncomment if feeling like implementing overflow, lol
        // let mut axis_remaining_space = axis_available_space;

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
                    let min_axis_size = child.calculate_min_width(context);

                    let axis_size = match child.get_width() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_width * p / 100.0,
                        Size::Auto => {
                            contains_auto = true;
                            axis_available_space / child_axis_auto_count as f32
                        }
                    };

                    let applied_axis_size = min_axis_size
                        .and_then(|min| axis_size.lt(&min).then_some(min))
                        .unwrap_or(axis_size);

                    child.set_computed_width(applied_axis_size);
                }
                FlexDirection::Column => {
                    let min_axis_size = child.calculate_min_height(context);

                    let axis_size = match child.get_height() {
                        Size::Pixels(p) => *p,
                        Size::Percentage(p) => available_height * p / 100.0,
                        Size::Auto => {
                            contains_auto = true;
                            axis_available_space / child_axis_auto_count as f32
                        }
                    };

                    let applied_axis_size = min_axis_size
                        .and_then(|min| axis_size.lt(&min).then_some(min))
                        .unwrap_or(axis_size);

                    child.set_computed_height(applied_axis_size);
                }
            }

            child.compute_children_size(context, &self_properties);
        }

        // If there are any Auto-sized children, they fill all the available space, thus there is None left
        self.computed_axis_available_space = match contains_auto {
            true => None,
            false => Some(axis_available_space), // FIXME does not account for overflow and children
                                                 // where child.min_size > child.size
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

    pub fn debug_id(&mut self, id: &'static str) -> &mut Self {
        self.debug_id = Some(id);
        self
    }

    #[track_caller]
    fn move_cursor(
        &self,
        context: &imgui::Ui,
        new_position: impl Into<MintVec2<f32>> + Clone,
        caller: &'static str,
    ) {
        let location = Location::caller();

        if !IS_SHOW_UNNAMED_ELEMENTS.load(atomic::Ordering::SeqCst) && self.debug_id.is_none() {
            context.set_cursor_screen_pos(new_position);
            return;
        } else {
            crate::ui::window::test_window::MOVE_CURSOR_CALLS.with(|calls| {
                calls.borrow_mut().push((
                    new_position.clone().into().into(),
                    self.debug_id,
                    caller,
                    location.line(),
                ));
            });
        }

        context.set_cursor_screen_pos(new_position);
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

        Rect::draw(
            context,
            draw_list,
            starting_position,
            ending_position,
            self.fill,
            self.border,
        );

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
                            self.move_cursor(
                                context,
                                [inner_cursor[0] + axis_empty_space, vertical_adjusted_start],
                                "build_row, i == 0, FlexAxisAlign::End",
                            );
                        }
                        _ => self.move_cursor(
                            context,
                            [inner_cursor[0], vertical_adjusted_start],
                            "build_row, i == 0, FlexAxisAlign::_",
                        ),
                    }
                } else {
                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            self.move_cursor(
                                context,
                                [inner_cursor[0] + calculated_gap, vertical_adjusted_start],
                                "build_row, i != 0, FlexAxisAlign::Between",
                            );
                        }
                        _ => self.move_cursor(
                            context,
                            [inner_cursor[0] + self.gap, vertical_adjusted_start],
                            "build_row, i != 0, FlexAxisAlign::_",
                        ),
                    }
                }

                let cursor_right_before_drawing_child = context.cursor_screen_pos();

                child.build(context, &draw_list);
            }
        }

        self.move_cursor(
            context,
            [ending_position[0], ending_position[1]],
            "build_row end",
        );
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

        Rect::draw(
            context,
            draw_list,
            starting_position,
            ending_position,
            self.fill,
            self.border,
        );

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
                            self.move_cursor(
                                context,
                                [
                                    cross_axis_adjusted_start,
                                    inner_cursor[1] + axis_empty_space,
                                ],
                                "build_column, i == 0, FlexAxisAlign::End",
                            );
                        }
                        FlexAxisAlign::Center => {
                            self.move_cursor(
                                context,
                                [
                                    cross_axis_adjusted_start,
                                    inner_cursor[1] + axis_empty_space / 2.0,
                                ],
                                "build_column, i == 0, FlexAxisAlign::Center",
                            );
                        }
                        _ => self.move_cursor(
                            context,
                            [cross_axis_adjusted_start, inner_cursor[1]],
                            "build_column, i == 0, FlexAxisAlign::_",
                        ),
                    }
                } else {
                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            self.move_cursor(
                                context,
                                [
                                    cross_axis_adjusted_start,
                                    inner_cursor[1] + calculated_gap, // + self.children[i - 1].get_computed_height().unwrap(),
                                ],
                                "build_column, i != 0, FlexAxisAlign::Between",
                            );
                        }
                        FlexAxisAlign::End => self.move_cursor(
                            context,
                            [
                                cross_axis_adjusted_start,
                                inner_cursor[1] + self.gap, // + self.children[i - 1].get_computed_height().unwrap(),
                            ],
                            "build_column, i != 0, FlexAxisAlign::End",
                        ),
                        _ => self.move_cursor(
                            context,
                            [
                                cross_axis_adjusted_start,
                                inner_cursor[1] + self.gap, // + self.children[i - 1].get_computed_height().unwrap(),
                            ],
                            "build_column, i != 0, FlexAxisAlign::_",
                        ),
                    }
                }

                child.build(context, &draw_list);
            }
        }

        self.move_cursor(
            context,
            [ending_position[0], ending_position[1]],
            "build_column end",
        );
    }
}

pub trait FlexChild: Builder {
    fn flex(&mut self) -> &mut Flex {
        let children = self.parent().get_children_mut().unwrap();

        children.push(UiElement::Flex(Flex::new()));

        match children.last_mut().unwrap() {
            UiElement::Flex(f) => f,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

impl<'a> FlexChild for FlexBuilder<'a> {}
impl<'a> TextChild for FlexBuilder<'a> {}
impl<'a> ButtonChild for FlexBuilder<'a> {}
impl<'a> DropdownChild for FlexBuilder<'a> {}
impl<'a> InputI32Child for FlexBuilder<'a> {}

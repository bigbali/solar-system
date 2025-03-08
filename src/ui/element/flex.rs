use std::{collections::HashMap, rc::Weak};

use bevy::color::{Color, LinearRgba};
use imgui::DrawListMut;

use crate::ui::UiColor;

use super::{
    button::{Button, ButtonChild}, dropdown::{Dropdown, DropdownBox, DropdownChild}, Border, Computed, Override, ParentProperties, Size, SizeOverride, UiElement, UiNode
};

#[derive(Debug, Clone, Copy)]
pub enum FlexAxisAlign {
    Start,
    End,
    Between,
    Stretch,
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
    Stretch,
}

pub struct FlexBuilder<'a> {
    parent: &'a mut Flex,
}

impl<'a> FlexBuilder<'a> {
    pub fn flex_default(&mut self) -> &mut Flex {
        self.parent.children.push(UiElement::Flex(Flex::default()));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }

    pub fn flex(&mut self, flex: Flex) -> &mut Flex {
        self.parent.children.push(UiElement::Flex(flex));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }

    pub fn parent_width(&self) -> f32 {
        self.parent.width
    }

    pub fn parent_height(&self) -> f32 {
        self.parent.height
    }
}

#[derive(Debug, Clone)]
pub enum FlexChildSizeOverride {
    Percentage(f32),
    Pixels(f32),
}

#[derive(Debug, Clone)]
pub struct Flex {
    pub axis_align_items: FlexAxisAlign,
    pub cross_axis_align_items: FlexCrossAxisAlign,
    pub direction: FlexDirection,
    // pub axis_size_override: HashMap<usize, FlexChildSizeOverride>,
    // pub cross_axis_size_override: HashMap<usize, FlexChildSizeOverride>,
    pub gap: f32,
    pub width: Size,
    pub height: Size,
    pub border: Border,
    // pub fill_parent: bool,
    pub fill: Option<UiColor>,
    pub children: Vec<UiElement>,
    pub computed_width: Option<f32>,
    pub computed_height: Option<f32>,
}

impl Default for Flex {
    fn default() -> Self {
        Self {
            axis_align_items: FlexAxisAlign::Start,
            cross_axis_align_items: FlexCrossAxisAlign::Start,
            direction: FlexDirection::Row,
            // axis_size_override: HashMap::new(),
            // cross_axis_size_override: HashMap::new(),
            gap: 0.0,
            width: Size::Pixels(320.0),
            height: Size::Pixels(60.0),
            border: Border {
                size: 0.0,
                color: UiColor::from(LinearRgba::BLACK),
            },
            // fill_parent: false,
            fill: None,
            children: Vec::new(),
            computed_width: None,
            computed_height: None,
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

    fn get_border(&self) -> Border {
        self.border
    }

    fn get_children(&self) -> &Vec<UiElement> {
        &self.children
    }

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override) {
        match self.direction {
            FlexDirection::Row => self.build_row(context, draw_list, cascading_override),
            FlexDirection::Column => {
                self.build_experimental(context, draw_list, cascading_override)
            }
        }
    }
}

impl Computed for Flex {
    fn get_computed_width(&self) -> Option<f32> {
        self.computed_width
    }

    fn get_computed_height(&self) -> Option<f32> {
        self.computed_height
    }


    fn compute_size(&mut self, parent_properties: &ParentProperties) -> (f32, f32) {
        assert!(parent_properties.computed_width.is_some(), "Computed parent width is unset.");
        assert!(parent_properties.computed_height.is_some(), "Computed parent height is unset.");

        let parent_width = parent_properties.computed_width.unwrap();
        let parent_height = parent_properties.computed_height.unwrap();

       // account for gap as well

       // what happens when fitcontent has child with fillavailable?
       // if fitcontent has child with fitcontent, throw error

      

        let w = match self.width {
            Size::FitContent => {
                fn ensure_valid_sizing(children: &Vec<UiElement>) {
                    for child in children {
                        match child.get_width() {
                            Size::FillAvailable => panic!("Flex with width FitContent cannot have direct child with width FillAvailable"),
                            Size::Percentage(_) => panic!("Flex with width FitContent cannot have direct child with width Percentage"),
                            _ => ()
                        }
                    }
                }

                let self_properties = ParentProperties {
                    computed_width: None,
                    computed_height: None,
                    width_sizing: &self.width,
                    height_sizing: &self.height,
                };

                let children_combined_width = match self.children.len() {
                    0 => 0.0,
                    1.. => match self.direction {
                        FlexDirection::Row => self.children.iter().map(|i| i.compute_size(&self_properties).0).sum::<f32>() + self.gap * (self.children.len() - 1) as f32,
                        FlexDirection::Column => {
                            self.children.iter().map(|i| (i.compute_size(&self_properties).0)).reduce(f32::max).unwrap()
                        }
                    }
                };

                children_combined_width

            },
            Size::FillAvailable => {
                parent_width
            }
            Size::Pixels(p) => {
                p
            }
            Size::Percentage(p) => {
                parent_width * p / 100.0
            }
        };

        let h = match self.height {
            Size::FitContent => {
                fn ensure_valid_sizing(children: &Vec<UiElement>) {
                    for child in children {
                        match child.get_height() {
                            Size::FillAvailable => panic!("Flex with width FitContent cannot have direct child with height FillAvailable"),
                            Size::Percentage(_) => panic!("Flex with width FitContent cannot have direct child with height Percentage"),
                            _ => ()
                        }
                    }
                }

                let self_properties = ParentProperties {
                    computed_width: None,
                    computed_height: None,
                    width_sizing: &self.width,
                    height_sizing: &self.height,
                };

                let children_combined_height = match self.children.len() {
                    0 => 0.0,
                    1.. => match self.direction {
                        FlexDirection::Row => {
                            self.children.iter().map(|i| (i.compute_size(&self_properties).1)).reduce(f32::max).unwrap()
                        }
                        FlexDirection::Column => self.children.iter().map(|i| i.compute_size(&self_properties).1).sum::<f32>() + self.gap * (self.children.len() - 1) as f32,
                    }
                };

                children_combined_height

            },
            Size::FillAvailable => {
                parent_height
            }
            Size::Pixels(p) => {
                p
            }
            Size::Percentage(p) => {
                parent_height * p / 100.0
            }
        };

        self.computed_width = Some(w);
        self.computed_height = Some(h);

        return (w, h);
    }

}

impl Flex {
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

    // pub fn axis_size_override(&mut self, index: usize, size: FlexChildSizeOverride) -> &mut Self {
    //     self.axis_size_override.insert(index, size);
    //     self
    // }

    // pub fn cross_axis_size_override(
    //     &mut self,
    //     index: usize,
    //     size: FlexChildSizeOverride,
    // ) -> &mut Self {
    //     self.axis_size_override.insert(index, size);
    //     self
    // }

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

    // pub fn height_auto(&mut self) -> &mut Self {
    //     self.height = self.children.iter().fold(0.0, |acc, e| {
    //         let height = e.get_height();
    //         if height > acc {
    //             return height;
    //         }

    //         acc
    //     }) + self.border.size * 2.0;
    //     self
    // }

    pub fn border(&mut self, border: Border) -> &mut Self {
        self.border = border;
        self
    }

    // pub fn fill_parent(&mut self, fill_parent: bool) -> &mut Self {
    //     self.fill_parent = fill_parent;
    //     self
    // }

    pub fn fill(&mut self, fill: UiColor) -> &mut Self {
        self.fill = Some(fill);
        self
    }

    pub fn children(&mut self, f: impl FnOnce(&mut FlexBuilder)) -> &mut Self {
        let mut builder = FlexBuilder { parent: self };
        f(&mut builder);
        self
    }

    fn build_row(
        &self,
        context: &imgui::Ui,
        draw_list: &DrawListMut,
        cascading_override: Override,
    ) {
        // if width auto:
        //     resize object according to initially implemented stretch and whatnot

        // width percentage
        // width pixels

        let children_combined_width = self.children.iter().map(|i| i.get_width()).sum::<f32>();
        let children_combined_width_with_gaps = children_combined_width + self.gap * (self.children.len() - 1);

        let self_max_width = match cascading_override.custom_rendering {
            true => cascading_override.width.unwrap_or(match self.width),
            false => match self.width {
                Size::FillAvailable => context.content_region_avail()[0],
                Size::FitContent => context.content_region_avail()[0] * p / 100.0,
                Size::Pixels(p) => p,
                Size::Percentage(p) => context.content_region_avail()[0] * p / 100.0,
            }
        };

        let max_height = match cascading_override.custom_rendering {
            true => cascading_override.height.unwrap_or(self.height),
            false => self
                .fill_parent
                .then(|| context.content_region_avail()[1])
                .unwrap_or(self.height),
        };

        let halfborder = self.border.size / 2.0;

        let items_width = self.children.iter().map(|i| i.get_width()).sum::<f32>();
        let tallest_item_height = self
            .children
            .iter()
            .map(|i| i.get_height())
            .reduce(|acc, e| {
                if e > acc {
                    return e;
                }
                acc
            })
            .unwrap_or(0.0);

        let horizontal_available_space_for_gap = self_max_width - items_width - self.border.size;

        let size = [self_max_width, max_height];

        let cursor = context.cursor_screen_pos();
        let starting_position = [(cursor[0] + halfborder), (cursor[1] + halfborder)];
        let ending_position = [
            (starting_position[0] + size[0] - halfborder),
            (starting_position[1] + size[1] - halfborder),
        ];
        let outer_starting_position = [
            (starting_position[0] - halfborder),
            (starting_position[1] - halfborder),
        ];
        let outer_ending_position = [
            (ending_position[0] + halfborder),
            (ending_position[1] + halfborder),
        ];

        if let Some(fill) = self.fill {
            draw_list
                .add_rect(starting_position, ending_position, fill)
                .filled(true)
                .build();
        }

        if self.border.size > 0.0 {
            draw_list
                .add_rect(starting_position, ending_position, self.border.color)
                .thickness(self.border.size)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let number_of_children = self.children.len();
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            let use_fixed_width: bool = match self.width {
                Size::FillAvailable => false,
                _ => true,
            };

            let use_fixed_height: bool = match self.height {
                Size::FillAvailable => false,
                _ => true,
            };

            let calculated_gap =
                ((horizontal_available_space_for_gap - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                let vertical_empty_space = max_height - child.get_height() - self.border.size;

                let inner_cursor = context.cursor_screen_pos();

                let vertical_adjusted_start = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Start => starting_position[1],
                    FlexCrossAxisAlign::End => starting_position[1] + vertical_empty_space,
                    FlexCrossAxisAlign::Center => {
                        starting_position[1] + (max_height / 2.0 - child.get_height() / 2.0)
                    }
                    FlexCrossAxisAlign::Stretch => starting_position[1],
                };

                if i == 0 {
                    match self.axis_align_items {
                        FlexAxisAlign::End => {
                            context.set_cursor_screen_pos([
                                inner_cursor[0] + horizontal_available_space_for_gap
                                    - self.gap * gap_division,
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

                let mut width_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (self_max_width - (self.gap * gap_division) - self.border.size * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                if let Some(size) = self.axis_size_override.get(&i) {
                    width_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * self_max_width,
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                let mut height_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_height - self.border.size),
                    _ => None,
                };

                if let Some(size) = self.cross_axis_size_override.get(&i) {
                    height_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * max_height,
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                child.build(
                    context,
                    &draw_list,
                    Override {
                        width: width_override,
                        height: height_override,
                        // parent_element: Some(Weak::)),
                        custom_rendering: true,
                    },
                );

                context.same_line_with_spacing(0.0, 0.0);
            }
        }

        match cascading_override.custom_rendering {
            true => context
                .set_cursor_screen_pos([outer_ending_position[0], outer_starting_position[1]]),
            false => context.set_cursor_screen_pos(outer_ending_position),
        }
    }

    fn build_column(
        &self,
        context: &imgui::Ui,
        draw_list: &DrawListMut,
        cascading_override: Override,
    ) {
        let max_width = match cascading_override.custom_rendering {
            true => cascading_override.width.unwrap_or(self.width),
            false => self
                .fill_parent
                .then(|| context.content_region_avail()[0])
                .unwrap_or(self.width),
        };

        let max_height = match cascading_override.custom_rendering {
            true => cascading_override.height.unwrap_or(self.height),
            false => self
                .fill_parent
                .then(|| context.content_region_avail()[1])
                .unwrap_or(self.height),
        };

        let halfborder = self.border.size / 2.0;

        let items_height = self.children.iter().map(|i| i.get_height()).sum::<f32>();

        let vertical_empty_space = max_height - items_height - self.border.size * 2.0;

        let size = [max_width, max_height];

        let cursor = context.cursor_screen_pos();
        let starting_position = [
            (cursor[0] + halfborder).floor(),
            (cursor[1] + halfborder).floor(),
        ];
        let ending_position = [
            (starting_position[0] + size[0] - halfborder).floor(),
            (starting_position[1] + size[1] - halfborder).floor(),
        ];

        if let Some(fill) = self.fill {
            draw_list
                .add_rect(starting_position, ending_position, fill)
                .filled(true)
                .build();
        }

        if self.border.size > 0.0 {
            draw_list
                .add_rect(starting_position, ending_position, self.border.color)
                .thickness(self.border.size)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            let calculated_gap = ((vertical_empty_space - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                let horizontal_empty_space = max_width - child.get_width() - self.border.size;

                let inner_cursor = context.cursor_screen_pos();

                let cross_axis_adjusted_start = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Start => starting_position[0],
                    FlexCrossAxisAlign::End => starting_position[0] + horizontal_empty_space,
                    FlexCrossAxisAlign::Center => {
                        starting_position[0] + (max_width / 2.0 - child.get_width() / 2.0)
                    }
                    FlexCrossAxisAlign::Stretch => starting_position[0],
                };

                if i == 0 {
                    match self.axis_align_items {
                        FlexAxisAlign::End => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                inner_cursor[1] + vertical_empty_space - self.gap * gap_division,
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
                                    + self.children[i - 1].get_height(),
                            ]);
                        }
                        FlexAxisAlign::End => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1] + self.gap + self.children[i - 1].get_height(),
                        ]),
                        FlexAxisAlign::Stretch => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1]
                                + (max_height - (self.gap * gap_division) - self.border.size * 2.0)
                                    / number_of_children as f32
                                + self.gap,
                        ]),
                        _ => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1] + self.gap + self.children[i - 1].get_height(),
                        ]),
                    }
                }

                let mut width_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_width - self.border.size),
                    _ => None,
                };

                if let Some(size) = self.cross_axis_size_override.get(&i) {
                    width_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * (max_width / 100.0),
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                let mut height_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (max_height - (self.gap * gap_division) - self.border.size * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                if let Some(size) = self.axis_size_override.get(&i) {
                    height_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * (max_height / 100.0),
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                if height_override.is_some() {
                    println!("height_override: {:?}, i: {}", height_override, i);
                }

                child.build(
                    context,
                    &draw_list,
                    Override {
                        width: width_override,
                        height: height_override,
                        custom_rendering: true,
                    },
                );
            }
        }

        match cascading_override.custom_rendering {
            true => context.set_cursor_screen_pos([starting_position[0], ending_position[1]]),
            false => context.set_cursor_screen_pos([ending_position[0], ending_position[1]]),
        }
    }

    fn build_experimental(
        &self,
        context: &imgui::Ui,
        draw_list: &DrawListMut,
        cascading_override: Override,
    ) {
        let max_width = match cascading_override.custom_rendering {
            true => cascading_override.width.unwrap_or(self.width),
            false => self
                .fill_parent
                .then(|| context.content_region_avail()[0])
                .unwrap_or(self.width),
        };

        let max_height = match cascading_override.custom_rendering {
            true => cascading_override.height.unwrap_or(self.height),
            false => self
                .fill_parent
                .then(|| context.content_region_avail()[1])
                .unwrap_or(self.height),
        };

        let halfborder = self.border.size / 2.0;

        let items_height = self.children.iter().map(|i| i.get_height()).sum::<f32>();

        let vertical_empty_space = max_height - items_height - self.border.size * 2.0;

        let size = [max_width, max_height];

        let cursor = context.cursor_screen_pos();
        let starting_position = [
            (cursor[0] + halfborder).floor(),
            (cursor[1] + halfborder).floor(),
        ];
        let ending_position = [
            (starting_position[0] + size[0] - halfborder).floor(),
            (starting_position[1] + size[1] - halfborder).floor(),
        ];

        if let Some(fill) = self.fill {
            draw_list
                .add_rect(starting_position, ending_position, fill)
                .filled(true)
                .build();
        }

        if self.border.size > 0.0 {
            draw_list
                .add_rect(starting_position, ending_position, self.border.color)
                .thickness(self.border.size)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            let calculated_gap = ((vertical_empty_space - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                let horizontal_empty_space = max_width - child.get_width() - self.border.size;

                let inner_cursor = context.cursor_screen_pos();

                let cross_axis_adjusted_start = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Start => starting_position[0],
                    FlexCrossAxisAlign::End => starting_position[0] + horizontal_empty_space,
                    FlexCrossAxisAlign::Center => {
                        starting_position[0] + (max_width / 2.0 - child.get_width() / 2.0)
                    }
                    FlexCrossAxisAlign::Stretch => starting_position[0],
                };

                if i == 0 {
                    match self.axis_align_items {
                        FlexAxisAlign::End => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                inner_cursor[1] + vertical_empty_space - self.gap * gap_division,
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
                                    + self.children[i - 1].get_height(),
                            ]);
                        }
                        FlexAxisAlign::End => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1] + self.gap + self.children[i - 1].get_height(),
                        ]),
                        FlexAxisAlign::Stretch => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            match cascading_override.height {
                                Some(h) => inner_cursor[1] + h - self.gap,
                                None => {
                                    inner_cursor[1]
                                        + (max_height
                                            - (self.gap * gap_division)
                                            - self.border.size * 2.0)
                                            / number_of_children as f32
                                        + self.gap
                                }
                            },
                        ]),
                        _ => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            inner_cursor[1] + self.gap + self.children[i - 1].get_height(),
                        ]),
                    }
                }

                let mut width_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_width - self.border.size),
                    _ => None,
                };

                if let Some(size) = self.cross_axis_size_override.get(&i) {
                    width_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * (max_width / 100.0),
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                let mut height_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (max_height - (self.gap * gap_division) - self.border.size * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                if let Some(size) = self.axis_size_override.get(&i) {
                    height_override = Some(match size {
                        FlexChildSizeOverride::Percentage(p) => *p * (max_height / 100.0),
                        FlexChildSizeOverride::Pixels(p) => *p,
                    });
                }

                if height_override.is_some() {
                    println!("height_override: {:?}, i: {}", height_override, i);
                }

                child.build(
                    context,
                    &draw_list,
                    Override {
                        width: width_override,
                        height: height_override,
                        custom_rendering: true,
                    },
                );
            }
        }

        match cascading_override.custom_rendering {
            true => context.set_cursor_screen_pos([starting_position[0], ending_position[1]]),
            false => context.set_cursor_screen_pos([ending_position[0], ending_position[1]]),
        }
    }
}

impl<'a> ButtonChild for FlexBuilder<'a> {
    fn button(&mut self, button: Button) -> &mut Button {
        self.parent.children.push(UiElement::Button(button));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Button(button) => button,
            _ => unreachable!("Button is not a child of Flex :("),
        }
    }
}

impl<'a> DropdownChild for FlexBuilder<'a> {
    fn dropdown<T: 'static + PartialEq + Clone>(
        &mut self,
        dropdown: Dropdown<T>,
    ) -> &mut Dropdown<T> {
        let dropdown_box = DropdownBox::new(dropdown);
        self.parent.children.push(UiElement::Dropdown(dropdown_box));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Dropdown(dropdown_box) => dropdown_box.downcast_mut::<T>().unwrap(),
            _ => unreachable!("Dropdown not dropdowning :("),
        }
    }
}

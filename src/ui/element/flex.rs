use bevy::color::{Color, LinearRgba};
use imgui::DrawListMut;

use crate::ui::UiColor;

use super::{
    button::{Button, ButtonChild},
    Border, Override, UiElement, UiNode,
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

pub struct Flex {
    pub axis_align_items: FlexAxisAlign,
    pub cross_axis_align_items: FlexCrossAxisAlign,
    pub direction: FlexDirection,
    pub gap: f32,
    pub width: f32,
    pub height: f32,
    pub border: Border,
    pub fill_parent: bool,
    pub fill: Option<UiColor>,
    pub children: Vec<UiElement>,
}

impl Default for Flex {
    fn default() -> Self {
        Self {
            axis_align_items: FlexAxisAlign::Start,
            cross_axis_align_items: FlexCrossAxisAlign::Start,
            direction: FlexDirection::Row,
            gap: 0.0,
            width: 320.0,
            height: 60.0,
            border: Border {
                size: 0.0,
                color: UiColor::from(LinearRgba::BLACK),
            },
            fill_parent: false,
            fill: None,
            children: Vec::new(),
        }
    }
}

impl UiNode for Flex {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_border(&self) -> Border {
        self.border
    }

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override) {
        match self.direction {
            FlexDirection::Row => self.build_row(context, draw_list, cascading_override),
            FlexDirection::Column => self.build_column(context, draw_list, cascading_override),
        }
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

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn height_auto(&mut self) -> &mut Self {
        self.height = self.children.iter().fold(0.0, |acc, e| {
            let height = e.get_height();
            if height > acc {
                return height;
            }

            acc
        }) + self.border.size * 2.0;
        self
    }

    pub fn border(&mut self, border: Border) -> &mut Self {
        self.border = border;
        self
    }

    pub fn fill_parent(&mut self, fill_parent: bool) -> &mut Self {
        self.fill_parent = fill_parent;
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

    fn build_row(
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

        let horizontal_available_space_for_gap = max_width - items_width - self.border.size;

        let size = [max_width, max_height];

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

                let width_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (max_width - (self.gap * gap_division) - self.border.size * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                let height_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_height - self.border.size),
                    _ => None,
                };

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

                let width_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_width - self.border.size),
                    _ => None,
                };

                let height_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (max_height - (self.gap * gap_division) - self.border.size * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

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

use std::{cell::RefCell, rc::Rc};

use bevy::log::tracing_subscriber::fmt::format;
use delegate::delegate;
use imgui::DrawListMut;

use super::FlexAxisAlign;

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

pub trait UiNode {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_border(&self) -> f32;

    // fn new() -> Self
    // where
    //     Self: Sized;

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override);
}

#[derive(Debug, Default, Clone)]
pub struct Override {
    width: Option<f32>,
    height: Option<f32>,
    custom_rendering: bool,
}

// **Different UI Components**
pub struct FlexRow {
    axis_align_items: FlexAxisAlign,
    cross_axis_align_items: FlexCrossAxisAlign,
    direction: FlexDirection,
    gap: f32,
    width: f32,
    height: f32,
    border: f32,
    fill_parent: bool,
    fill: Option<[f32; 4]>,
    children: Vec<UiElement>,
}

impl Default for FlexRow {
    fn default() -> Self {
        Self {
            axis_align_items: FlexAxisAlign::Start,
            cross_axis_align_items: FlexCrossAxisAlign::Start,
            direction: FlexDirection::Row,
            gap: 0.0,
            width: 320.0,
            height: 60.0,
            border: 0.0,
            fill_parent: false,
            fill: None,
            children: Vec::new(),
        }
    }
}

impl UiNode for FlexRow {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_border(&self) -> f32 {
        self.border
    }

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override) {
        match self.direction {
            FlexDirection::Row => self.build_row(context, draw_list, cascading_override),
            FlexDirection::Column => self.build_column(context, draw_list, cascading_override),
        }
    }
}

impl FlexRow {
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
        }) + self.border * 2.0;
        self
    }

    pub fn border(&mut self, border: f32) -> &mut Self {
        self.border = border;
        self
    }

    pub fn fill_parent(&mut self, fill_parent: bool) -> &mut Self {
        self.fill_parent = fill_parent;
        self
    }

    pub fn fill(&mut self, fill: [f32; 4]) -> &mut Self {
        self.fill = Some(fill);
        self
    }

    pub fn flex_row(&mut self) -> &mut FlexRow {
        self.children.push(UiElement::FlexRow(FlexRow::default()));

        match self.children.last_mut().unwrap() {
            UiElement::FlexRow(flex_row) => flex_row,
            _ => unreachable!("FlexRow is not FlexRow."),
        }
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

        let halfborder = self.border / 2.0;

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

        let horizontal_available_space_for_gap = max_width - items_width - self.border * 2.0;

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

        if self.border > 0.0 {
            draw_list
                .add_rect(starting_position, ending_position, [1.0, 1.0, 1.0, 1.0])
                .thickness(self.border)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let number_of_children = self.children.len();
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            let calculated_gap =
                ((horizontal_available_space_for_gap - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                let vertical_empty_space = max_height - child.get_height() - self.border;

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
                                starting_position[0] + horizontal_available_space_for_gap
                                    - self.gap * gap_division,
                                vertical_adjusted_start,
                            ]);
                        }
                        _ => context
                            .set_cursor_screen_pos([starting_position[0], vertical_adjusted_start]),
                    }
                } else {
                    let cursor = context.cursor_screen_pos();

                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            context.set_cursor_screen_pos([
                                cursor[0] + calculated_gap,
                                vertical_adjusted_start,
                            ]);
                        }
                        _ => context
                            .set_cursor_screen_pos([cursor[0] + self.gap, vertical_adjusted_start]),
                    }
                }

                let width_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(
                        (max_width - (self.gap * gap_division) - self.border * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                let height_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(max_height - self.border),
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
            true => context.set_cursor_screen_pos([ending_position[0], starting_position[1]]),
            false => context.set_cursor_screen_pos([ending_position[0], ending_position[1]]),
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

        let halfborder = self.border / 2.0;

        let items_height = self.children.iter().map(|i| i.get_height()).sum::<f32>();

        let vertical_empty_space = max_height - items_height - self.border * 2.0;

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

        if self.border > 0.0 {
            draw_list
                .add_rect(starting_position, ending_position, [1.0, 1.0, 1.0, 1.0])
                .thickness(self.border)
                .build();
        }

        let number_of_children = self.children.len();

        if number_of_children > 0 {
            let number_of_children = self.children.len();
            let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

            let calculated_gap = ((vertical_empty_space - halfborder) / gap_division).round();

            for (i, child) in self.children.iter().enumerate() {
                let horizontal_empty_space = max_width - child.get_width() - self.border;

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
                                starting_position[1] + vertical_empty_space
                                    - self.gap * gap_division,
                            ]);
                        }
                        _ => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            starting_position[1],
                        ]),
                    }
                } else {
                    match self.axis_align_items {
                        FlexAxisAlign::Between => {
                            context.set_cursor_screen_pos([
                                cross_axis_adjusted_start,
                                starting_position[1]
                                    + calculated_gap
                                    + self.children[i - 1].get_height(),
                            ]);
                        }
                        _ => context.set_cursor_screen_pos([
                            cross_axis_adjusted_start,
                            starting_position[1] + self.gap + self.children[i - 1].get_height(),
                        ]),
                    }
                }

                let width_override: Option<f32> = match self.cross_axis_align_items {
                    FlexCrossAxisAlign::Stretch => Some(
                        (max_width - (self.gap * gap_division) - self.border * 2.0)
                            / number_of_children as f32,
                    ),
                    _ => None,
                };

                let height_override: Option<f32> = match self.axis_align_items {
                    FlexAxisAlign::Stretch => Some(max_height - self.border),
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

pub struct Button {
    width: f32,
    height: f32,
    border: f32,
    label: String,
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

    fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override) {
        println!("Rendering Button: {}", self.label);
    }
}

// **Enum to hold different types**
pub enum UiElement {
    FlexRow(FlexRow),
    Button(Button),
}

impl UiNode for UiElement {
    delegate! {
        to match self {
            UiElement::FlexRow(f) => f,
            UiElement::Button(b) => b,
        } {
            fn get_width(&self) -> f32;
            fn get_height(&self) -> f32;
            fn get_border(&self) -> f32;
            fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut, cascading_override: Override);
        }
    }
}

pub struct RootNode {
    // context: Rc<RefCell<imgui::Ui>>,
    children: Vec<UiElement>, // Stores multiple types
}

impl RootNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn flex_row(&mut self) -> &mut FlexRow {
        self.children.push(UiElement::FlexRow(FlexRow::default()));

        // Safely extract &mut FlexRow from the last UiElement
        match self.children.last_mut().unwrap() {
            UiElement::FlexRow(flex_row) => flex_row,
            _ => unreachable!("We just pushed a FlexRow, so this is impossible"),
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.children.push(UiElement::Button(button));
    }

    // pub fn build(&mut self) {
    //     for child in &self.children {
    //         child.build(&self.context, &self.draw_list);
    //     }
    // }

    pub fn build(&mut self, ui: &imgui::Ui) {
        // Borrow the context inside the method
        let context = ui;

        // Get the draw list directly from the borrowed context
        let draw_list = context.get_window_draw_list();

        // Now build the children
        for child in &self.children {
            child.build(&context, &draw_list, Override::default());
        }
    }
}

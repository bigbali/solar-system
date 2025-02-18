use std::{cell::RefCell, rc::Rc};

use bevy::log::tracing_subscriber::fmt::format;
use delegate::delegate;
use imgui::DrawListMut;

use super::FlexAlign;

pub enum FlexDirection {
    Row,
    Column,
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
    axis_align_items: FlexAlign,
    cross_axis_align_items: FlexAlign,
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
            axis_align_items: FlexAlign::Start,
            cross_axis_align_items: FlexAlign::Start,
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
        context.group(|| {
            let max_width = match cascading_override.custom_rendering {
                true => cascading_override.width.unwrap_or(self.width),
                false => self
                    .fill_parent
                    .then(|| context.content_region_avail()[0])
                    .unwrap_or(self.width),
            };

            let halfborder = self.border / 2.0;

            let items_width = self.children.iter().map(|i| i.get_width()).sum::<f32>();

            let available_space_for_gap = max_width - items_width - self.border * 2.0;

            let cursor = context.cursor_screen_pos();
            let starting_position = [
                (cursor[0] + halfborder).floor(),
                (cursor[1] + halfborder).floor(),
            ];
            let size = [max_width, self.height];
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
                let gap_division = (number_of_children - 1).max(1) as f32; // make sure that we don't divide by 0

                let calculated_gap =
                    ((available_space_for_gap - halfborder) / gap_division).round();

                for (i, child) in self.children.iter().enumerate() {
                    if i == 0 {
                        match self.axis_align_items {
                            FlexAlign::End => {
                                context.set_cursor_screen_pos([
                                    starting_position[0] + available_space_for_gap
                                        - self.gap * gap_division,
                                    starting_position[1],
                                ]);
                            }
                            _ => (),
                        }
                    } else {
                        match self.axis_align_items {
                            FlexAlign::Start => {
                                context.same_line_with_spacing(0.0, self.gap);
                            }
                            FlexAlign::End => {
                                context.same_line_with_spacing(0.0, self.gap);
                            }
                            FlexAlign::Between => {
                                context.same_line_with_spacing(0.0, calculated_gap);
                            }
                            FlexAlign::Stretch => context.same_line_with_spacing(0.0, self.gap),
                        }
                    }

                    child.build(
                        context,
                        &draw_list,
                        Override {
                            width: match self.axis_align_items {
                                FlexAlign::Stretch => Some(
                                    (max_width - (self.gap * gap_division) - self.border * 2.0)
                                        / number_of_children as f32,
                                ),
                                _ => None,
                            },
                            height: None,
                            custom_rendering: true,
                        },
                    );
                }
            }

            match cascading_override.custom_rendering {
                true => context.set_cursor_screen_pos([ending_position[0], starting_position[1]]),
                false => context.set_cursor_screen_pos([ending_position[0], ending_position[1]]),
            }
        });
    }
}

impl FlexRow {
    pub fn align_axis(&mut self, spacing: FlexAlign) -> &mut Self {
        self.axis_align_items = spacing;
        self
    }

    pub fn align_cross_axis(&mut self, spacing: FlexAlign) -> &mut Self {
        self.cross_axis_align_items = spacing;
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

        // Safely extract &mut FlexRow from the last UiElement
        match self.children.last_mut().unwrap() {
            UiElement::FlexRow(flex_row) => flex_row,
            _ => unreachable!("We just pushed a FlexRow, so this is impossible"),
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

// impl UiNode for UiElement {
//     fn get_width(&self) -> f32 {
//         match self {
//             UiElement::FlexRow(f) => f.get_width(),
//             UiElement::Button(b) => b.get_width(),
//         }
//     }
//     fn get_height(&self) -> f32 {
//         match self {
//             UiElement::FlexRow(f) => f.get_height(),
//             UiElement::Button(b) => b.get_height(),
//         }
//     }
//     fn build(&self, context: &imgui::Ui, draw_list: &DrawListMut) {
//         match self {
//             UiElement::FlexRow(f) => f.build(context, draw_list),
//             UiElement::Button(b) => b.build(context, draw_list),
//         }
//     }
// }

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

// **RootNode with heterogeneous children**
pub struct RootNode {
    // context: Rc<RefCell<imgui::Ui>>,
    children: Vec<UiElement>, // Stores multiple types
}

impl RootNode {
    // pub fn new(context: &imgui::Ui) -> Self {
    //     Self {
    //         draw_list: context.get_window_draw_list(),
    //         context,
    //         children: Vec::new(),
    //     }
    // }

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

// // **Usage Example**
// fn main() {
//     let ui = imgui::Ui::new(); // Example instantiation

//     let mut root = RootNode::new(ui);
//     root.add_flex_row(FlexRow {
//         width: 300.0,
//         height: 50.0,
//     });
//     root.add_button(Button {
//         width: 100.0,
//         height: 30.0,
//         label: "Click Me".to_string(),
//     });

//     root.build(); // Renders all elements
// }

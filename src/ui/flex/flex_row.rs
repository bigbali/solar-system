// use std::{cell::RefCell, rc::Rc};

// use imgui::DrawListMut;

// use super::FlexSpacing;

// pub trait UiNode {
//     fn get_width(&self) -> f32;
//     fn get_height(&self) -> f32;

//     fn new() -> Self
//     where
//         Self: Sized;

//     fn build(
//         &self,
//         context: &imgui::Ui,
//         draw_list: &DrawListMut,
//         cascading_override: Option<Override>,
//     );
// }

// pub struct Override {
//     width: Option<f32>,
//     height: Option<f32>,
// }

// pub trait Children<'ui> {
//     fn add_child<T>(&'ui mut self) -> Rc<RefCell<dyn UiNode + 'ui>>
//     where
//         T: UiNode + 'ui;
// }
// // pub trait Children<'a> {
// //     fn add_child<T>(&'a mut self) -> &'a mut Box<dyn UiNode + 'a>
// //     where
// //         T: UiNode + 'a;
// // }

// pub struct RootNode<'ui, 'a> {
//     context: &'ui imgui::Ui,
//     draw_list: DrawListMut<'ui>,
//     // items: Vec<Box<dyn UiNode + 'a>>,
//     items: Vec<Rc<RefCell<dyn UiNode + 'a>>>,
// }

// impl<'ui> RootNode<'ui> {
//     pub fn new(context: &'ui imgui::Ui) -> Self {
//         Self {
//             context,
//             draw_list: context.get_window_draw_list(),
//             items: vec![],
//         }
//     }

//     pub fn build(&mut self) {
//         for item in self.items.iter() {
//             item.borrow().build(self.context, &self.draw_list, None);
//         }
//     }
// }

// impl<'ui> Children<'ui> for RootNode<'ui> {
//     // fn add_child<T>(&'a mut self) -> &'a mut Box<dyn UiNode + 'a>
//     fn add_child<T>(&mut self) -> Rc<RefCell<dyn UiNode<'ui> + 'ui>>
//     where
//         T: UiNode<'ui> + 'ui,
//     {
//         self.items.push(Rc::new(RefCell::new(T::new())));
//         // self.items.push(Box::new(T::new()));
//         self.items.last().unwrap().clone()
//     }
// }

// pub struct FlexRow<'ui> {
//     horizontal_spacing: FlexSpacing,
//     vertical_spacing: FlexSpacing,
//     gap: f32,
//     width: f32,
//     height: f32,
//     border: f32,
//     fill: Option<[f32; 4]>,
//     items: Vec<Rc<RefCell<dyn UiNode + 'ui>>>,
// }

// impl<'a> UiNode for FlexRow<'a> {
//     fn get_width(&self) -> f32 {
//         self.width
//     }

//     fn get_height(&self) -> f32 {
//         self.height
//     }

//     fn new() -> Self {
//         Self {
//             horizontal_spacing: FlexSpacing::Start,
//             vertical_spacing: FlexSpacing::Start,
//             gap: 0.0,
//             width: 320.0,
//             height: 60.0,
//             border: 0.0,
//             fill: None,
//             items: vec![],
//         }
//     }

//     fn build(
//         &self,
//         context: &imgui::Ui,
//         draw_list: &DrawListMut,
//         cascading_override: Option<Override>,
//     ) {
//         let items_width = self
//             .items
//             .iter()
//             .map(|i| i.borrow().get_width())
//             .sum::<f32>();
//         let container_available_width = context.window_content_region_max()[0] - self.border * 4.0;
//         let container_available_height = context.window_content_region_max()[1] - self.border * 4.0;

//         let available_space_for_gap = container_available_width - items_width;

//         let calculated_gap =
//             (available_space_for_gap - self.border * 2.0) / (self.items.len() - 1) as f32;

//         let width_override: Option<f32> = match self.horizontal_spacing {
//             FlexSpacing::Stretch => Some(
//                 (container_available_width
//                     - (self.gap * (self.items.len() as f32 - 1.0)) as f32
//                     - self.border * 2.0)
//                     / self.items.len() as f32,
//             ),
//             _ => None,
//         };

//         context.new_line();

//         context.group(|| {
//             let cursor_start = context.cursor_screen_pos();
//             let p0 = cursor_start;
//             let size = [container_available_width, self.height];
//             let p1 = [p0[0] + size[0], p0[1] + size[1]];

//             context // TODO add padding
//                 .set_cursor_screen_pos([
//                     cursor_start[0] + self.border,
//                     cursor_start[1] + self.border,
//                 ]);
//             let draw_list = context.get_window_draw_list();

//             if let Some(fill) = self.fill {
//                 draw_list.add_rect(p0, p1, fill).filled(true).build();
//             }

//             if self.border > 0.0 {
//                 draw_list
//                     .add_rect(p0, p1, [1.0, 1.0, 1.0, 1.0])
//                     .thickness(self.border)
//                     .build();
//             }

//             for (i, item) in self.items.iter().enumerate() {
//                 if i == 0 {
//                     match self.horizontal_spacing {
//                         FlexSpacing::End => {
//                             context.set_cursor_screen_pos([
//                                 p0[0] + available_space_for_gap
//                                     - self.gap * (self.items.len() - 1) as f32
//                                     - self.border,
//                                 p0[1] + self.border,
//                             ]);
//                         }
//                         _ => (),
//                     }
//                 } else {
//                     match self.horizontal_spacing {
//                         FlexSpacing::Start => {
//                             context.same_line_with_spacing(0.0, self.gap);
//                         }
//                         FlexSpacing::End => {
//                             context.same_line_with_spacing(0.0, self.gap);
//                         }
//                         FlexSpacing::Between => {
//                             context.same_line_with_spacing(0.0, calculated_gap);
//                         }
//                         FlexSpacing::Stretch => context.same_line_with_spacing(0.0, self.gap),
//                     }
//                 }

//                 let cascading_override = match width_override {
//                     Some(width_override) => Some(Override {
//                         width: Some(width_override),
//                         height: None,
//                     }),
//                     None => None,
//                 };

//                 item.borrow().build(context, &draw_list, cascading_override);
//             }

//             let cursor_end = p1;
//             context.set_cursor_screen_pos([cursor_end[0], cursor_start[1] + self.height]);
//         });
//     }
// }

// impl<'a> Children<'a> for FlexRow<'a> {
//     fn add_child<T>(&'a mut self) -> Rc<RefCell<dyn UiNode<'a> + 'a>>
//     where
//         T: UiNode<'a> + 'a,
//     {
//         self.items.push(Rc::new(RefCell::new(T::new())));
//         // self.items.push(Box::new(T::new()));
//         self.items.last().unwrap().clone()
//     }
// }
// // impl_node!(FlexRow<'_>);

// impl<'a> FlexRow<'a> {
//     pub fn horizontal_spacing(&mut self, spacing: FlexSpacing) -> &mut Self {
//         self.horizontal_spacing = spacing;
//         self
//     }

//     pub fn vertical_spacing(&mut self, spacing: FlexSpacing) -> &mut Self {
//         self.vertical_spacing = spacing;
//         self
//     }

//     pub fn gap(&mut self, gap: f32) -> &mut Self {
//         self.gap = gap;
//         self
//     }

//     pub fn width(&mut self, width: f32) -> &mut Self {
//         self.width = width;
//         self
//     }

//     pub fn height(&mut self, height: f32) -> &mut Self {
//         self.height = height;
//         self
//     }

//     pub fn height_auto(&mut self) -> &mut Self {
//         self.height = self.items.iter().fold(0.0, |acc, e| {
//             let height = e.borrow().get_height();
//             if height > acc {
//                 return height;
//             }

//             acc
//         }) + self.border * 2.0;
//         self
//     }

//     pub fn border(&mut self, border: f32) -> &mut Self {
//         self.border = border;
//         self
//     }

//     pub fn fill(&mut self, fill: [f32; 4]) -> &mut Self {
//         self.fill = Some(fill);
//         self
//     }
// }

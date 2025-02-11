// use std::{any::Any, cell::RefCell, rc::Rc};

// use imgui::DrawListMut;

// use crate::{
//     impl_uicontainer,
//     ui::{UiContainer, UiElement, UiElementType},
// };

// use super::FlexSpacing;

// // pub struct Button {
// //     pub label: String,
// //     pub size: [f32; 2],
// //     pub f: Box<dyn Fn() + 'static>,
// //     pub ctx: &'static imgui::Ui,
// // }

// pub trait Buildable<'a>: Any {
//     fn new(ui: &'a imgui::Ui) -> Self
//     where
//         Self: Sized;
//     fn get_context(&self) -> &'a imgui::Ui;
//     fn get_width(&self) -> f32;
//     fn get_height(&self) -> f32;
//     fn get_type(&self) -> UiElementType;
//     fn build(&self);
//     fn build_internal(&self, context: &imgui::Ui, draw_list: Option<&DrawListMut>);
//     // fn as_any(&self) -> &(dyn Any + '_)
//     // where
//     //     Self: Sized,
//     // {
//     //     self
//     // }

//     // fn as_any_mut(&mut self) -> &mut (dyn Any + '_)
//     // where
//     //     Self: Sized,
//     // {
//     //     self
//     // }

//     fn as_any(&self) -> &(dyn Any + 'static);
//     fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
// }

// pub struct FlexRowTest<'a> {
//     horizontal_spacing: FlexSpacing,
//     vertical_spacing: FlexSpacing,
//     gap: f32,
//     width: f32,
//     height: f32,
//     border: f32,
//     fill: Option<[f32; 4]>,
//     imgui: &'a imgui::Ui,
//     children: Vec<Box<dyn Buildable<'a>>>,
// }

// // impl_uicontainer!(FlexRow<'_>);

// impl<'a> Buildable<'a> for FlexRowTest<'a> {
//     fn new(ui: &'a imgui::Ui) -> Self {
//         Self::new(ui)
//     }

//     fn get_context(&self) -> &'a imgui::Ui {
//         self.imgui
//     }

//     fn get_width(&self) -> f32 {
//         self.width
//     }

//     fn get_height(&self) -> f32 {
//         self.height
//     }

//     fn get_type(&self) -> UiElementType {
//         UiElementType::FlexRowTest
//     }

//     // fn flex_row(&self) -> FlexRowTest {
//     //     let flex_row = &mut FlexRowTest::new(&self.imgui);

//     //     self.items.push(Box::new(flex_row));

//     //     flex_row
//     // }

//     // fn flex<C>(&mut self, f: C) -> &mut Self
//     // where
//     //     C: FnOnce(&mut Self),
//     // {
//     //     let mut child = FlexRowTest::new(self.imgui);

//     //     f(&mut child);

//     //     self.items.push(Box::new(child));

//     //     self
//     // }

//     fn build(&self) {
//         self.build_internal(None);
//     }

//     fn build_internal(&self, draw_list: Option<&DrawListMut>) {
//         match draw_list {
//             Some(draw_list) => self.build_debug(false, draw_list),
//             None => self.build_debug(false, &self.imgui.get_window_draw_list()),
//         }
//     }

//     fn as_any(&self) -> &dyn Any {
//         todo!()
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         todo!()
//     }

//     // fn build_as_child(&self, draw_list: &DrawListMut) {
//     //     self.build_debug(false, draw_list);
//     // }
// }

// impl<'a> FlexRowTest<'a> {
//     pub fn new(ui: &'a imgui::Ui) -> Self {
//         Self {
//             horizontal_spacing: FlexSpacing::Start,
//             vertical_spacing: FlexSpacing::Start,
//             gap: 0.0,
//             width: 320.0,
//             height: 60.0,
//             border: 0.0,
//             fill: None,
//             imgui: ui,
//             children: vec![],
//         }
//     }

//     pub fn flex_row(&mut self) -> &mut FlexRowTest<'a> {
//         self.add_child::<FlexRowTest>()
//     }

//     // pub fn button(&mut self, label: &str) -> &mut Button<'a> {
//     //     self.add_child::<Button>()
//     // }

//     // fn add_child<T: Buildable<'a>>(&mut self) -> &mut T {
//     //     // Create the child instance
//     //     let mut child = T::new(self.get_context());

//     //     // Box the child and push it into items
//     //     self.items.push(Box::new(child));

//     //     // Return a mutable reference to the last added child
//     //     self.items.last_mut().unwrap().as_mut()
//     // }

//     // pub fn add_child<T: Buildable<'a>>(&mut self) -> &mut T {
//     //     let child = T::new(self.imgui);

//     //     self.children.push(Box::new(child));

//     //     // // Get a mutable reference to the newly added child
//     //     self.children
//     //         .last_mut()
//     //         .unwrap()
//     //         .as_mut()
//     //         .downcast_mut::<T>()
//     //         .expect("Type mismatch in add_child")
//     // }

//     pub fn add_child<T: Buildable<'a> + 'static>(&mut self) -> &mut T {
//         let child = T::new(self.imgui);
//         self.children.push(Box::new(child));

//         // Now `downcast_mut()` works because `Buildable<'a>` is `Any`
//         self.children
//             .last_mut()
//             .unwrap()
//             .as_mut()
//             .as_any_mut()
//             .downcast_mut::<T>()
//             .expect("Type mismatch in add_child")
//     }

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
//         self.height = self.children.iter().fold(0.0, |acc, e| {
//             if e.get_height() > acc {
//                 return e.get_height();
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

//     // pub fn children<C>(&mut self, f: C) -> &mut Self
//     // where
//     //     C: FnOnce(&mut Self),
//     // {
//     //     // let mut child = Self::new(self.imgui);

//     //     f(self);

//     //     // self.items.push(Box::new(child));

//     //     self
//     // }

//     // fn children<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self {
//     //     f(self);
//     //     self
//     // }

//     pub fn children<F>(&mut self, builder: F) -> &mut Self
//     where
//         F: FnOnce(&mut Self),
//     {
//         builder(self);
//         self
//     }

//     pub fn build_debug(&self, debug: bool, draw_list: &DrawListMut) {
//         let items_width = self.children.iter().map(|i| i.get_width()).sum::<f32>();
//         let container_available_width =
//             self.imgui.window_content_region_max()[0] - self.border * 4.0;
//         let container_available_height =
//             self.imgui.window_content_region_max()[1] - self.border * 4.0;

//         let available_space_for_gap = container_available_width - items_width;

//         let calculated_gap = 8.0;
//         // (available_space_for_gap - self.border * 2.0) / (self.items.len() - 1) as f32;

//         let width_override: Option<f32> = match self.horizontal_spacing {
//             FlexSpacing::Stretch => Some(
//                 (container_available_width
//                     - (self.gap * (self.children.len() as f32 - 1.0)) as f32
//                     - self.border * 2.0)
//                     / self.children.len() as f32,
//             ),
//             _ => None,
//         };

//         self.imgui.new_line();

//         self.imgui.group(|| {
//             let cursor_start = self.imgui.cursor_screen_pos();
//             let p0 = cursor_start;
//             let size = [container_available_width, self.height];
//             let p1 = [p0[0] + size[0], p0[1] + size[1]];

//             self.imgui // TODO add padding
//                 .set_cursor_screen_pos([
//                     cursor_start[0] + self.border,
//                     cursor_start[1] + self.border,
//                 ]);

//             if let Some(fill) = self.fill {
//                 draw_list.add_rect(p0, p1, fill).filled(true).build();
//             }

//             if self.border > 0.0 {
//                 draw_list
//                     .add_rect(p0, p1, [1.0, 1.0, 1.0, 1.0])
//                     .thickness(self.border)
//                     .build();
//             }

//             for (i, item) in self.children.iter().enumerate() {
//                 if i == 0 {
//                     match self.horizontal_spacing {
//                         FlexSpacing::End => {
//                             self.imgui.set_cursor_screen_pos([
//                                 p0[0] + available_space_for_gap
//                                     - self.gap * (self.children.len() - 1) as f32
//                                     - self.border,
//                                 p0[1] + self.border,
//                             ]);
//                         }
//                         _ => (),
//                     }
//                 } else {
//                     match self.horizontal_spacing {
//                         FlexSpacing::Start => {
//                             self.imgui.same_line_with_spacing(0.0, self.gap);
//                         }
//                         FlexSpacing::End => {
//                             self.imgui.same_line_with_spacing(0.0, self.gap);
//                         }
//                         FlexSpacing::Between => {
//                             self.imgui.same_line_with_spacing(0.0, calculated_gap);
//                         }
//                         FlexSpacing::Stretch => self.imgui.same_line_with_spacing(0.0, self.gap),
//                     }
//                 }

//                 let w = match width_override {
//                     Some(w) => w,
//                     None => item.get_width(),
//                 };

//                 item.build_internal(draw_list);
//             }

//             let cursor_end = p1;
//             self.imgui
//                 .set_cursor_screen_pos([cursor_end[0], cursor_start[1] + self.height]);
//         });
//     }
// }

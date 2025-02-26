use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::color::LinearRgba;

use crate::ui::UiColor;

use super::{Border, Override, UiNode};

thread_local! {
    static DROPDOWN_SELECTED_MAP: RefCell<HashMap<usize, i32>> = RefCell::new(HashMap::new());
}

thread_local! {
    static DROPDOWN_ID_INCR: AtomicUsize = AtomicUsize::new(0);
}

pub struct InputI32 {
    pub id: usize,
    pub width: f32,
    pub height: f32,
    pub border: Border,
    pub background: UiColor,
    pub label: String,
    pub on_change: Option<Box<dyn Fn(i32)>>,
    pub step: i32,
    pub default_value: i32,
}

impl InputI32 {
    pub fn id_that_will_not_work_in_immediate_mode_oopsies() -> usize {
        DROPDOWN_ID_INCR.with(|incr| incr.fetch_add(1, Ordering::SeqCst))
    }

    pub fn manual_id(id: usize) -> usize {
        id
    }

    pub fn set_value(&self, value: i32) {
        DROPDOWN_SELECTED_MAP.with_borrow_mut(|map| map.insert(self.id, value));
    }

    pub fn get_value(&self) -> i32 {
        DROPDOWN_SELECTED_MAP.with_borrow(|map| *map.get(&self.id).unwrap_or(&self.default_value))
    }

    // pub fn get_value_ref(&self) -> &i32 {
    //     DROPDOWN_SELECTED_MAP.with_borrow(|map| map.get(&self.id).unwrap_or(&self.default_value))
    // }
}

impl Default for InputI32 {
    fn default() -> Self {
        Self {
            id: InputI32::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: 120.0,
            height: 48.0,
            border: Border {
                size: 0.0,
                color: UiColor::from(LinearRgba::BLACK),
            },
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_change: None,
            step: 1,
            default_value: 0, // values: Vec::new(),
                              // selected_index: Cell::new(0),
        }
    }
}

impl UiNode for InputI32 {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_border(&self) -> Border {
        self.border
    }

    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        cascading_override: Override,
    ) {
        // if self.values.is_empty() {
        //     return;
        // }

        // context.input_scalar(label, value)

        // if let Some(cb) = context.begin_combo(self.label.clone(), self.get_label()) {
        //     let mut current: usize = self.get_current_index();

        //     for (i, value) in self.values.iter().enumerate() {
        //         let v = &value.value;
        //         let label = &value.label;

        //         if current == i {
        //             context.set_item_default_focus();
        //         }

        //         let clicked = context
        //             .selectable_config(label)
        //             .selected(current == i)
        //             .build();

        //         if clicked {
        //             self.set_current_index(i);
        //         }
        //     }
        // }
        // println!(
        //     "selected {:?}",
        //     DROPDOWN_SELECTED_MAP.with_borrow(|m| m.clone())
        // );
    }
}

// impl UiNode for DropdownBox {
//     fn get_width(&self) -> f32 {
//         self.inner.get_width()
//     }

//     fn get_height(&self) -> f32 {
//         self.inner.get_height()
//     }

//     fn get_border(&self) -> Border {
//         self.inner.get_border()
//     }

//     fn build(
//         &self,
//         context: &imgui::Ui,
//         draw_list: &imgui::DrawListMut,
//         cascading_override: Override,
//     ) {
//         self.inner.build(context, draw_list, cascading_override);
//     }
// }

// pub trait DropdownChild {
//     // type Item: PartialEq;
//     fn dropdown<T: PartialEq + 'static>(&mut self, dropdown: Dropdown<T>) -> &mut Dropdown<T>;
// }

// pub trait ErasedDropdown: std::any::Any + UiNode {
//     fn as_any(&self) -> &dyn std::any::Any;
//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// impl<T: 'static + PartialEq> ErasedDropdown for Dropdown<T> {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

// // impl<T> TypeErasedUiNode for Dropdown<T> {}

// // pub trait TypeErasedUiNode: UiNode + ErasedDropdown {}

// // pub struct DropdownBox {
// //     pub inner: Box<dyn ErasedDropdown>,
// // }
// pub struct DropdownBox {
//     pub inner: Box<dyn ErasedDropdown>,
// }

// impl DropdownBox {
//     pub fn new<T: PartialEq + 'static>(dropdown: Dropdown<T>) -> Self {
//         Self {
//             inner: Box::new(dropdown),
//         }
//     }

//     pub fn downcast_mut<T: PartialEq + 'static>(&mut self) -> Option<&mut Dropdown<T>> {
//         self.inner.as_any_mut().downcast_mut::<Dropdown<T>>()
//     }
// }

// impl Deref for DropdownBox {
//     type Target = dyn UiNode;

//     fn deref(&self) -> &Self::Target {
//         &*self.inner
//     }
// }

// impl DerefMut for DropdownBox {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut *self.inner
//     }
// }

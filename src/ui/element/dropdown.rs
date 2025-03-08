use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use bevy::{color::LinearRgba, reflect::Type};

use crate::ui::{apply_button_color, clear_button_color, UiColor};

use super::{Border, Override, UiElement, UiNode};

thread_local! {
    static DROPDOWN_SELECTED_MAP: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());
}

thread_local! {
    static DROPDOWN_ID_INCR: AtomicUsize = AtomicUsize::new(0);
}

#[derive(PartialEq, Debug, Clone)]
pub struct DropdownValue<T> {
    pub value: T,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct Dropdown<T>
where
    T: PartialEq,
{
    pub id: usize,
    pub width: f32,
    pub height: f32,
    pub border: Border,
    pub background: UiColor,
    pub label: String,
    pub on_select: OnSelectCallback<T>,
    pub values: Vec<DropdownValue<T>>,
}

impl Dropdown<()> {
    pub fn id_that_will_not_work_in_immediate_mode_oopsies() -> usize {
        DROPDOWN_ID_INCR.with(|incr| incr.fetch_add(1, Ordering::SeqCst))
    }

    pub fn manual_id(id: usize) -> usize {
        id
    }
}

impl<T: PartialEq + Clone> Dropdown<T> {
    pub fn from_values(&mut self, values: Vec<DropdownValue<T>>) -> &mut Self {
        self.values = values;
        self
    }

    pub fn add_value(&mut self, value: T, label: String) -> &mut Self {
        self.values.push(DropdownValue { value, label });
        self
    }

    pub fn get_selected(&self) -> &DropdownValue<T> {
        &self.values[self.get_current_index()]
    }

    pub fn get_value(&self) -> &T {
        &self.values[self.get_current_index()].value
    }

    pub fn get_value_copy(&self) -> T {
        self.values[self.get_current_index()].value.clone()
    }

    pub fn get_label(&self) -> &String {
        &self.values[self.get_current_index()].label
    }

    pub fn set_current_index(&self, index: usize) {
        DROPDOWN_SELECTED_MAP.with_borrow_mut(|map| map.insert(self.id, index));
    }

    pub fn get_current_index(&self) -> usize {
        DROPDOWN_SELECTED_MAP.with_borrow(|map| *map.get(&self.id).unwrap_or(&0))
    }
}

impl<T: PartialEq> Default for Dropdown<T> {
    fn default() -> Self {
        Self {
            id: Dropdown::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: 120.0,
            height: 48.0,
            border: Border {
                size: 0.0,
                color: UiColor::from(LinearRgba::BLACK),
            },
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_select: OnSelectCallback(None),
            values: Vec::new(),
        }
    }
}

impl<T: PartialEq + Clone> UiNode for Dropdown<T> {
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
        if self.values.is_empty() {
            return;
        }

        let width = match cascading_override.width {
            Some(width) => width,
            None => self.width,
        };

        let height = match cascading_override.height {
            Some(height) => height,
            None => self.height,
        };

        let w = context.push_item_width(width);

        if let Some(cb) = context.begin_combo(self.label.clone(), self.get_label()) {
            let current: usize = self.get_current_index();

            for (i, value) in self.values.iter().enumerate() {
                let v = &value.value;
                let label = &value.label;

                if current == i {
                    context.set_item_default_focus();
                }

                let clicked = context
                    .selectable_config(label)
                    .size([self.width, self.height / self.values.len() as f32])
                    .selected(current == i)
                    .build();

                if clicked {
                    self.set_current_index(i);

                    if let Some(callback) = &self.on_select.0 {
                        callback(v.clone());
                    }
                }
            }

            cb.end();
        }

        w.end();
        println!(
            "selected {:?}",
            DROPDOWN_SELECTED_MAP.with_borrow(|m| m.clone())
        );
    }
}

impl UiNode for DropdownBox {
    fn get_width(&self) -> f32 {
        self.inner.get_width()
    }

    fn get_height(&self) -> f32 {
        self.inner.get_height()
    }

    fn get_border(&self) -> Border {
        self.inner.get_border()
    }

    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        cascading_override: Override,
    ) {
        self.inner.build(context, draw_list, cascading_override);
    }
}

pub trait DropdownChild {
    fn dropdown<T: 'static + PartialEq + Clone>(
        &mut self,
        dropdown: Dropdown<T>,
    ) -> &mut Dropdown<T>;
}

pub trait ErasedDropdown: std::any::Any + UiNode {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + PartialEq + Clone> ErasedDropdown for Dropdown<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct DropdownBox {
    pub inner: Rc<dyn ErasedDropdown>,
}

impl DropdownBox {
    pub fn new<T: 'static + PartialEq + Clone>(dropdown: Dropdown<T>) -> Self {
        Self {
            inner: Rc::new(dropdown),
        }
    }

    pub fn downcast_mut<T: PartialEq + 'static>(&mut self) -> Option<&mut Dropdown<T>> {
        // self.inner.as_any_mut().downcast_mut::<Dropdown<T>>()

        Rc::get_mut(&mut self.inner)
            .and_then(|inner| inner.as_any_mut().downcast_mut::<Dropdown<T>>())
    }
}

impl fmt::Debug for DropdownBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DropdownBox")
    }
}

#[derive(Clone)]
pub struct OnSelectCallback<T>(pub Option<Arc<dyn Fn(T)>>);

impl<T> fmt::Debug for OnSelectCallback<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OnSelect Closure")
    }
}

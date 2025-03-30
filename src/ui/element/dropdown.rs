use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    fmt,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use bevy::color::LinearRgba;

use crate::ui::UiColor;

use super::{Border, Builder, Computed, Size, UiElement, UiElementType, UiNode};

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
    pub width: Size,
    pub height: Size,
    pub border: Option<Border>,
    pub background: UiColor,
    pub label: String,
    pub on_select: OnSelectCallback<T>,
    pub values: Vec<DropdownValue<T>>,
    pub computed_width: Option<f32>,
    pub computed_height: Option<f32>,
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
    pub fn new() -> Self {
        Self {
            id: Dropdown::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: Size::Pixels(120.0),
            height: Size::Pixels(120.0),
            border: None,
            background: UiColor::from(LinearRgba::BLACK),
            label: "Dropdown".to_string(),
            on_select: OnSelectCallback(None),
            values: Vec::new(),
            computed_width: None,
            computed_height: None,
        }
    }

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

    pub fn width(&mut self, v: Size) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: Size) -> &mut Self {
        self.height = v;
        self
    }

    pub fn label(&mut self, v: String) -> &mut Self {
        self.label = v;
        self
    }

    pub fn border(&mut self, v: Border) -> &mut Self {
        self.border = Some(v);
        self
    }

    pub fn background(&mut self, v: UiColor) -> &mut Self {
        self.background = v;
        self
    }
}

impl<T: PartialEq> Drop for Dropdown<T> {
    fn drop(&mut self) {
        DROPDOWN_ID_INCR.with(|incr| {
            let _ = incr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| {
                if v == 0 {
                    None
                } else {
                    Some(0)
                }
            });
        });
    }
}

impl<T: PartialEq> Default for Dropdown<T> {
    fn default() -> Self {
        Self {
            id: Dropdown::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: Size::Pixels(120.0),
            height: Size::Pixels(120.0),
            border: None,
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_select: OnSelectCallback(None),
            values: Vec::new(),
            computed_width: None,
            computed_height: None,
        }
    }
}

impl<T: PartialEq> Computed for Dropdown<T> {
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
}

impl<T: PartialEq + Clone> UiNode for Dropdown<T> {
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
        None
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>> {
        None
    }

    fn get_type(&self) -> UiElementType {
        UiElementType::Dropdown
    }

    fn build(&self, context: &imgui::Ui, _draw_list: &imgui::DrawListMut) {
        if self.values.is_empty() {
            return;
        }

        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        let cursor = context.cursor_screen_pos();

        let item_width_token = context.push_item_width(width);

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
                    .size([width, height / self.values.len() as f32])
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

        item_width_token.end();

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

impl UiNode for DropdownBox {
    fn get_width(&self) -> &Size {
        self.inner.get_width()
    }

    fn get_height(&self) -> &Size {
        self.inner.get_height()
    }

    fn get_border(&self) -> Option<Border> {
        self.inner.get_border()
    }

    fn get_children(&self) -> Option<&Vec<UiElement>> {
        self.inner.get_children()
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>> {
        Rc::get_mut(&mut self.inner).unwrap().get_children_mut()
    }

    fn get_type(&self) -> UiElementType {
        self.inner.get_type()
    }

    fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut) {
        self.inner.build(context, draw_list);
    }
}

impl Computed for DropdownBox {
    fn get_computed_width(&self) -> Option<f32> {
        self.inner.get_computed_width()
    }

    fn set_computed_width(&mut self, new_width: f32) {
        Rc::get_mut(&mut self.inner)
            .unwrap()
            .set_computed_width(new_width);
    }

    fn get_computed_height(&self) -> Option<f32> {
        self.inner.get_computed_height()
    }

    fn set_computed_height(&mut self, new_height: f32) {
        Rc::get_mut(&mut self.inner)
            .unwrap()
            .set_computed_height(new_height);
    }
}

pub trait DropdownChild: Builder {
    fn dropdown<T: 'static + PartialEq + Clone>(&mut self) -> &mut Dropdown<T> {
        let dropdown_box = DropdownBox::new(Dropdown::<T>::new());
        let children = self.parent().get_children_mut().unwrap();

        children.push(UiElement::Dropdown(dropdown_box));

        match children.last_mut().unwrap() {
            UiElement::Dropdown(dropdown_box) => dropdown_box.downcast_mut::<T>().unwrap(),
            _ => unreachable!("Dropdown not dropdowning :("),
        }
    }
}

pub trait ErasedDropdown: Any + UiNode + Computed {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + PartialEq + Clone> ErasedDropdown for Dropdown<T> {
    fn as_any(&self) -> &dyn Any {
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

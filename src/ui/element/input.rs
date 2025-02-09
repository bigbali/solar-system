use std::{
    any::Any,
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use bevy::{color::LinearRgba, scene::ron::value};

use crate::ui::UiColor;

use super::{Border, Builder, Computed, HasChildren, Size, UiElement, UiElementType, UiNode};

thread_local! {
    static DROPDOWN_SELECTED_MAP: RefCell<HashMap<usize, i32>> = RefCell::new(HashMap::new());
}

thread_local! {
    static DROPDOWN_ID_INCR: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, Clone)]
// TODO Input<T> where T: InputType::I32
pub struct InputI32 {
    id: usize,
    width: Size,
    height: Size,
    border: Option<Border>,
    background: UiColor,
    label: String,
    on_change: OnChangeCallback<i32>,
    step: i32,
    default_value: i32,
    computed_width: Option<f32>,
    computed_height: Option<f32>,
}

impl InputI32 {
    pub fn new() -> Self {
        Self::default()
    }

    // Even though we reset the id's when dropped, we will still face issues if the order of rendering the elements changes.
    // Thus: TODO: generate constant id via macros, maybe using the place where the element is created (line, col).
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

    pub fn step(&mut self, v: i32) -> &mut Self {
        self.step = v;
        self
    }

    pub fn default_value(&mut self, v: i32) -> &mut Self {
        self.default_value = v;
        self
    }

    pub fn label(&mut self, v: String) -> &mut Self {
        self.label = v;
        self
    }
}

impl Drop for InputI32 {
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

impl Default for InputI32 {
    fn default() -> Self {
        Self {
            id: InputI32::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: Size::Pixels(120.0),
            height: Size::Pixels(48.0),
            border: None,
            background: UiColor::from(LinearRgba::BLACK),
            label: "Button".to_string(),
            on_change: OnChangeCallback(None),
            step: 1,
            default_value: 0,
            computed_width: None,
            computed_height: None,
        }
    }
}

impl Computed for InputI32 {
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

    fn compute_children_size(&mut self, _parent_properties: &super::ParentProperties) {}
}

impl UiNode for InputI32 {
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
        UiElementType::InputI32
    }

    fn build(&self, context: &imgui::Ui, _draw_list: &imgui::DrawListMut) {
        let mut value = self.get_value();

        // Since we cannot get a direct mutable reference with a long enough lifetime,
        // let's just get the value, update it, and then set it back.
        if context
            .input_int(&self.label, &mut value)
            .step(self.step)
            .build()
        {
            if let Some(callback) = &self.on_change.0 {
                callback(value);
            }

            self.set_value(value);
        }

        println!(
            "map: {:#?}",
            DROPDOWN_SELECTED_MAP.with_borrow(|v| v.clone())
        );
    }
}

pub trait InputI32Child: Builder {
    fn input_i32(&mut self) -> &mut InputI32 {
        let maybe_children = self.parent().get_children_mut();

        assert!(
            maybe_children.is_some(),
            "Parent of builder has no children"
        );

        let children = maybe_children.unwrap();
        children.push(UiElement::InputI32(InputI32::new()));

        match children.last_mut().unwrap() {
            UiElement::InputI32(input) => input,
            _ => unreachable!("InputI32 not inputing :("),
        }
    }
}

#[derive(Clone)]
pub struct OnChangeCallback<T>(pub Option<Arc<dyn Fn(T)>>);

impl<T> fmt::Debug for OnChangeCallback<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OnChange Closure")
    }
}

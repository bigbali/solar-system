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
use imgui::{ColorStackToken, StyleStackToken};

use crate::ui::UiColor;

use super::{
    rect::Rect, Border, Builder, Computed, HasChildren, Size, UiElement, UiElementType, UiNode,
};

thread_local! {
    static DROPDOWN_SELECTED_MAP: RefCell<HashMap<usize, i32>> = RefCell::new(HashMap::new());
}

thread_local! {
    static DROPDOWN_ID_INCR: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, Clone, Copy)]
enum LabelPlacement {
    Left,
    Right,
    Top,
}

// pub enum InputType {
//     I32(InputI32),
//     F32(InputF32),
//     String(InputString),
// }

// pub struct InputI32Multiple;

// pub struct Input {
//     input: InputType,
// }

#[derive(Debug, Clone)]
// TODO Input<T> where T: InputType::I32
pub struct InputI32 {
    id: usize,
    width: Size,
    height: Size,
    border: Option<Border>,
    background: Option<UiColor>,
    label: &'static str,
    label_alignment: LabelPlacement,
    on_change: OnChangeCallback<i32>,
    step: i32,
    default_value: i32,
    computed_width: Option<f32>,
    computed_height: Option<f32>,
}

impl InputI32 {
    const INPUT_FIELD_MIN_WIDTH: f32 = 16.0;
    const INPUT_FIELD_MIN_HEIGHT: f32 = 12.0;

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

    pub fn width(&mut self, v: Size) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: Size) -> &mut Self {
        self.height = v;
        self
    }

    pub fn step(&mut self, v: i32) -> &mut Self {
        self.step = v;
        self
    }

    pub fn default_value(&mut self, v: i32) -> &mut Self {
        self.default_value = v;
        self
    }

    pub fn label(&mut self, v: &'static str) -> &mut Self {
        self.label = v;
        self
    }

    pub fn background(&mut self, v: UiColor) -> &mut Self {
        self.background = Some(v);
        self
    }
}

impl<'a> Drop for InputI32 {
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

impl<'a> Default for InputI32 {
    fn default() -> Self {
        Self {
            id: InputI32::id_that_will_not_work_in_immediate_mode_oopsies(),
            width: Size::Pixels(120.0),
            height: Size::Pixels(48.0),
            border: None,
            background: None,
            label: "Button",
            label_alignment: LabelPlacement::Top,
            on_change: OnChangeCallback(None),
            step: 1,
            default_value: 0,
            computed_width: None,
            computed_height: None,
        }
    }
}

impl<'a> Computed for InputI32 {
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

    fn calculate_min_width(&self, context: &imgui::Ui) -> Option<f32> {
        match self.label_alignment {
            LabelPlacement::Left | LabelPlacement::Right => {
                Some(context.calc_text_size(&self.label)[0] + Self::INPUT_FIELD_MIN_WIDTH)
            }
            LabelPlacement::Top => Some(Self::INPUT_FIELD_MIN_WIDTH),
        }
    }

    fn calculate_min_height(&self, context: &imgui::Ui) -> Option<f32> {
        match self.label_alignment {
            LabelPlacement::Left | LabelPlacement::Right => Some(Self::INPUT_FIELD_MIN_HEIGHT),
            LabelPlacement::Top => {
                Some(context.calc_text_size(&self.label)[1] + Self::INPUT_FIELD_MIN_HEIGHT)
            }
        }
    }
}

impl<'a> UiNode for InputI32 {
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
        assert!(self.computed_width.is_some(), "Computed width is unset.");
        assert!(self.computed_height.is_some(), "Computed height is unset.");

        let width = self.computed_width.unwrap();
        let height = self.computed_height.unwrap();

        let mut value = self.get_value();

        let cursor = context.cursor_screen_pos();

        Rect::draw(
            context,
            _draw_list,
            cursor,
            [
                cursor[0] + self.computed_width.unwrap(),
                cursor[1] + self.computed_height.unwrap(),
            ],
            self.background,
            self.border,
        );

        let ax = context.push_item_width(width);
        let mut style_stack: Vec<StyleStackToken> = Vec::new();

        let x = context.clone_style().frame_border_size;

        // style_stack.push(context.push_style_var(imgui::StyleVar::FramePadding([
        //     4.0,
        //     (height - context.current_font_size()) / 2.0 - x * 2.0,
        // ])));

        match self.label_alignment {
            LabelPlacement::Left => {
                context.text(&self.label);
                context.same_line();
            }
            LabelPlacement::Top => {
                context.text(&self.label);
            }
            _ => {}
        }

        let label = match self.label_alignment {
            LabelPlacement::Right => self.label.to_string(),
            _ => format!("##{}", &self.label),
        };

        // Since we cannot get a direct mutable reference with a long enough lifetime,
        // let's just get the value, update it, and then set it back.
        if context.input_int(label, &mut value).step(self.step).build() {
            if let Some(callback) = &self.on_change.0 {
                callback(value);
            }

            self.set_value(value);
        }

        ax.end();

        context.set_cursor_screen_pos([cursor[0] + width, cursor[1] + height]);
    }
}

pub trait InputI32Child: Builder {
    fn input_i32(&mut self) -> &mut InputI32 {
        let children = self.parent().get_children_mut().unwrap();

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

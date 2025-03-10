use std::rc::Weak;

use button::Button;
use delegate::delegate;
use dropdown::{Dropdown, DropdownBox, ErasedDropdown};
use flex::Flex;

use super::UiColor;

pub mod button;
pub mod dropdown;
pub mod flex;
pub mod input;
pub mod root;
pub mod window;

pub trait UiNode {
    /// Returns the width of the element.
    /// Do note that this may be different from the final computed width of the element.
    fn get_width(&self) -> &Size;

    /// Returns the height of the element.
    /// Do note that this may be different from the final computed height of the element.
    fn get_height(&self) -> &Size;

    fn get_border(&self) -> Border;
    fn get_children(&self) -> Option<&Vec<UiElement>>;
    fn get_type(&self) -> UiElementType;
    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        // cascading_override: Override,
    );
}

pub trait Computed {
    /// Returns the actual computed width of the element.
    /// Available only after the element has been built.
    fn get_computed_width(&self) -> Option<f32>;
    fn set_computed_width(&mut self, new_width: f32);

    /// Returns the actual computed height of the element.
    /// Available only after the element has been built.
    fn get_computed_height(&self) -> Option<f32>;
    fn set_computed_height(&mut self, new_height: f32);

    /// Recursively calculates the size of the element's children,
    /// so we already have the calculated size of the elements when we build them.\
    /// This simplified our rendering logic, and that tastes like candy.\
    /// Must be invoked by the root node.\
    /// It mutates the element's children.
    /// Do consider that the element's own sizes are calculated by its parent.
    fn compute_children_size(&mut self, parent_properties: &ParentProperties);
}

pub struct ParentProperties<'a> {
    computed_width: Option<f32>,
    computed_height: Option<f32>,
    width_sizing: &'a Size,
    height_sizing: &'a Size,
    padding: f32,
}

pub struct SizeOverride {
    width: Option<f32>,
    height: Option<f32>,
}

// #[derive(Debug, Default, Clone)]
// pub struct Override
// </* 'a */>
// {
//     width: Option<f32>,
//     height: Option<f32>,
//     parent_element: Option<Weak<UiElement>>,
//     custom_rendering: bool,
// }

#[derive(Debug, Clone)]
pub enum Size {
    Pixels(f32),
    Percentage(f32),
    // FitContent, // Don't really need this in this here project. It's also bloody hard to implement.
    /// Allow dynamically resizing, for example in a flex container when it's
    /// alignment is set to stretch.
    Auto,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Border {
    pub size: f32,
    pub color: UiColor,
}

pub enum UiElementType {
    Flex,
    Button,
    Dropdown,
    InputI32,
    InputF32,
    InputString,
}

#[derive(Debug, Clone)]
pub enum UiElement {
    Flex(Flex),
    Button(Button),
    Dropdown(DropdownBox),
}

impl UiNode for UiElement {
    delegate! {
        to match self {
            UiElement::Flex(f) => f,
            UiElement::Button(b) => b,
            UiElement::Dropdown(d) => d,
        } {
            fn get_width(&self) -> &Size;
            fn get_height(&self) -> &Size;
            fn get_border(&self) -> Border;
            fn get_children(&self) -> Option<&Vec<UiElement>>;
            fn get_type(&self) -> UiElementType;
            fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut, /* cascading_override: Override */);
        }
    }
}

impl Computed for UiElement {
    delegate! {
        to match self {
            UiElement::Flex(f) => f,
            UiElement::Button(b) => b,
            UiElement::Dropdown(d) => d,
        } {
            fn get_computed_width(&self) -> Option<f32>;
            fn set_computed_width(&mut self, new_width: f32);
            fn get_computed_height(&self) -> Option<f32>;
            fn set_computed_height(&mut self, new_height: f32);
            fn compute_children_size(&mut self, parent_properties: &ParentProperties);
        }
    }
}

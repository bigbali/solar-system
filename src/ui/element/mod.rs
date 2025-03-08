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
    fn get_children(&self) -> &Vec<UiElement>;
    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        cascading_override: Override,
    );
}

pub trait Computed {
    /// Returns the actual computed width of the element.
    /// Available only after the element has been built.
    fn get_computed_width(&self) -> Option<f32>;

    /// Returns the actual computed height of the element.
    /// Available only after the element has been built.
    fn get_computed_height(&self) -> Option<f32>;
    fn compute_size(&self, parent_properties: &ParentProperties) -> (f32, f32);
}

pub struct ParentProperties<'a> {
    computed_width: Option<f32>,
    computed_height: Option<f32>,
    width_sizing: &'a Size,
    height_sizing: &'a Size,
}

pub struct SizeOverride {
    width: Option<f32>,
    height: Option<f32>,
}

#[derive(Debug, Default, Clone)]
pub struct Override
</* 'a */>
{
    width: Option<f32>,
    height: Option<f32>,
    parent_element: Option<Weak<UiElement>>,
    custom_rendering: bool,
}
// #[derive(Debug, Default, Clone)]
// pub struct Override</* 'a */> {
//     width: Option<f32>,
//     height: Option<f32>,
//     parent_element: Option<Weak<UiElement>>,
//     custom_rendering: bool,
// }

pub enum Size {
    Pixels(f32),
    Percentage(f32),
    FitContent,
    FillAvailable,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Border {
    pub size: f32,
    pub color: UiColor,
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
            fn get_children(&self) -> &Vec<UiElement>;
            fn get_border(&self) -> Border;
            fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut, cascading_override: Override);
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
            fn get_computed_height(&self) -> Option<f32>;
            fn compute_size(&self, parent_properties: &ParentProperties) -> (f32, f32);
        }
    }
}

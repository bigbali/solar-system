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
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_border(&self) -> Border;
    fn build(
        &self,
        context: &imgui::Ui,
        draw_list: &imgui::DrawListMut,
        cascading_override: Override,
    );
}

#[derive(Debug, Default, Clone)]
pub struct Override {
    width: Option<f32>,
    height: Option<f32>,
    custom_rendering: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Border {
    pub size: f32,
    pub color: UiColor,
}

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
            fn get_width(&self) -> f32;
            fn get_height(&self) -> f32;
            fn get_border(&self) -> Border;
            fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut, cascading_override: Override);
        }
    }
}

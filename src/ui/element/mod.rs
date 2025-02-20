use bevy::color::Color;
use button::Button;
use delegate::delegate;
use flex::Flex;

use super::UiColor;

pub mod button;
pub mod flex;
pub mod root;

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
}

impl UiNode for UiElement {
    delegate! {
        to match self {
            UiElement::Flex(f) => f,
            UiElement::Button(b) => b,
        } {
            fn get_width(&self) -> f32;
            fn get_height(&self) -> f32;
            fn get_border(&self) -> Border;
            fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut, cascading_override: Override);
        }
    }
}

use button::Button;
use delegate::delegate;
use dropdown::DropdownBox;
use flex::Flex;
use input::InputI32;
use rect::Rect;
use text::Text;

use super::UiColor;

pub mod button;
pub mod dropdown;
pub mod flex;
pub mod input;
pub mod rect;
pub mod root;
pub mod text;
pub mod window;

pub trait BasicUiNode {
    fn get_self(&self) -> &impl UiNode;
}

pub trait UiNode {
    /// Returns the width of the element.
    /// Do note that this may be different from the final computed width of the element.
    fn get_width(&self) -> &Size;

    /// Returns the height of the element.
    /// Do note that this may be different from the final computed height of the element.
    fn get_height(&self) -> &Size;

    fn get_border(&self) -> Option<Border>;

    /// Returns the children of the element, if the element has children.
    /// Do note that even if Some(...) is returned, the children vector may still be empty.
    fn get_children(&self) -> Option<&Vec<UiElement>>;
    fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>>;
    fn get_type(&self) -> UiElementType;
    fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut);
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
    /// TODO: actually, why is it not its own trait if it needs to be invoked by the root node?
    fn compute_children_size(&mut self, parent_properties: &ParentProperties);
}

pub trait HasChildren {
    fn children(&mut self) -> &mut Vec<UiElement>;
}

pub trait Builder {
    fn parent(&mut self) -> &mut impl UiNode;
}

pub struct ParentProperties<'a> {
    computed_width: Option<f32>,
    computed_height: Option<f32>,
    width_sizing: &'a Size,
    height_sizing: &'a Size,
    padding: f32,
}

#[derive(Debug, Clone)]
pub enum Size {
    Pixels(f32),
    Percentage(f32),
    // FitContent, // Don't really need this in this here project.
    /// Allow dynamically resizing to fill available space.
    Auto,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Border {
    pub size: f32,
    pub color: UiColor,
}

pub enum UiElementType {
    Flex,
    Text,
    Button,
    Dropdown,
    InputI32,
    InputF32,
    InputString,
    Rect,
}

#[derive(Debug, Clone)]
pub enum UiElement {
    Flex(Flex),
    Text(Text),
    Button(Button),
    Dropdown(DropdownBox),
    InputI32(InputI32),
    // InputF32(InputF32),
    // InputString(InputString),
    Rect(Rect),
}

impl UiNode for UiElement {
    delegate! {
        to match self {
            UiElement::Flex(f) => f,
            UiElement::Text(t) => t,
            UiElement::Button(b) => b,
            UiElement::Dropdown(d) => d,
            UiElement::InputI32(i) => i,
            UiElement::Rect(r) => r,
        } {
            fn get_width(&self) -> &Size;
            fn get_height(&self) -> &Size;
            fn get_border(&self) -> Option<Border>;
            fn get_children(&self) -> Option<&Vec<UiElement>>;
            fn get_children_mut(&mut self) -> Option<&mut Vec<UiElement>>;
            fn get_type(&self) -> UiElementType;
            fn build(&self, context: &imgui::Ui, draw_list: &imgui::DrawListMut, /* cascading_override: Override */);
        }
    }
}

impl Computed for UiElement {
    delegate! {
        to match self {
            UiElement::Flex(f) => f,
            UiElement::Text(t) => t,
            UiElement::Button(b) => b,
            UiElement::Dropdown(d) => d,
            UiElement::InputI32(i) => i,
            UiElement::Rect(r) => r,
        } {
            fn get_computed_width(&self) -> Option<f32>;
            fn set_computed_width(&mut self, new_width: f32);
            fn get_computed_height(&self) -> Option<f32>;
            fn set_computed_height(&mut self, new_height: f32);
            fn compute_children_size(&mut self, parent_properties: &ParentProperties);
        }
    }
}

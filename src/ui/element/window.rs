use bevy::color::LinearRgba;
use imgui::Condition;

use crate::ui::{
    util::{id, with_color_scheme},
    UiColor,
};

use super::{
    flex::Flex, Border, Computed, Override, ParentProperties, Size, SizeOverride, UiElement, UiNode,
};

pub struct WindowBuilder<'a> {
    window: &'a mut Window,
}

impl<'a> WindowBuilder<'a> {
    pub fn flex_default(&mut self) -> &mut Flex {
        self.window.children.push(UiElement::Flex(Flex::default()));

        match self.window.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }

    pub fn flex(&mut self, flex: Flex) -> &mut Flex {
        self.window.children.push(UiElement::Flex(flex));

        match self.window.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

// pub enum WindowPositionType {
//     Fixed,
//     Movable,
// }

#[derive(PartialEq, Debug, Clone)]
pub enum WindowPlacementAlignTo {
    Start,
    Center,
    End,
}

#[derive(PartialEq, Debug, Clone)]
pub enum WindowPlacement {
    Manual(f32),
    AlignTo(WindowPlacementAlignTo),
}

impl Default for WindowPlacement {
    fn default() -> Self {
        Self::AlignTo(WindowPlacementAlignTo::Start)
    }
}

#[derive(Debug, Clone)]
pub struct WindowPosition {
    pub x: WindowPlacement,
    pub y: WindowPlacement,
}

#[derive(Debug, Clone)]
pub enum WindowDimension {
    Fixed(f32),
    Percentage(f32),
    Stretch,
}

impl Default for WindowDimension {
    fn default() -> Self {
        Self::Fixed(800.0)
    }
}

pub struct Window {
    title: String,
    title_bar: bool,
    border: Border,
    displayed: bool,
    fixed: bool,
    resizable: bool,
    background: UiColor,
    // pub width: Size,
    // pub height: Size,
    width: WindowDimension,
    height: WindowDimension,
    position: WindowPosition,
    padding: f32,
    children: Vec<UiElement>,
    computed_width: f32,
    computed_height: f32,
}

// impl UiNode for Window {
//     fn get_width(&self) -> Size {
//         self.width
//     }

//     fn get_height(&self) -> Size {
//         self.height
//     }

//     fn get_border(&self) -> Border {
//         self.border
//     }

//     fn build(
//         &self,
//         context: &imgui::Ui,
//         draw_list: &imgui::DrawListMut,
//         cascading_override: Override,
//     ) {
//         self.build_window(context, draw_list, cascading_override, self);
//     }
// }

impl Window {
    fn compute_initial_size(
        bevy_window: bevy::window::Window,
        width: WindowDimension,
        height: WindowDimension,
    ) -> (f32, f32) {
        let bevy_window_width = bevy_window.width();
        let bevy_window_height = bevy_window.height();

        let width = match width {
            WindowDimension::Fixed(width) => width,
            WindowDimension::Percentage(percentage) => bevy_window_width * (percentage / 100.0),
            WindowDimension::Stretch => bevy_window_width,
        };

        let height = match height {
            WindowDimension::Fixed(height) => height,
            WindowDimension::Percentage(percentage) => bevy_window_height * (percentage / 100.0),
            WindowDimension::Stretch => bevy_window_height,
        };

        return (width, height);
    }

    fn compute_children_size(&mut self) {
        let self_properties = ParentProperties {
            computed_width: Some(self.computed_width),
            computed_height: Some(self.computed_height),

            // Since the sizes are already known before we start computing the children,
            // we can set them to be fixed.
            width_sizing: &Size::Pixels(self.computed_width),
            height_sizing: &Size::Pixels(self.computed_height),
        };

        for child in self.children.iter_mut() {
            child.compute_size(&self_properties);
        }
    }
}

impl Window {
    pub fn new(
        bevy_window: bevy::window::Window,
        width: WindowDimension,
        height: WindowDimension,
    ) -> Self {
        let (w, h) = Self::compute_initial_size(bevy_window, width, height);

        Self {
            children: Vec::new(),
            title: id(),
            title_bar: true,
            border: Border::default(),
            displayed: true,
            resizable: false,
            fixed: true,
            background: UiColor::from(LinearRgba::new(0.1, 0.1, 0.1, 1.0)),
            width: WindowDimension::Fixed(800.0),
            height: WindowDimension::Fixed(600.0),
            position: WindowPosition {
                x: WindowPlacement::Manual(0.0),
                y: WindowPlacement::Manual(0.0),
            },
            padding: 8.0,
            computed_width: w,
            computed_height: h,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn title_bar(&mut self, v: bool) -> &mut Self {
        self.title_bar = v;
        self
    }

    pub fn displayed(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn width(&mut self, v: WindowDimension) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: WindowDimension) -> &mut Self {
        self.height = v;
        self
    }

    pub fn position(&mut self, v: WindowPosition) -> &mut Self {
        self.position = v;
        self
    }

    pub fn children(&mut self, f: impl FnOnce(&mut WindowBuilder)) -> &mut Self {
        let mut builder = WindowBuilder { window: self };
        f(&mut builder);
        self
    }

    pub fn build_window(&mut self, context: &imgui::Ui, bevy_window: &bevy::window::Window) {
        with_color_scheme(context, || {
            let bevy_window_width = bevy_window.width();
            let bevy_window_height = bevy_window.height();

            self.compute_children_size();

            let (width, height) = (self.computed_width, self.computed_height);

            let position_x = match &self.position.x {
                WindowPlacement::Manual(x) => *x,
                WindowPlacement::AlignTo(align_to) => match align_to {
                    WindowPlacementAlignTo::Start => 0.0,
                    WindowPlacementAlignTo::Center => bevy_window_width / 2.0 - width / 2.0,
                    WindowPlacementAlignTo::End => bevy_window_width - width,
                },
            };

            let position_y = match &self.position.y {
                WindowPlacement::Manual(y) => *y,
                WindowPlacement::AlignTo(align_to) => match align_to {
                    WindowPlacementAlignTo::Start => 0.0,
                    WindowPlacementAlignTo::Center => bevy_window_height / 2.0 - height / 2.0,
                    WindowPlacementAlignTo::End => bevy_window_height - height,
                },
            };

            let position_condition = match self.fixed {
                true => imgui::Condition::Always,
                false => imgui::Condition::FirstUseEver,
            };

            let size_condition = match self.resizable {
                false => imgui::Condition::Always,
                true => imgui::Condition::FirstUseEver,
            };

            println!(
                "position_x: {}, position_y: {}, width: {}, height: {}, bevy height: {}",
                position_x, position_y, width, height, bevy_window_height
            );

            let style_stack = vec![context
                .push_style_var(imgui::StyleVar::WindowPadding([self.padding, self.padding]))];

            let c = context.push_style_color(imgui::StyleColor::WindowBg, self.background);

            let ax = context
                .window(self.title.clone())
                .title_bar(self.title_bar)
                .position([position_x, position_y], position_condition)
                .size([width, height], size_condition)
                .movable(!self.fixed)
                .resizable(self.resizable)
                .begin();

            let draw_list = context.get_window_draw_list();

            for (i, child) in self.children.iter_mut().enumerate() {
                child.build(&context, &draw_list, Override::default());
            }

            for style in style_stack {
                style.pop();
            }

            c.pop();

            ax.unwrap().end();
        });
    }
}

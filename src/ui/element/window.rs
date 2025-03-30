use bevy::{color::LinearRgba, log::error, log::warn};
use imgui::Condition;

use crate::ui::{
    util::{id, with_color_scheme},
    UiColor,
};

use super::{
    flex::{Flex, FlexChild},
    Border, Computed, ParentProperties, Size, UiElement, UiNode,
};

pub struct WindowBuilder<'a> {
    window: &'a mut UiWindow,
}

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
pub enum WindowSize {
    Pixels(f32),
    Percentage(f32),
    // Stretch, // Actually, this is effectively the same as Percentage(100.0). Bye then, good old Stretch.
}

impl Default for WindowSize {
    fn default() -> Self {
        Self::Pixels(800.0)
    }
}

pub struct UiWindow {
    title: String,
    title_bar: bool,
    border: Border,
    displayed: bool,
    movable: bool,
    resizable: bool,
    background: UiColor,
    width: WindowSize,
    height: WindowSize,
    position: WindowPosition,
    padding: f32,
    children: Vec<UiElement>,
    computed_width: f32,
    computed_height: f32,
}

impl UiWindow {
    fn compute_initial_size(
        bevy_window: &bevy::window::Window,
        width: WindowSize,
        height: WindowSize,
    ) -> (f32, f32) {
        let bevy_window_width = bevy_window.width();
        let bevy_window_height = bevy_window.height();

        let width = match width {
            WindowSize::Pixels(width) => width,
            WindowSize::Percentage(percentage) => bevy_window_width * (percentage / 100.0),
            // WindowDimension::Stretch => bevy_window_width,
        };

        let height = match height {
            WindowSize::Pixels(height) => height,
            WindowSize::Percentage(percentage) => bevy_window_height * (percentage / 100.0),
            // WindowDimension::Stretch => bevy_window_height,
        };

        return (width, height);
    }

    fn compute_children_size(&mut self, context: &imgui::Ui) {
        if self.children.len() == 0 {
            panic!("Window with title '{}' has no children.", self.title);
        }

        if self.children.len() > 1 {
            panic!("Window with title '{}' has more than 1 child. \
                    For now, only 1 child per root container is allowed, because it's not worth the time \
                    to implement layout logic for the window element. \
                    Use a single Flex child, and do your layout there.", self.title);
        }

        let self_properties = ParentProperties {
            computed_width: Some(self.computed_width),
            computed_height: Some(self.computed_height),

            // Since the sizes are already known before we start computing the children,
            // we can set them to be fixed.
            width_sizing: &Size::Pixels(self.computed_width),
            height_sizing: &Size::Pixels(self.computed_height),
            padding: self.padding,
        };

        // NOTE: we are allowing exactly 0 or 1 child for now
        for child in &mut self.children {
            child.set_computed_width(match child.get_width() {
                Size::Pixels(p) => *p,
                Size::Percentage(p) => self.computed_width * p / 100.0,
                Size::Auto => self.computed_width - (self.padding * 2.0),
            });
            child.set_computed_height(match child.get_height() {
                Size::Pixels(p) => *p,
                Size::Percentage(p) => self.computed_height * p / 100.0,
                Size::Auto => self.computed_height - (self.padding * 2.0),
            });

            child.compute_children_size(context, &self_properties);
        }
    }
}

impl UiWindow {
    pub fn new(bevy_window: &bevy::window::Window, width: WindowSize, height: WindowSize) -> Self {
        let (w, h) = Self::compute_initial_size(&bevy_window, width, height);

        Self {
            children: Vec::new(),
            title: id(),
            title_bar: true,
            border: Border::default(),
            displayed: true,
            resizable: false,
            movable: false,
            background: UiColor::from(LinearRgba::new(0.1, 0.1, 0.1, 1.0)),
            width: WindowSize::Pixels(800.0),
            height: WindowSize::Pixels(600.0),
            position: WindowPosition {
                x: WindowPlacement::Manual(0.0),
                y: WindowPlacement::Manual(0.0),
            },
            padding: 8.0,
            computed_width: w,
            computed_height: h,
        }
    }

    pub fn title(&mut self, v: String) -> &mut Self {
        self.title = v;
        self
    }

    pub fn title_bar(&mut self, v: bool) -> &mut Self {
        self.title_bar = v;
        self
    }

    pub fn displayed(&mut self, v: bool) -> &mut Self {
        self.displayed = v;
        self
    }

    pub fn width(&mut self, v: WindowSize) -> &mut Self {
        self.width = v;
        self
    }

    pub fn height(&mut self, v: WindowSize) -> &mut Self {
        self.height = v;
        self
    }

    pub fn position(&mut self, v: WindowPosition) -> &mut Self {
        self.position = v;
        self
    }

    pub fn movable(&mut self, v: bool) -> &mut Self {
        self.movable = v;
        self
    }

    pub fn resizable(&mut self, v: bool) -> &mut Self {
        self.resizable = v;
        self
    }

    pub fn background(&mut self, v: UiColor) -> &mut Self {
        self.background = v;
        self
    }

    pub fn padding(&mut self, v: f32) -> &mut Self {
        self.padding = v;
        self
    }

    pub fn children(&mut self, f: impl FnOnce(&mut WindowBuilder)) -> &mut Self {
        let mut builder = WindowBuilder { window: self };
        f(&mut builder);
        self
    }

    pub fn build(&mut self, context: &imgui::Ui, bevy_window: &bevy::window::Window) {
        with_color_scheme(context, || {
            self.compute_children_size(context);

            let bevy_window_width = bevy_window.width();
            let bevy_window_height = bevy_window.height();

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

            let position_condition = match self.movable {
                true => imgui::Condition::FirstUseEver,
                false => imgui::Condition::Always,
            };

            let size_condition = match self.resizable {
                true => imgui::Condition::FirstUseEver,
                false => imgui::Condition::Always,
            };

            let style_stack = vec![context
                .push_style_var(imgui::StyleVar::WindowPadding([self.padding, self.padding]))];

            let background_token =
                context.push_style_color(imgui::StyleColor::WindowBg, self.background);

            let window_token = context
                .window(self.title.clone())
                .title_bar(self.title_bar)
                .position([position_x, position_y], position_condition)
                .size([width, height], size_condition)
                .movable(self.movable)
                .resizable(self.resizable)
                .begin();

            let draw_list = context.get_window_draw_list();

            for (_i, child) in self.children.iter_mut().enumerate() {
                child.build(&context, &draw_list /* Override::default() */);
            }

            for style in style_stack {
                style.pop();
            }

            background_token.pop();

            window_token.unwrap().end();
        });
    }
}

impl<'a> WindowBuilder<'a> {
    pub fn flex(&mut self) -> &mut Flex {
        self.window.children.push(UiElement::Flex(Flex::new()));

        match self.window.children.last_mut().unwrap() {
            UiElement::Flex(flex) => flex,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

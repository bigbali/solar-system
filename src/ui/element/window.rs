use imgui::Condition;

use crate::ui::util::{id, with_color_scheme};

use super::{flex::Flex, Override, UiElement, UiNode};

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
    pub title: String,
    pub title_bar: bool,
    pub displayed: bool,
    pub fixed: bool,
    pub resizable: bool,
    pub width: WindowDimension,
    pub height: WindowDimension,
    pub position: WindowPosition,
    pub padding: f32,
    pub children: Vec<UiElement>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            title: id(),
            title_bar: true,
            displayed: true,
            resizable: false,
            fixed: true,
            width: WindowDimension::Fixed(800.0),
            height: WindowDimension::Fixed(600.0),
            position: WindowPosition {
                x: WindowPlacement::Manual(0.0),
                y: WindowPlacement::Manual(0.0),
            },
            padding: 8.0,
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

    pub fn build(&mut self, context: &imgui::Ui, bevy_window: &bevy::window::Window) {
        with_color_scheme(context, || {
            let draw_list = context.get_window_draw_list();
            let style_stack = vec![context
                .push_style_var(imgui::StyleVar::WindowPadding([self.padding, self.padding]))];

            let bevy_window_width = bevy_window.width();
            let bevy_window_height = bevy_window.width();

            let width = match self.width {
                WindowDimension::Fixed(width) => width,
                WindowDimension::Percentage(percentage) => bevy_window_width * percentage,
                WindowDimension::Stretch => bevy_window_width,
            };

            let height = match self.height {
                WindowDimension::Fixed(height) => height,
                WindowDimension::Percentage(percentage) => bevy_window_height * percentage,
                WindowDimension::Stretch => bevy_window_height,
            };

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
                "position_x: {}, position_y: {}, width: {}, height: {}",
                position_x, position_y, width, height
            );

            context
                .window(self.title.clone())
                .title_bar(self.title_bar)
                .position([position_x, position_y], position_condition)
                .size([width, height], size_condition)
                .build(|| {
                    for child in self.children.iter_mut() {
                        child.build(&context, &draw_list, Override::default());
                    }
                });

            for style in style_stack {
                style.pop();
            }
        });
    }
}

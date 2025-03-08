use std::{
    cell::Cell,
    collections::HashMap,
    sync::{atomic, Arc, Mutex},
};

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_mod_imgui::ImguiContext;

use crate::ui::{
    element::{
        button::{Button, ButtonChild},
        dropdown::{Dropdown, DropdownChild, DropdownValue},
        flex::{
            self, Flex, FlexAxisAlign, FlexChildSizeOverride, FlexCrossAxisAlign, FlexDirection,
        },
        root::RootNode,
        window::{
            Window as UiWindow, WindowDimension, WindowPlacement, WindowPlacementAlignTo,
            WindowPosition,
        },
        Border,
    },
    UiColor,
};

pub fn spawn_window_system(mut context: NonSendMut<ImguiContext>, windows: Query<&Window>) {
    let bevy_window = windows.single();

    let ui = context.ui();

    let mut window = UiWindow {
        title: "Spawn Window".to_string(),
        title_bar: false,
        displayed: true,
        fixed: true,
        resizable: false,
        background: UiColor::from(LinearRgba::new(0.7, 0.5, 0.4, 1.0)),
        width: WindowDimension::Percentage(50.0),
        // width: WindowDimension::Fixed(400.0),
        height: WindowDimension::Fixed(160.0),
        position: WindowPosition {
            x: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
            y: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
        },
        padding: 0.0,
        ..UiWindow::new()
    };

    window
        .children(|w| {
            w.flex_default()
                .flex_direction(flex::FlexDirection::Column)
                .fill_parent(true)
                .cross_axis_size_override(0, FlexChildSizeOverride::Percentage(100.0))
                .fill(UiColor::from(LinearRgba::new(
                    255.0 / 255.0,
                    80.0 / 255.0,
                    90.0 / 255.0,
                    1.0,
                )))
                .align_cross_axis(FlexCrossAxisAlign::Stretch)
                .align_axis(FlexAxisAlign::Start)
                .children(|f| {
                    let ff = f
                        .flex(Flex {
                            // fill_parent: true,
                            direction: FlexDirection::Column,
                            gap: 8.0,
                            axis_align_items: FlexAxisAlign::Start,
                            cross_axis_align_items: FlexCrossAxisAlign::Stretch,
                            axis_size_override: HashMap::from([
                                (0, FlexChildSizeOverride::Pixels(10.0)),
                                (1, FlexChildSizeOverride::Pixels(142.0)),
                            ]),
                            fill: Some(UiColor::from(LinearRgba::new(
                                160.0 / 255.0,
                                160.0 / 255.0,
                                255.0 / 255.0,
                                1.0,
                            ))),
                            ..default()
                        })
                        .children(|a| {
                            a.flex_default().fill(UiColor::new(Color::WHITE));
                            a.flex_default().fill(UiColor::new(Color::BLACK));
                        });
                });
        })
        .build(ui, bevy_window);
}

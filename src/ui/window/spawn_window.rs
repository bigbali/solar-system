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
        flex::{self, Flex, FlexAxisAlign, FlexCrossAxisAlign, FlexDirection},
        root::RootNode,
        window::{
            UiWindow, WindowDimension, WindowPlacement, WindowPlacementAlignTo, WindowPosition,
        },
        Border, Size,
    },
    UiColor,
};

pub fn spawn_window_system(mut context: NonSendMut<ImguiContext>, windows: Query<&Window>) {
    let bevy_window = windows.single();

    let ui = context.ui();

    let mut window = UiWindow::new(
        bevy_window,
        WindowDimension::Percentage(50.0),
        WindowDimension::Pixels(160.0),
    );

    window
        .title("Spawn Window".to_string())
        .title_bar(false)
        .displayed(true)
        // .movable(true)
        // .resizable(false)
        .background(UiColor::from(LinearRgba::new(0.7, 0.5, 0.4, 1.0)))
        .position(WindowPosition {
            x: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
            y: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
        })
        .padding(0.0);

    // let mut window = UiWindow {
    //     title: "Spawn Window".to_string(),
    //     title_bar: false,
    //     displayed: true,
    //     fixed: true,
    //     resizable: false,
    //     background: UiColor::from(LinearRgba::new(0.7, 0.5, 0.4, 1.0)),
    //     width: WindowDimension::Percentage(50.0),
    //     // width: WindowDimension::Fixed(400.0),
    //     height: WindowDimension::Pixels(160.0),
    //     position: WindowPosition {
    //         x: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
    //         y: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
    //     },
    //     padding: 0.0,
    //     ..UiWindow::new()
    // };

    window
        .children(|w| {
            w.flex()
                .flex_direction(flex::FlexDirection::Column)
                .align_axis(FlexAxisAlign::Between)
                .width(Size::Auto)
                .height(Size::Auto)
                .fill(UiColor::from(LinearRgba::new(1.0, 0.5, 0.5, 1.0)))
                .children(|f| {
                    f.flex()
                        .flex_direction(flex::FlexDirection::Row)
                        .width(Size::Auto)
                        .height(Size::Pixels(40.0))
                        .fill(UiColor::from(LinearRgba::new(0.5, 1.0, 0.5, 1.0)));
                    f.flex()
                        .flex_direction(flex::FlexDirection::Row)
                        .width(Size::Auto)
                        .height(Size::Percentage(50.0))
                        // .height(Size::Pixels(80.0))
                        .fill(UiColor::from(LinearRgba::new(0.5, 0.5, 1.0, 1.0)));

                    // let ff = f
                    //     .flex(Flex {
                    //         // fill_parent: true,
                    //         direction: FlexDirection::Column,
                    //         gap: 8.0,
                    //         fill: Some(UiColor::from(LinearRgba::new(
                    //             160.0 / 255.0,
                    //             160.0 / 255.0,
                    //             255.0 / 255.0,
                    //             1.0,
                    //         ))),
                    //         ..default()
                    //     })
                    //     .children(|a| {
                    //         a.flex_default().fill(UiColor::new(Color::WHITE));
                    //         a.flex_default().fill(UiColor::new(Color::BLACK));
                    //     });
                });
        })
        .build(ui, bevy_window);
}

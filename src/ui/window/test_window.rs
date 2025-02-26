use std::{
    cell::Cell,
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
            Window as UiWindow, WindowDimension, WindowPlacement, WindowPlacementAlignTo,
            WindowPosition,
        },
        Border,
    },
    UiColor,
};

static UI_TEST_MODE: atomic::AtomicBool = atomic::AtomicBool::new(false);

pub static mut PLACEMENT_X: WindowPlacementAlignTo = WindowPlacementAlignTo::Start;
pub static mut PLACEMENT_Y: WindowPlacementAlignTo = WindowPlacementAlignTo::Start;

pub fn test_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyT) {
        let _ = // Toggle test mode
            UI_TEST_MODE.fetch_update(atomic::Ordering::SeqCst, atomic::Ordering::SeqCst, |x| {
                Some(!x)
            });
    }

    if !UI_TEST_MODE.load(atomic::Ordering::SeqCst) {
        return;
    }

    let bevy_window = windows.single();

    let ui = context.ui();
    // let window = ui.window("##Test Window");

    let mut cw = UiWindow {
        title: "Test Window".to_string(),
        title_bar: true,
        displayed: true,
        fixed: true,
        resizable: false,
        width: WindowDimension::Stretch,
        height: WindowDimension::Stretch,
        position: WindowPosition {
            x: WindowPlacement::Manual(0.0),
            y: WindowPlacement::Manual(0.0),
        },
        padding: 8.0,
        ..UiWindow::new()
    };

    let mut windowplacement_hor = WindowPlacement::Manual(0.0);
    let mut windowplacement_vert = WindowPlacement::Manual(0.0);

    cw.children(|w| {
        w.flex_default()
            .flex_direction(flex::FlexDirection::Column)
            .fill(UiColor::from(LinearRgba::new(
                100.0 / 255.0,
                80.0 / 255.0,
                90.0 / 255.0,
                1.0,
            )))
            .children(|f| {
                let ff = f
                    .flex(Flex {
                        fill_parent: true,
                        direction: FlexDirection::Row,
                        gap: 8.0,
                        axis_align_items: FlexAxisAlign::Start,
                        cross_axis_align_items: FlexCrossAxisAlign::Start,
                        fill: Some(UiColor::from(LinearRgba::new(
                            90.0 / 255.0,
                            80.0 / 255.0,
                            50.0 / 255.0,
                            1.0,
                        ))),
                        ..default()
                    })
                    .children(|a| {
                        windowplacement_hor = a
                            .dropdown(Dropdown::<WindowPlacement> {
                                id: Dropdown::manual_id(0),
                                width: 200.0,
                                height: 48.0,
                                border: Border {
                                    size: 1.0,
                                    color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                                },
                                background: UiColor::from(LinearRgba::new(
                                    80.0 / 255.0,
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                    1.0,
                                )),
                                label: "Horizontal placement".to_string(),
                                on_select: None,
                                values: vec![
                                    DropdownValue {
                                        value: WindowPlacement::Manual(0.0),
                                        label: "Manual".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::Start,
                                        ),
                                        label: "Align to start".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::End,
                                        ),
                                        label: "Align to end".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::Center,
                                        ),
                                        label: "Align to center".to_string(),
                                    },
                                ],
                            })
                            .get_value_copy();
                        windowplacement_vert = a
                            .dropdown(Dropdown::<WindowPlacement> {
                                id: Dropdown::manual_id(1),
                                width: 200.0,
                                height: 48.0,
                                border: Border {
                                    size: 1.0,
                                    color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                                },
                                background: UiColor::from(LinearRgba::new(
                                    80.0 / 255.0,
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                    1.0,
                                )),
                                label: "Vertical placement".to_string(),
                                on_select: None,
                                values: vec![
                                    DropdownValue {
                                        value: WindowPlacement::Manual(0.0),
                                        label: "Manual".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::Start,
                                        ),
                                        label: "Align to start".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::End,
                                        ),
                                        label: "Align to end".to_string(),
                                    },
                                    DropdownValue {
                                        value: WindowPlacement::AlignTo(
                                            WindowPlacementAlignTo::Center,
                                        ),
                                        label: "Align to center".to_string(),
                                    },
                                ],
                            })
                            .get_value_copy();
                    });
            });
    })
    .position(WindowPosition {
        x: windowplacement_hor,
        y: windowplacement_vert,
    })
    .build(ui, bevy_window);

    // let size = [
    //     bevy_window.resolution.width() as f32,
    //     bevy_window.resolution.height() as f32,
    // ];

    // window
    //     .size(size, imgui::Condition::Always)
    //     .position([0.0, 0.0], imgui::Condition::Always)
    //     .title_bar(false)
    //     .build(|| {
    //         let mut root = RootNode::new();

    //         root.children(|r| {
    //             r.flex(Flex {
    //                 fill_parent: true,
    //                 gap: 8.0,
    //                 axis_align_items: FlexAxisAlign::Stretch,
    //                 cross_axis_align_items: FlexCrossAxisAlign::Stretch,
    //                 fill: Some(UiColor::from(LinearRgba::new(
    //                     30.0 / 255.0,
    //                     30.0 / 255.0,
    //                     30.0 / 255.0,
    //                     1.0,
    //                 ))),
    //                 ..default()
    //             })
    //             .children(|f| {
    //                 f.flex(Flex {
    //                     axis_align_items: FlexAxisAlign::Stretch,
    //                     cross_axis_align_items: FlexCrossAxisAlign::Stretch,
    //                     // direction: flex::FlexDirection::Column,
    //                     gap: 8.0,
    //                     fill: Some(UiColor::from(LinearRgba::new(
    //                         50.0 / 255.0,
    //                         50.0 / 255.0,
    //                         50.0 / 255.0,
    //                         1.0,
    //                     ))),
    //                     border: Border {
    //                         size: 1.0,
    //                         color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                     },
    //                     ..default()
    //                 })
    //                 .children(|_f| {
    //                     _f.flex(Flex {
    //                         axis_align_items: FlexAxisAlign::Start,
    //                         cross_axis_align_items: FlexCrossAxisAlign::Start,
    //                         width: _f.parent_width() / 2.0 - 4.0,
    //                         fill: Some(UiColor::from(LinearRgba::new(
    //                             50.0 / 255.0,
    //                             70.0 / 255.0,
    //                             80.0 / 255.0,
    //                             1.0,
    //                         ))),
    //                         border: Border {
    //                             size: 1.0,
    //                             color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                         },
    //                         ..default()
    //                     })
    //                     .children(|__f| {
    //                         __f.button(Button {
    //                             width: 135.0,
    //                             height: 48.0,
    //                             border: Border {
    //                                 size: 1.0,
    //                                 color: UiColor::from(LinearRgba::new(0.8, 0.4, 0.3, 1.0)),
    //                             },
    //                             background: UiColor::from(LinearRgba::new(
    //                                 80.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 1.0,
    //                             )),
    //                             label: "Button 1".to_string(),
    //                             on_click: Some(Box::new(|| {
    //                                 println!("Button 1 clicked");
    //                             })),
    //                         });
    //                         __f.button(Button {
    //                             width: 135.0,
    //                             height: 48.0,
    //                             border: Border {
    //                                 size: 1.0,
    //                                 color: UiColor::from(LinearRgba::new(0.8, 0.4, 0.3, 1.0)),
    //                             },
    //                             background: UiColor::from(LinearRgba::new(
    //                                 80.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 1.0,
    //                             )),
    //                             label: "Button 2".to_string(),
    //                             on_click: Some(Box::new(|| {
    //                                 println!("Button 2 clicked");
    //                             })),
    //                         });
    //                     });
    //                     _f.flex(Flex {
    //                         axis_align_items: FlexAxisAlign::Start,
    //                         cross_axis_align_items: FlexCrossAxisAlign::Start,
    //                         width: _f.parent_width() / 2.0 - 4.0,

    //                         fill: Some(UiColor::from(LinearRgba::new(
    //                             50.0 / 255.0,
    //                             70.0 / 255.0,
    //                             80.0 / 255.0,
    //                             1.0,
    //                         ))),
    //                         border: Border {
    //                             size: 1.0,
    //                             color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                         },
    //                         ..default()
    //                     });
    //                 });
    //                 f.flex(Flex {
    //                     axis_align_items: FlexAxisAlign::Stretch,
    //                     cross_axis_align_items: FlexCrossAxisAlign::Stretch,
    //                     direction: flex::FlexDirection::Column,

    //                     gap: 8.0,
    //                     fill: Some(UiColor::from(LinearRgba::new(
    //                         50.0 / 255.0,
    //                         50.0 / 255.0,
    //                         50.0 / 255.0,
    //                         1.0,
    //                     ))),
    //                     border: Border {
    //                         size: 1.0,
    //                         color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                     },
    //                     ..default()
    //                 })
    //                 .children(|_f| {
    //                     _f.flex(Flex {
    //                         axis_align_items: FlexAxisAlign::Stretch,
    //                         cross_axis_align_items: FlexCrossAxisAlign::Stretch,
    //                         fill: Some(UiColor::from(LinearRgba::new(
    //                             90.0 / 255.0,
    //                             60.0 / 255.0,
    //                             50.0 / 255.0,
    //                             1.0,
    //                         ))),
    //                         border: Border {
    //                             size: 1.0,
    //                             color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                         },
    //                         ..default()
    //                     });
    //                     _f.flex(Flex {
    //                         axis_align_items: FlexAxisAlign::Start,
    //                         cross_axis_align_items: FlexCrossAxisAlign::Start,
    //                         fill: Some(UiColor::from(LinearRgba::new(
    //                             90.0 / 255.0,
    //                             60.0 / 255.0,
    //                             50.0 / 255.0,
    //                             1.0,
    //                         ))),
    //                         border: Border {
    //                             size: 1.0,
    //                             color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                         },
    //                         ..default()
    //                     });
    //                 });
    //             });
    //         })
    //         .build(ui);
    //     });
}

use std::sync::{atomic, Arc, Mutex};

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_mod_imgui::ImguiContext;

use crate::ui::{
    element::{
        button::{Button, ButtonChild},
        flex::{self, Flex, FlexAxisAlign, FlexCrossAxisAlign},
        root::RootNode,
        Border,
    },
    UiColor,
};

static UI_TEST_MODE: atomic::AtomicBool = atomic::AtomicBool::new(false);

pub fn test_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyT) {
        let _ =
            UI_TEST_MODE.fetch_update(atomic::Ordering::SeqCst, atomic::Ordering::SeqCst, |x| {
                Some(!x)
            });
    }

    if !UI_TEST_MODE.load(atomic::Ordering::SeqCst) {
        return;
    }

    let bevy_window = windows.single();

    let ui = context.ui();
    let window = ui.window("##Test Window");

    let size = [
        bevy_window.resolution.width() as f32,
        bevy_window.resolution.height() as f32,
    ];

    window
        .size(size, imgui::Condition::Always)
        .position([0.0, 0.0], imgui::Condition::Always)
        .title_bar(false)
        .build(|| {
            let mut root = RootNode::new();

            root.children(|r| {
                r.flex(Flex {
                    fill_parent: true,
                    gap: 8.0,
                    axis_align_items: FlexAxisAlign::Stretch,
                    cross_axis_align_items: FlexCrossAxisAlign::Stretch,
                    fill: Some(UiColor::from(LinearRgba::new(
                        30.0 / 255.0,
                        30.0 / 255.0,
                        30.0 / 255.0,
                        1.0,
                    ))),
                    ..default()
                })
                .children(|f| {
                    f.flex(Flex {
                        axis_align_items: FlexAxisAlign::Stretch,
                        cross_axis_align_items: FlexCrossAxisAlign::Stretch,
                        // direction: flex::FlexDirection::Column,
                        gap: 8.0,
                        fill: Some(UiColor::from(LinearRgba::new(
                            50.0 / 255.0,
                            50.0 / 255.0,
                            50.0 / 255.0,
                            1.0,
                        ))),
                        border: Border {
                            size: 1.0,
                            color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                        },
                        ..default()
                    })
                    .children(|_f| {
                        _f.flex(Flex {
                            axis_align_items: FlexAxisAlign::Start,
                            cross_axis_align_items: FlexCrossAxisAlign::Start,
                            width: _f.parent_width() / 2.0 - 4.0,
                            fill: Some(UiColor::from(LinearRgba::new(
                                50.0 / 255.0,
                                70.0 / 255.0,
                                80.0 / 255.0,
                                1.0,
                            ))),
                            border: Border {
                                size: 1.0,
                                color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                            },
                            ..default()
                        })
                        .children(|__f| {
                            __f.button(Button {
                                width: 135.0,
                                height: 48.0,
                                border: Border {
                                    size: 1.0,
                                    color: UiColor::from(LinearRgba::new(0.8, 0.4, 0.3, 1.0)),
                                },
                                background: UiColor::from(LinearRgba::new(
                                    80.0 / 255.0,
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                    1.0,
                                )),
                                label: "Button 1".to_string(),
                                on_click: Some(Box::new(|| {
                                    println!("Button 1 clicked");
                                })),
                            });
                            __f.button(Button {
                                width: 135.0,
                                height: 48.0,
                                border: Border {
                                    size: 1.0,
                                    color: UiColor::from(LinearRgba::new(0.8, 0.4, 0.3, 1.0)),
                                },
                                background: UiColor::from(LinearRgba::new(
                                    80.0 / 255.0,
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                    1.0,
                                )),
                                label: "Button 2".to_string(),
                                on_click: Some(Box::new(|| {
                                    println!("Button 2 clicked");
                                })),
                            });
                        });
                        _f.flex(Flex {
                            axis_align_items: FlexAxisAlign::Start,
                            cross_axis_align_items: FlexCrossAxisAlign::Start,
                            width: _f.parent_width() / 2.0 - 4.0,

                            fill: Some(UiColor::from(LinearRgba::new(
                                50.0 / 255.0,
                                70.0 / 255.0,
                                80.0 / 255.0,
                                1.0,
                            ))),
                            border: Border {
                                size: 1.0,
                                color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                            },
                            ..default()
                        });
                    });
                    f.flex(Flex {
                        axis_align_items: FlexAxisAlign::Stretch,
                        cross_axis_align_items: FlexCrossAxisAlign::Stretch,
                        direction: flex::FlexDirection::Column,

                        gap: 8.0,
                        fill: Some(UiColor::from(LinearRgba::new(
                            50.0 / 255.0,
                            50.0 / 255.0,
                            50.0 / 255.0,
                            1.0,
                        ))),
                        border: Border {
                            size: 1.0,
                            color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                        },
                        ..default()
                    })
                    .children(|_f| {
                        _f.flex(Flex {
                            axis_align_items: FlexAxisAlign::Stretch,
                            cross_axis_align_items: FlexCrossAxisAlign::Stretch,
                            fill: Some(UiColor::from(LinearRgba::new(
                                90.0 / 255.0,
                                60.0 / 255.0,
                                50.0 / 255.0,
                                1.0,
                            ))),
                            border: Border {
                                size: 1.0,
                                color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                            },
                            ..default()
                        });
                        _f.flex(Flex {
                            axis_align_items: FlexAxisAlign::Start,
                            cross_axis_align_items: FlexCrossAxisAlign::Start,
                            fill: Some(UiColor::from(LinearRgba::new(
                                90.0 / 255.0,
                                60.0 / 255.0,
                                50.0 / 255.0,
                                1.0,
                            ))),
                            border: Border {
                                size: 1.0,
                                color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
                            },
                            ..default()
                        });
                    });
                });
            })
            .build(ui);
        });
}

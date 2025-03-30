use std::{
    cell::RefCell,
    sync::atomic::{self, AtomicBool},
};

use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::ui::{
    element::{
        dropdown::{DropdownChild, DropdownValue},
        flex::{FlexAxisAlign, FlexChild, FlexDirection},
        window::{UiWindow, WindowPlacement, WindowPlacementAlignTo, WindowPosition, WindowSize},
        Size,
    },
    UiColor,
};

/// Whether or not we should display the debug UI overlay.
static IS_UI_TEST_MODE: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Whether or not we should display the cursor move calls we have made from our Flex elements.
pub static IS_DEBUG_FLEX_CURSOR_CALLS: AtomicBool = AtomicBool::new(false);

/// Whether or not we should display Flex elements that have no debug id.
pub static IS_SHOW_UNNAMED_ELEMENTS: AtomicBool = AtomicBool::new(false);

thread_local! {
    pub static MOVE_CURSOR_CALLS: RefCell<Vec<([f32; 2], Option<&'static str>, &'static str, u32)>> = RefCell::new(Vec::new());
}

pub fn test_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyT) {
        let _ = // Toggle test mode
            IS_UI_TEST_MODE.fetch_update(atomic::Ordering::SeqCst, atomic::Ordering::SeqCst, |x| {
                Some(!x)
            });
    }

    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyF) {
        let _ = IS_DEBUG_FLEX_CURSOR_CALLS.fetch_update(
            atomic::Ordering::SeqCst,
            atomic::Ordering::SeqCst,
            |x| Some(!x),
        );
    }

    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyG) {
        let _ = IS_SHOW_UNNAMED_ELEMENTS.fetch_update(
            atomic::Ordering::SeqCst,
            atomic::Ordering::SeqCst,
            |x| Some(!x),
        );
    }

    if !IS_UI_TEST_MODE.load(atomic::Ordering::SeqCst) {
        return;
    }

    let bevy_window = windows.single();

    let ui = context.ui();

    let mut cw = UiWindow::new(
        bevy_window,
        WindowSize::Percentage(100.0),
        WindowSize::Percentage(100.0),
    );

    cw.title("Test Window".to_string())
        .title_bar(true)
        .movable(false)
        .resizable(false)
        .position(WindowPosition {
            x: WindowPlacement::AlignTo(WindowPlacementAlignTo::Start),
            y: WindowPlacement::AlignTo(WindowPlacementAlignTo::Start),
        })
        .padding(8.0);

    let mut windowplacement_hor = WindowPlacement::Manual(0.0);
    let mut windowplacement_vert = WindowPlacement::Manual(0.0);

    cw.children(|w| {
        w.flex()
            .debug_id("TOP LEVEL CONTAINER FLEX")
            .width(Size::Percentage(100.0))
            .height(Size::Percentage(100.0))
            .align_axis(FlexAxisAlign::Between)
            .gap(8.0)
            .fill(UiColor::from(LinearRgba::RED))
            .children(|f| {
                f.flex()
                    .debug_id("CHILD 1")
                    .gap(8.0)
                    .width(Size::Auto)
                    .height(Size::Auto)
                    .flex_direction(FlexDirection::Column)
                    .fill(UiColor::from(LinearRgba::GREEN))
                    .children(|a| {
                        windowplacement_hor = a
                            .dropdown::<WindowPlacement>()
                            .width(Size::Pixels(200.0))
                            .height(Size::Pixels(48.0))
                            .label("Horizontal placement".to_string())
                            .from_values(vec![
                                DropdownValue {
                                    value: WindowPlacement::Manual(0.0),
                                    label: "Manual".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::Start),
                                    label: "Align to start".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
                                    label: "Align to end".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
                                    label: "Align to center".to_string(),
                                },
                            ])
                            .get_value_copy();
                        windowplacement_vert = a
                            .dropdown::<WindowPlacement>()
                            .width(Size::Pixels(200.0))
                            .height(Size::Pixels(48.0))
                            .label("Vertical placement".to_string())
                            .from_values(vec![
                                DropdownValue {
                                    value: WindowPlacement::Manual(0.0),
                                    label: "Manual".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::Start),
                                    label: "Align to start".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
                                    label: "Align to end".to_string(),
                                },
                                DropdownValue {
                                    value: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
                                    label: "Align to center".to_string(),
                                },
                            ])
                            .get_value_copy();
                    });

                f.flex()
                    .debug_id("CHILD 2")
                    .width(Size::Auto)
                    .height(Size::Auto)
                    .fill(LinearRgba::rgb(0.8, 0.8, 0.3).into());
            });
    })
    .position(WindowPosition {
        x: windowplacement_hor,
        y: windowplacement_vert,
    })
    .build(ui, bevy_window);

    if IS_DEBUG_FLEX_CURSOR_CALLS.load(atomic::Ordering::SeqCst) {
        ui.window("FlexCursorCallsDebug")
            .size([300.0, 400.0], imgui::Condition::FirstUseEver)
            .build(|| {
                MOVE_CURSOR_CALLS.with_borrow(|v| {
                    for (pos, label, info, line) in v.iter() {
                        ui.text(format!("{:?}, {:?}", label, pos));
                        ui.new_line();
                        ui.text(format!("    {:?}: {}", info, line));
                        ui.new_line();
                    }
                });
            });
    }

    MOVE_CURSOR_CALLS.take();
}

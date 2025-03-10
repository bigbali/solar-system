use std::sync::atomic;

use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::ui::{
    element::{
        dropdown::{Dropdown, DropdownChild, DropdownValue, OnSelectCallback},
        flex::{self, Flex, FlexAxisAlign, FlexCrossAxisAlign, FlexDirection},
        window::{
            UiWindow, WindowDimension, WindowPlacement, WindowPlacementAlignTo, WindowPosition,
        },
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

    // let mut cw = UiWindow {
    //     title: "Test Window".to_string(),
    //     title_bar: true,
    //     displayed: true,
    //     fixed: true,
    //     resizable: false,
    //     width: WindowDimension::Stretch,
    //     height: WindowDimension::Stretch,
    //     position: WindowPosition {
    //         x: WindowPlacement::Manual(0.0),
    //         y: WindowPlacement::Manual(0.0),
    //     },
    //     padding: 8.0,
    //     ..UiWindow::new()
    // };

    // let mut windowplacement_hor = WindowPlacement::Manual(0.0);
    // let mut windowplacement_vert = WindowPlacement::Manual(0.0);

    // cw.children(|w| {
    //     w.flex_default()
    //         .flex_direction(flex::FlexDirection::Column)
    //         .fill(UiColor::from(LinearRgba::new(
    //             100.0 / 255.0,
    //             80.0 / 255.0,
    //             90.0 / 255.0,
    //             1.0,
    //         )))
    //         .children(|f| {
    //             let ff = f
    //                 .flex(Flex {
    //                     fill_parent: true,
    //                     direction: FlexDirection::Row,
    //                     gap: 8.0,
    //                     axis_align_items: FlexAxisAlign::Start,
    //                     cross_axis_align_items: FlexCrossAxisAlign::Start,
    //                     fill: Some(UiColor::from(LinearRgba::new(
    //                         90.0 / 255.0,
    //                         80.0 / 255.0,
    //                         50.0 / 255.0,
    //                         1.0,
    //                     ))),
    //                     ..default()
    //                 })
    //                 .children(|a| {
    //                     windowplacement_hor = a
    //                         .dropdown(Dropdown::<WindowPlacement> {
    //                             id: Dropdown::manual_id(0),
    //                             width: 200.0,
    //                             height: 48.0,
    //                             border: Border {
    //                                 size: 1.0,
    //                                 color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                             },
    //                             background: UiColor::from(LinearRgba::new(
    //                                 80.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 1.0,
    //                             )),
    //                             label: "Horizontal placement".to_string(),
    //                             on_select: OnSelectCallback(None),
    //                             values: vec![
    //                                 DropdownValue {
    //                                     value: WindowPlacement::Manual(0.0),
    //                                     label: "Manual".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::Start,
    //                                     ),
    //                                     label: "Align to start".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::End,
    //                                     ),
    //                                     label: "Align to end".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::Center,
    //                                     ),
    //                                     label: "Align to center".to_string(),
    //                                 },
    //                             ],
    //                         })
    //                         .get_value_copy();
    //                     windowplacement_vert = a
    //                         .dropdown(Dropdown::<WindowPlacement> {
    //                             id: Dropdown::manual_id(1),
    //                             width: 200.0,
    //                             height: 48.0,
    //                             border: Border {
    //                                 size: 1.0,
    //                                 color: UiColor::from(LinearRgba::new(0.8, 0.8, 0.8, 1.0)),
    //                             },
    //                             background: UiColor::from(LinearRgba::new(
    //                                 80.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 30.0 / 255.0,
    //                                 1.0,
    //                             )),
    //                             label: "Vertical placement".to_string(),
    //                             on_select: OnSelectCallback(None),
    //                             values: vec![
    //                                 DropdownValue {
    //                                     value: WindowPlacement::Manual(0.0),
    //                                     label: "Manual".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::Start,
    //                                     ),
    //                                     label: "Align to start".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::End,
    //                                     ),
    //                                     label: "Align to end".to_string(),
    //                                 },
    //                                 DropdownValue {
    //                                     value: WindowPlacement::AlignTo(
    //                                         WindowPlacementAlignTo::Center,
    //                                     ),
    //                                     label: "Align to center".to_string(),
    //                                 },
    //                             ],
    //                         })
    //                         .get_value_copy();
    //                 });
    //         });
    // })
    // .position(WindowPosition {
    //     x: windowplacement_hor,
    //     y: windowplacement_vert,
    // })
    // .build(ui, bevy_window);
}

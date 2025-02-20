use std::{cell::RefCell, rc::Rc};

use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::simulation::body::*;

use super::{
    apply_button_color,
    clear_button_color,
    element::{
        self,
        button::ButtonChild,
        flex::{self, FlexAxisAlign, FlexCrossAxisAlign},
        root::RootNode,
    },
    // flex::{
    //     // flex1::{self, Children},
    //     flex2::{self, FlexCrossAxisAlign, FlexRow},
    //     FlexAxisAlign,
    // },
    util::with_color_scheme,
};

static mut ALIGN_AXIS_SELECT: usize = 0;
static mut ALIGN_CROSS_AXIS_SELECT: usize = 0;
static mut ALIGN_AXIS: FlexAxisAlign = FlexAxisAlign::Start;
static mut ALIGN_CROSS_AXIS: FlexCrossAxisAlign = FlexCrossAxisAlign::Start;
static mut DEBUG: bool = false;
static mut FILL: bool = false;
static mut BORDER: bool = true;
static mut HAUTO: bool = true;

pub fn left_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    query: Query<(&Transform, &Body, Entity), (Without<Camera>, With<Planet>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut camera_speed: ResMut<bevy_flycam::MovementSettings>,
    mut follow: ResMut<crate::simulation::settings::FollowBody>,
    mut trajectory: ResMut<crate::simulation::trajectory::CalculateTrajectory>,
    mut trajectories: ResMut<crate::simulation::trajectory::Trajectories>,
    mut selected_body: ResMut<crate::simulation::settings::SelectedBody>,
    mut parameters: ResMut<crate::simulation::settings::SimulationParameters>,
    mut elapsed_time: ResMut<crate::simulation::settings::ElapsedTime>,
    sun: Res<Sun>,
) {
    let bevy_window = windows.single();
    let sun = query.get(sun.0);

    let ui = context.ui();
    let action_window = ui.window("Actions");

    let size = [
        (bevy_window.resolution.physical_width() / 4) as f32,
        bevy_window.resolution.physical_height() as f32,
    ];

    let x: Vec<imgui::StyleStackToken> =
        vec![ui.push_style_var(imgui::StyleVar::WindowPadding([8.0, 30.0]))];

    with_color_scheme(ui, || {
        action_window
            .size(size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.separator();
                ui.text("Bodies");
                ui.separator();
                // ui.dummy([0.0, 4.0]);

                ui.checkbox("Fill", unsafe { &mut FILL });
                ui.checkbox("Debug", unsafe { &mut DEBUG });
                ui.checkbox("Border", unsafe { &mut BORDER });
                ui.checkbox("H auto", unsafe { &mut HAUTO });

                let items = vec!["Left", "Right", "Between", "Stretch"];
                if let Some(cb) = ui.begin_combo(
                    "##Combo",
                    format!("Axis: {}", items[unsafe { ALIGN_AXIS_SELECT }]),
                ) {
                    for (i, cur) in items.iter().enumerate() {
                        unsafe {
                            if items[ALIGN_AXIS_SELECT] == *cur {
                                // Auto-scroll to selected item
                                ui.set_item_default_focus();
                            }
                            // Create a "selectable"
                            let clicked = ui
                                .selectable_config(cur)
                                .selected(items[ALIGN_AXIS_SELECT] == *cur)
                                .build();
                            // When item is clicked, store it
                            if clicked {
                                ALIGN_AXIS_SELECT = i;
                            }

                            ALIGN_AXIS = match items[ALIGN_AXIS_SELECT] {
                                "Right" => FlexAxisAlign::End,
                                "Between" => FlexAxisAlign::Between,
                                "Stretch" => FlexAxisAlign::Stretch,
                                _ => FlexAxisAlign::Start,
                            }
                        }
                    }
                }
                let items2 = vec!["Start", "End", "Center", "Stretch"];
                if let Some(cb) = ui.begin_combo(
                    "##Combo2",
                    format!("Cross axis: {}", items2[unsafe { ALIGN_CROSS_AXIS_SELECT }]),
                ) {
                    for (i, cur) in items2.iter().enumerate() {
                        unsafe {
                            if items[ALIGN_CROSS_AXIS_SELECT] == *cur {
                                // Auto-scroll to selected item
                                ui.set_item_default_focus();
                            }
                            // Create a "selectable"
                            let clicked = ui
                                .selectable_config(cur)
                                .selected(items2[ALIGN_CROSS_AXIS_SELECT] == *cur)
                                .build();
                            // When item is clicked, store it
                            if clicked {
                                ALIGN_CROSS_AXIS_SELECT = i;
                            }

                            ALIGN_CROSS_AXIS = match items2[ALIGN_CROSS_AXIS_SELECT] {
                                "Start" => FlexCrossAxisAlign::Start,
                                "End" => FlexCrossAxisAlign::End,
                                "Center" => FlexCrossAxisAlign::Center,
                                "Stretch" => FlexCrossAxisAlign::Stretch,
                                _ => FlexCrossAxisAlign::Start,
                            }
                        }
                    }
                }

                // let mut root = RootNode::new(ui);

                // root.add_child::<FlexRow>();

                // let mut root = flex1::RootNode::new(ui);

                // let mut binding = root.add_child::<FlexRow>();
                // let mut x = binding.get_mut().;

                // let root = flex2::RootNode::new(Rc::new(RefCell::new(ui)));

                // let rc_ui = Rc::new(RefCell::new(&mut *ui));

                // Create RootNode
                // let mut root = RootNode::new();

                // root.children(|r| {
                //     r.flex_default()
                //         .align_axis(unsafe { ALIGN_AXIS })
                //         .align_cross_axis(unsafe { ALIGN_CROSS_AXIS })
                //         .flex_direction(flex::FlexDirection::Column)
                //         .width(600.0)
                //         .height(800.0)
                //         .border(1.0)
                //         .gap(8.0)
                //         .fill([1.0, 0.0, 0.0, 1.0])
                //         .children(|f| {
                //             f.flex_default()
                //                 .width(300.0)
                //                 .height(200.0)
                //                 .border(3.0)
                //                 .fill([0.0, 0.0, 1.0, 1.0])
                //                 .align_axis(unsafe { ALIGN_AXIS })
                //                 .align_cross_axis(unsafe { ALIGN_CROSS_AXIS })
                //                 .children(|a| {
                //                     a.button(element::button::Button::default());
                //                     a.button(element::button::Button {
                //                         width: 100.0,
                //                         height: 28.0,
                //                         border: 0.0,
                //                         background: LinearRgba::RED,
                //                         label: "Button".to_string(),
                //                     });
                //                 });

                //             f.flex_default()
                //                 .width(100.0)
                //                 .height(100.0)
                //                 .border(5.0)
                //                 .fill([0.0, 1.0, 0.0, 1.0]);

                //             f.flex_default()
                //                 .width(200.0)
                //                 .height(150.0)
                //                 .border(1.0)
                //                 .fill([0.0, 1.0, 1.0, 1.0])
                //                 .align_axis(unsafe { ALIGN_AXIS })
                //                 .children(|a| {
                //                     a.button(element::button::Button {
                //                         width: 100.0,
                //                         height: 60.0,
                //                         border: 0.0,
                //                         background: LinearRgba::RED,
                //                         label: "Button".to_string(),
                //                     });
                //                 });
                //         });
                // });

                // root.build(ui);

                // ui.dummy([0.0, 16.0]);

                // let mut root3 = flex2::RootNode::new();

                // let z = root3
                //     .flex_row()
                //     .align_axis(unsafe { ALIGN_AXIS })
                //     .align_cross_axis(unsafe { ALIGN_CROSS_AXIS })
                //     .width(600.0)
                //     .height(400.0)
                //     .border(1.0)
                //     .gap(8.0)
                //     .fill([1.0, 0.0, 0.0, 1.0]);

                // z.flex_row()
                //     .width(300.0)
                //     .height(200.0)
                //     .border(1.0)
                //     .fill([0.0, 0.0, 1.0, 1.0]);

                // z.flex_row()
                //     .width(100.0)
                //     .height(100.0)
                //     .border(1.0)
                //     .fill([0.0, 1.0, 0.0, 1.0]);

                // z.flex_row()
                //     .width(125.0)
                //     .height(150.0)
                //     .border(1.0)
                //     .fill([1.0, 1.0, 0.0, 1.0]);

                // root3.build(ui);

                // ui.dummy([0.0, 16.0]);

                // let mut root2 = flex2::RootNode::new();

                // let y = root2
                //     .flex_row()
                //     .align_axis(unsafe { ALIGN_AXIS })
                //     .align_cross_axis(unsafe { ALIGN_CROSS_AXIS })
                //     .width(600.0)
                //     .height(300.0)
                //     .border(0.0)
                //     .gap(8.0)
                //     // .fill_parent(true)
                //     .fill([1.0, 0.0, 0.0, 1.0]);

                // y.flex_row()
                //     .width(300.0)
                //     .height(200.0)
                //     .border(2.0)
                //     .fill([0.0, 0.0, 1.0, 1.0]);

                // y.flex_row()
                //     .width(100.0)
                //     .height(100.0)
                //     .border(2.0)
                //     .fill([0.0, 1.0, 0.0, 1.0]);

                // root2.build(ui);

                // let mut rootx = flex2::RootNode::new();
                // rootx
                //     .children(|r| {
                //         r.flex_row().fill([0.5, 0.5, 0.5, 1.0]);
                //     })
                //     .build(ui);
                // println!("pos before update {:?}", ui.cursor_screen_pos());
                // ui.set_cursor_screen_pos([0.0, 0.0]);
                // println!("pos after update {:?}", ui.cursor_screen_pos());
                // ui.set_cursor_screen_pos([100.0, 100.0]);
                // println!("pos after second update {:?}", ui.cursor_screen_pos());

                // root.build();

                // let mut binding = FlexRow::new(ui);
                // let mut fr = binding
                //     .gap(8.0)
                //     .width(size[0])
                //     .height(600.0)
                //     .horizontal_spacing(unsafe { SPACING });

                // if { unsafe { FILL } } {
                //     fr = fr.fill([1.0, 0.0, 0.0, 1.0]);
                // }

                // if { unsafe { BORDER } } {
                //     fr = fr.border(1.0);
                // }

                // fr.button("hello", [120.0, 48.0], || println!("hello again xd"))
                //     .button("xd funny", [60.0, 48.0], || println!("xd funny"))
                //     .button("xd funny", [75.0, 48.0], || println!("xd funny"))
                //     .button("xd funny", [100.0, 48.0], || println!("xd funny"));

                // if { unsafe { HAUTO } } {
                //     fr = fr.height_auto();
                // }

                // fr.build_debug(unsafe { DEBUG });

                // FlexRow::new(ui)
                //     .gap(4.0)
                //     .horizontal_spacing(FlexSpacing::End)
                //     .button("hello", [120.0, 48.0], || println!("hello again xd"))
                //     .button("xd funny", [60.0, 48.0], || println!("xd funny"))
                //     .build();

                // FlexRow::new(ui)
                //     .gap(4.0)
                //     .horizontal_spacing(FlexSpacing::Between)
                //     .button("hello", [120.0, 48.0], || println!("hello again xd"))
                //     .button("xd funny", [60.0, 48.0], || println!("xd funny"))
                //     .build();

                // FlexRow::new(ui)
                //     .gap(4.0)
                //     .horizontal_spacing(FlexSpacing::Stretch)
                //     .button("hello", [120.0, 48.0], || println!("hello again xd"))
                //     .button("xd funny", [60.0, 48.0], || println!("xd funny"))
                //     .build();

                let mut camera_transform = camera.single_mut();

                for (_, body, entity) in query.iter() {
                    let color_stack = apply_button_color(ui, body.metadata.color.to_linear());

                    let button_text = match follow.entity {
                        Some(u_entity)
                            if entity == u_entity
                                && follow.is_active
                                && body.metadata.name.is_some() =>
                        {
                            format!("{} (following)", body.metadata.name.clone().unwrap())
                        }
                        _ => body
                            .metadata
                            .name
                            .clone()
                            .unwrap_or("<unknown body>".to_string())
                            .to_string(),
                    };

                    if ui.button_with_size(button_text, [164.0, 48.0]) {
                        match selected_body.entity {
                            Some(selected_entity) => {
                                if entity != selected_entity {
                                    selected_body.entity = Some(entity);
                                } else {
                                    selected_body.entity = None;
                                }
                            }
                            None => {
                                selected_body.entity = Some(entity);
                            }
                        };
                    }

                    clear_button_color(color_stack);

                    ui.dummy([0.0, 8.0]);
                }

                ui.group(|| {
                    ui.separator();
                    ui.text("Simulation Parameters");
                    ui.separator();
                    ui.dummy([0.0, 4.0]);

                    ui.input_float(
                        "Gravitational Constant",
                        &mut parameters.gravitational_constant,
                    )
                    .step(1e-4)
                    .build();
                    ui.input_float("Time Scaling (days/second)", &mut parameters.time_step)
                        .step(1.0 / 60.0) // step by 1 day/second
                        .build();
                    ui.input_float(
                        "Simulation Speed Multiplier",
                        &mut parameters.updates_per_step,
                    )
                    .step(1.0)
                    .build();

                    ui.dummy([0.0, 8.0]);
                });

                ui.group(|| {
                    ui.separator();
                    ui.text("Trajectory");
                    ui.separator();
                    ui.dummy([0.0, 4.0]);

                    ui.input_int("Steps", &mut trajectory.steps)
                        .step(100)
                        .build();

                    if ui.button_with_size("Calculate Trajectories", [200.0, 48.0]) {
                        trajectory.calculated = true;
                    } else {
                        trajectory.calculated = false;
                    }

                    ui.same_line_with_spacing(0.0, 4.0);

                    if ui.button_with_size("Clear Trajectories", [200.0, 48.0]) {
                        trajectories.0.clear();
                    }

                    ui.dummy([0.0, 8.0]);
                });

                ui.group(|| {
                    ui.separator();
                    ui.text("Selected Body");
                    ui.separator();
                    ui.dummy([0.0, 4.0]);

                    match selected_body.entity {
                        Some(entity) => {
                            let selected_body = query.get(entity);

                            if let Ok((_, body, _)) = selected_body {
                                ui.text(format!(
                                    "Selected: {:?}",
                                    body.metadata
                                        .name
                                        .clone()
                                        .unwrap_or("<unknown>".to_string())
                                ));
                            }

                            if ui.button_with_size("Visit", [200.0, 48.0]) {
                                if let Ok((transform, body, _)) = selected_body {
                                    camera_transform.translation = Vec3 {
                                        x: transform.translation.x,
                                        y: transform.translation.y,
                                        z: transform.translation.z
                                            + body.data.radius
                                            + match sun {
                                                Ok((_, b, _)) => b.data.radius,
                                                Err(_) => 0.2,
                                            },
                                    };
                                }
                            }

                            if ui.button_with_size("Follow", [200.0, 48.0]) {
                                if follow.entity != Some(entity) {
                                    follow.entity = Some(entity);
                                } else {
                                    follow.entity = None;
                                }
                            }
                        }
                        None => {
                            ui.text("No body selected");
                        }
                    }
                    ui.dummy([0.0, 8.0]);
                });

                ui.checkbox("Follow Planet", &mut follow.is_active);

                ui.input_float("Camera Speed", &mut camera_speed.speed)
                    .step(100.0)
                    .build();

                ui.input_float("Simulation Speed", &mut parameters.updates_per_step)
                    .step(0.1)
                    .build();

                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));

                ui.text(format!(
                    "Camera Position: (x: {:.1}, y: {:.1}, z: {:.1})",
                    camera_transform.translation.x,
                    camera_transform.translation.y,
                    camera_transform.translation.z
                ));

                ui.text(format!("Time Passed: {:.2} days", elapsed_time.0));
            });
    });

    for token in x {
        token.pop();
    }
}

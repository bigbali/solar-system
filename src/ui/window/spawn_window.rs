use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;

use crate::{
    simulation::settings::SimulationParameters,
    ui::{
        element::{
            flex::{self, FlexAxisAlign, FlexChild, FlexCrossAxisAlign},
            input::InputI32Child,
            text::TextChild,
            window::{
                UiWindow, WindowPlacement, WindowPlacementAlignTo, WindowPosition, WindowSize,
            },
            Size,
        },
        UiColor,
    },
};

pub fn spawn_window_system(
    mut context: NonSendMut<ImguiContext>,
    windows: Query<&Window>,
    mut params: ResMut<SimulationParameters>,
) {
    let bevy_window = windows.single();

    let ui = context.ui();

    let mut window = UiWindow::new(
        bevy_window,
        WindowSize::Percentage(50.0),
        WindowSize::Pixels(160.0),
    );

    window
        .title("Spawn Window".to_string())
        .title_bar(false)
        .displayed(true)
        .movable(false)
        // .resizable(false)
        .background(UiColor::from(LinearRgba::new(0.7, 0.5, 0.4, 1.0)))
        .position(WindowPosition {
            x: WindowPlacement::AlignTo(WindowPlacementAlignTo::Center),
            y: WindowPlacement::AlignTo(WindowPlacementAlignTo::End),
        })
        .padding(0.0);

    window
        .children(|w| {
            w.flex()
                .flex_direction(flex::FlexDirection::Column)
                // .align_axis(FlexAxisAlign::Between)
                .width(Size::Auto)
                .height(Size::Auto)
                .fill(UiColor::from(LinearRgba::new(1.0, 0.5, 0.5, 1.0)))
                .children(|f| {
                    f.flex()
                        .width(Size::Auto)
                        .height(Size::Pixels(40.0))
                        .fill(UiColor::from(LinearRgba::new(0.5, 1.0, 0.5, 1.0)))
                        .children(|ff| {
                            ff.text("Spawn Body")
                                .background(Color::BLACK.into())
                                .align_x(FlexCrossAxisAlign::Center)
                                .align_y(FlexCrossAxisAlign::Center);
                        });
                    f.flex()
                        .flex_direction(flex::FlexDirection::Column)
                        .width(Size::Auto)
                        .height(Size::Auto)
                        .fill(UiColor::from(LinearRgba::new(0.5, 0.5, 1.0, 1.0)))
                        .gap(8.0)
                        .children(|ff2| {
                            ff2.input_i32()
                                .label("Radius")
                                .step(1)
                                .width(Size::Percentage(33.3))
                                // .height(Size::Pixels(40.0))
                                .height(Size::Auto)
                                .background(LinearRgba::RED.into())
                                .default_value(0);
                            ff2.input_i32()
                                .label("Mass (Solar Mass)")
                                .step(1)
                                .width(Size::Percentage(33.3))
                                .height(Size::Auto)
                                .default_value(0);
                            let abc = ff2
                                .input_i32()
                                .label("TIME STEP (DELETE ME LATER)")
                                .step(1)
                                .width(Size::Percentage(33.3))
                                .height(Size::Auto)
                                .default_value((params.time_step * 100.0) as i32);

                            params.time_step = abc.get_value() as f32 / 100.0;
                        });
                });
        })
        .build(ui, bevy_window);
}

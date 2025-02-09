use crate::{
    impl_uicontainer,
    ui::{UiContainer, UiElement, UiElementType},
};

use super::FlexSpacing;

pub struct FlexRow<'a> {
    horizontal_spacing: FlexSpacing,
    vertical_spacing: FlexSpacing,
    gap: f32,
    width: f32,
    height: f32,
    border: f32,
    fill: Option<[f32; 4]>,
    imgui: &'a imgui::Ui,
    items: Vec<UiElement>,
}

impl_uicontainer!(FlexRow<'_>);

impl<'a> FlexRow<'a> {
    pub fn new(ui: &'a imgui::Ui) -> Self {
        Self {
            horizontal_spacing: FlexSpacing::Start,
            vertical_spacing: FlexSpacing::Start,
            gap: 0.0,
            width: 320.0,
            height: 60.0,
            border: 0.0,
            fill: None,
            imgui: ui,
            items: vec![],
        }
    }

    pub fn horizontal_spacing(&mut self, spacing: FlexSpacing) -> &mut Self {
        self.horizontal_spacing = spacing;
        self
    }

    pub fn vertical_spacing(&mut self, spacing: FlexSpacing) -> &mut Self {
        self.vertical_spacing = spacing;
        self
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn height_auto(&mut self) -> &mut Self {
        self.height = self.items.iter().fold(0.0, |acc, e| {
            println!("e.height: {}, acc: {}", e.height, acc);
            if e.height > acc {
                return e.height;
            }

            acc
        }) + self.border * 2.0;
        self
    }

    pub fn border(&mut self, border: f32) -> &mut Self {
        self.border = border;
        self
    }

    pub fn fill(&mut self, fill: [f32; 4]) -> &mut Self {
        self.fill = Some(fill);
        self
    }

    pub fn button(
        &mut self,
        label: impl AsRef<str>,
        size: [f32; 2],
        f: impl Fn() + 'static,
    ) -> &mut Self {
        self.items.push(UiElement {
            width: size[0],
            height: size[1],
            item_type: UiElementType::Button(label.as_ref().to_string(), Box::new(f)),
        });

        self
    }

    pub fn build(&self) {
        self.build_debug(false);
    }

    pub fn build_debug(&self, debug: bool) {
        let items_width = self.items.iter().map(|i| i.width).sum::<f32>();
        let container_available_width =
            self.imgui.window_content_region_max()[0] - self.border * 4.0;
        let container_available_height =
            self.imgui.window_content_region_max()[1] - self.border * 4.0;

        let available_space_for_gap = container_available_width - items_width;

        let calculated_gap =
            (available_space_for_gap - self.border * 2.0) / (self.items.len() - 1) as f32;

        let width_override: Option<f32> = match self.horizontal_spacing {
            FlexSpacing::Stretch => Some(
                (container_available_width
                    - (self.gap * (self.items.len() as f32 - 1.0)) as f32
                    - self.border * 2.0)
                    / self.items.len() as f32,
            ),
            _ => None,
        };

        self.imgui.new_line();

        self.imgui.group(|| {
            let cursor_start = self.imgui.cursor_screen_pos();
            let p0 = cursor_start;
            let size = [container_available_width, self.height];
            let p1 = [p0[0] + size[0], p0[1] + size[1]];

            self.imgui // TODO add padding
                .set_cursor_screen_pos([
                    cursor_start[0] + self.border,
                    cursor_start[1] + self.border,
                ]);
            let draw_list = self.imgui.get_window_draw_list();

            if let Some(fill) = self.fill {
                draw_list.add_rect(p0, p1, fill).filled(true).build();
            }

            if self.border > 0.0 {
                draw_list
                    .add_rect(p0, p1, [1.0, 1.0, 1.0, 1.0])
                    .thickness(self.border)
                    .build();
            }

            for (i, item) in self.items.iter().enumerate() {
                if i == 0 {
                    match self.horizontal_spacing {
                        FlexSpacing::End => {
                            self.imgui.set_cursor_screen_pos([
                                p0[0] + available_space_for_gap
                                    - self.gap * (self.items.len() - 1) as f32
                                    - self.border,
                                p0[1] + self.border,
                            ]);
                        }
                        _ => (),
                    }
                } else {
                    match self.horizontal_spacing {
                        FlexSpacing::Start => {
                            self.imgui.same_line_with_spacing(0.0, self.gap);
                        }
                        FlexSpacing::End => {
                            self.imgui.same_line_with_spacing(0.0, self.gap);
                        }
                        FlexSpacing::Between => {
                            self.imgui.same_line_with_spacing(0.0, calculated_gap);
                        }
                        FlexSpacing::Stretch => self.imgui.same_line_with_spacing(0.0, self.gap),
                    }
                }

                let w = match width_override {
                    Some(w) => w,
                    None => item.width,
                };

                match &item.item_type {
                    UiElementType::Button(label, f) => {
                        let pressed = self
                            .imgui
                            .button_with_size(label.as_str(), [w, item.height]);
                        if pressed {
                            println!("pressed");
                            f();
                        }
                    }
                    UiElementType::Label(label) => {
                        self.imgui.text(label.as_str());
                    }
                    UiElementType::InputFloat(label) => {
                        self.imgui.input_float(label.as_str(), &mut 0.0).build();
                    }
                    UiElementType::InputInt(label) => {
                        self.imgui.input_int(label.as_str(), &mut 0).build();
                    }
                }
            }

            let cursor_end = p1;
            self.imgui
                .set_cursor_screen_pos([cursor_end[0], cursor_start[1] + self.height]);

            if debug {
                self.imgui.new_line();

                self.imgui.text(format!("self.gap: {}", self.gap).as_str());
                self.imgui
                    .text(format!("self.spacing: {:?}", self.horizontal_spacing).as_str());
                self.imgui
                    .text(format!("self.width: {:?}", self.width).as_str());
                self.imgui
                    .text(format!("self.border: {:?}", self.border).as_str());
                self.imgui.text(format!("p0: {:?}", p0).as_str());
                self.imgui.text(format!("p1: {:?}", p1).as_str());
                self.imgui.text(format!("size: {:?}", size).as_str());
                self.imgui
                    .text(format!("calculated_gap: {}", calculated_gap).as_str());
                self.imgui
                    .text(format!("available_space_for_gap: {}", available_space_for_gap).as_str());
                self.imgui.text(
                    format!("container_available_width: {:?}", container_available_width).as_str(),
                );
                self.imgui
                    .text(format!("imgui.window_size: {:?}", self.imgui.window_size()).as_str());
                self.imgui.text(
                    format!("imgui.item_rect_size: {:?}", self.imgui.item_rect_size()).as_str(),
                );
                self.imgui.text(
                    format!(
                        "imgui.content_region_max: {:?}",
                        self.imgui.window_content_region_max()[0]
                    )
                    .as_str(),
                );
                self.imgui.text(
                    format!("imgui.style.item_inner_spacing: {:?}", unsafe {
                        self.imgui.style().item_inner_spacing
                    })
                    .as_str(),
                );
                self.imgui.text(
                    format!("imgui.style.frame_padding: {:?}", unsafe {
                        self.imgui.style().frame_padding
                    })
                    .as_str(),
                );
                self.imgui.new_line();
            }
        });
    }
}

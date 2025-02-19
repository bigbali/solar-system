use super::{flex::Flex, Override, UiElement, UiNode};

pub struct RootBuilder<'a> {
    parent: &'a mut RootNode,
}

impl<'a> RootBuilder<'a> {
    pub fn flex_row(&mut self) -> &mut Flex {
        self.parent.children.push(UiElement::Flex(Flex::default()));

        match self.parent.children.last_mut().unwrap() {
            UiElement::Flex(flex_row) => flex_row,
            _ => unreachable!("Flex is not flexing :("),
        }
    }
}

pub struct RootNode {
    children: Vec<UiElement>,
}

impl RootNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&mut self, f: impl FnOnce(&mut RootBuilder)) -> &mut Self {
        let mut builder = RootBuilder { parent: self };
        f(&mut builder);
        self
    }

    pub fn build(&mut self, ui: &imgui::Ui) {
        let context = ui;
        let draw_list = context.get_window_draw_list();

        for child in &self.children {
            child.build(&context, &draw_list, Override::default());
        }

        context.new_line();
    }
}

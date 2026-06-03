#[derive(Clone, Default)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree(Vec<Self>);
#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Keep,
    Delete,
}

impl Tree {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn ui(&mut self, ui: &mut egui::Ui, filename: &str) -> Action {
        self.ui_impl(ui, 0, filename)
    }
}

impl Tree {
    fn ui_impl(&mut self, ui: &mut egui::Ui, depth: usize, name: &str) -> Action {
        egui::CollapsingHeader::new(name)
            .default_open(depth < 1)
            .show(ui, |ui| self.children_ui(ui, depth))
            .body_returned
            .unwrap_or(Action::Keep)
    }

    fn children_ui(&mut self, ui: &mut egui::Ui, depth: usize) -> Action {
        if depth > 0
            && ui
                .button(egui::RichText::new("delete").color(ui.visuals().warn_fg_color))
                .clicked()
        {
            return Action::Delete;
        }

        self.0 = std::mem::take(self)
            .0
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui_impl(ui, depth + 1, &format!("child #{i}")) == Action::Keep {
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();

        if ui.button("+").clicked() {
            self.0.push(Self::default());
        }

        Action::Keep
    }
}

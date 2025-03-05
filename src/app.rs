use crate::ui::class_list_panel::ClassListPanel;

pub struct MakeClassApp {
    class_list_panel: ClassListPanel,
}

impl MakeClassApp {
    pub fn new() -> Self {
        Self {
            class_list_panel: ClassListPanel::default(),
        }
    }
}
impl eframe::App for MakeClassApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.class_list_panel.show(ctx);
    }
}

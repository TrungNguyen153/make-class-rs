use eframe::egui::{Color32, Theme};

use crate::{
    global_state::global_state,
    ui::{class_list_panel::ClassListPanel, inspector_panel::InspectorPanel},
};

pub struct MakeClassApp {
    class_list_panel: ClassListPanel,
    inspector: InspectorPanel,
}

impl MakeClassApp {
    pub fn new() -> Self {
        Self {
            class_list_panel: ClassListPanel::default(),
            inspector: InspectorPanel::default(),
        }
    }
}
impl eframe::App for MakeClassApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(Theme::Dark);
        self.class_list_panel.show(ctx);
        if let Some(r) = self.inspector.show(ctx) {
            //
        }

        let mut style = (*ctx.style()).clone();
        let saved = style.clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x10, 0x10, 0x10);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::LIGHT_GRAY;
        ctx.set_style(style);

        global_state().toasts.show(ctx);
        ctx.set_style(saved);
    }
}

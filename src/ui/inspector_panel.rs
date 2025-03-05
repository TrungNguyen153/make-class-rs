use eframe::egui::{CentralPanel, Context, Key, Ui, collapsing_header::CollapsingState};

use crate::{
    address::parse_address_str, field::FieldResponse, global_state::global_state,
    inspection::InspectorContext, styling::get_current_font_size_hex_view,
};

pub struct InspectorPanel {
    address_buffer: String,
    allow_scroll: bool,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            address_buffer: format!("{:#X}", 0),
            allow_scroll: true,
        }
    }
}

impl InspectorPanel {
    pub fn show(&mut self, ctx: &Context) -> Option<FieldResponse> {
        let mut r = None;

        CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.style_mut().override_font_id = Some(get_current_font_size_hex_view());

                let Some(active_class) = global_state().class_list.selected_class() else {
                    ui.centered_and_justified(|ui| {
                        ui.heading(obfstr!(
                            "Select a class from the class list to begin inspection."
                        ));
                    });
                    return;
                };

                r = CollapsingState::load_with_default_open(
                    ctx,
                    obfstring!("_inspector_panel").into(),
                    true,
                )
                .show_header(ui, |ui| {
                    //
                    ui.label(format!("{} -", active_class.name));
                    ui.spacing_mut().text_edit_width = self
                        .address_buffer
                        .chars()
                        .map(|c| ui.fonts(|f| f.glyph_width(&get_current_font_size_hex_view(), c)))
                        .sum::<f32>()
                        .max(160.);

                    let r = ui.text_edit_singleline(&mut self.address_buffer);

                    // parse addr on enter
                    if r.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        match parse_address_str(&self.address_buffer) {
                            Ok(addr) => {
                                active_class.address.set(addr);
                            }
                            Err(e) => {
                                global_state().toasts.error(format!("{e}"));
                            }
                        }
                    }

                    // reset it for current data
                    if !r.has_focus() {
                        self.address_buffer = format!("0x{:X}", active_class.address.get());
                    }
                })
                .body(|ui| self.inspector(ui))
                .2;
            });
        });

        r?.inner
    }

    fn inspector(&mut self, ui: &mut Ui) -> Option<FieldResponse> {
        let ctx = InspectorContext {
            selection: todo!(),
            class_container: todo!(),
            address: todo!(),
            offset: todo!(),
            class_list: todo!(),
            toasts: todo!(),
        };
        None
    }
}

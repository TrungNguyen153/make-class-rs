use eframe::egui::{
    CentralPanel, Context, Key, ScrollArea, Ui, collapsing_header::CollapsingState,
};

use crate::{
    address::AddressString, field::FieldResponse, global_state::global_state,
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
                        match AddressString::load_str(&self.address_buffer) {
                            Ok(addr) => {
                                active_class.address.replace(addr);
                            }
                            Err(e) => {
                                global_state().toasts.error(format!("{e}"));
                            }
                        }
                    }

                    // reset it for current data
                    if !r.has_focus() {
                        self.address_buffer = active_class.address.borrow().to_string();
                    }
                })
                .body(|ui| self.inspector(ui))
                .2;
            });
        });

        r?.inner
    }

    fn inspector(&mut self, ui: &mut Ui) -> Option<FieldResponse> {
        let state = global_state();

        let class = state.class_list.selected_class()?;

        let mut ctx = InspectorContext {
            selection: state.selection_field,
            class_container: state.class_list.selected()?,
            address: class.address.borrow().address_value(),
            offset: 0,
            class_list: &state.class_list,
            toasts: &mut state.toasts,
        };

        let response = ScrollArea::vertical()
            .auto_shrink([false, true])
            .hscroll(true)
            .enable_scrolling(self.allow_scroll)
            .show(ui, |ui| {
                match class
                    .fields
                    .iter()
                    .fold(None, |r, f| r.or(f.draw(ui, &mut ctx)))
                {
                    Some(r) => Some(r),
                    None => None,
                }
                //
            });

        state.selection_field = ctx.selection;

        response.inner
    }
}

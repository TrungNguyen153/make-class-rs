use eframe::egui::{self, Context, Id, Modal};

pub enum ModelResponse {
    AcceptAddNBytes(usize),
    AcceptInsertNBytes(usize),
}

#[derive(Default)]
pub struct Modals {
    pub open_add_n_bytes: bool,
    pub open_insert_n_bytes: bool,
    n_bytes: usize,
}

impl Modals {
    pub fn show(&mut self, ctx: &Context) -> Option<ModelResponse> {
        self.add_or_insert_n_bytes_model(ctx)
    }
    pub fn add_or_insert_n_bytes_model(&mut self, ctx: &Context) -> Option<ModelResponse> {
        let mut response = None;
        if self.open_add_n_bytes || self.open_insert_n_bytes {
            let modal = Modal::new(Id::new(obfstr!("AddNBytesModal"))).show(ctx, |ui| {
                ui.set_width(100.);
                let heading = if self.open_add_n_bytes {
                    obfstring!("Add Bytes")
                } else {
                    obfstring!("Insert Bytes")
                };
                ui.heading(heading);

                ui.add(
                    egui::Slider::new(&mut self.n_bytes, 0..=20_000usize).text(obfstr!("Bytes")),
                );

                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("No").clicked() {
                            // wait stable api
                            // ui.close();
                            self.open_add_n_bytes = false;
                            self.open_insert_n_bytes = false;
                        }

                        if ui.button("Accept").clicked() {
                            if self.n_bytes != 0 {
                                if self.open_add_n_bytes {
                                    response.replace(ModelResponse::AcceptAddNBytes(self.n_bytes));
                                } else {
                                    response
                                        .replace(ModelResponse::AcceptInsertNBytes(self.n_bytes));
                                }
                            }
                            // wait stable api
                            // ui.close();
                            self.open_add_n_bytes = false;
                            self.open_insert_n_bytes = false;
                        }
                    },
                );
            });

            if modal.should_close() {
                self.open_add_n_bytes = false;
                self.open_insert_n_bytes = false;
            }
        }
        response
    }
}

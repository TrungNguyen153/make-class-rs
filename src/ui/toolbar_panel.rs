use eframe::egui::{
    self, Button, Color32, Context, CornerRadius, Frame, Margin, RichText, TopBottomPanel,
};

use crate::field::{Field, boolean::BoolField, int::IntField};

pub enum ToolBarResponse {
    ChangeFieldKind(Box<dyn Field>),
}

#[derive(Default)]
pub struct ToolBarPanel {}

impl ToolBarPanel {
    pub fn show(&mut self, ctx: &Context) -> Option<ToolBarResponse> {
        let mut response = None;

        let style = ctx.style();
        let frame = Frame {
            inner_margin: Margin::same(0),
            corner_radius: CornerRadius::ZERO,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        };

        TopBottomPanel::top("_top_bar")
            .frame(frame)
            .show(ctx, |ui| {
                // ui.spacing_mut().item_spacing.x = 0.;
                // ui.visuals_mut().widgets.inactive.corner_radius = CornerRadius::ZERO;

                ui.horizontal(|ui| {
                    ui.menu_button("Project", |ui| {});

                    ui.separator();

                    ui.menu_button("Process", |ui| {});

                    ui.separator();

                    if ui.button("Generator").clicked() {
                        //
                    }

                    ui.separator();

                    if ui.button("Spider").clicked() {
                        //
                    }

                    ui.separator();

                    if ui.button("Notes").clicked() {
                        //
                    }

                    ui.separator();
                });

                ui.horizontal(|ui| {
                    self.field_change_group(ui, &mut response);
                });
            });

        response
    }

    fn field_change_group(&self, ui: &mut egui::Ui, response: &mut Option<ToolBarResponse>) {
        ui.vertical(|ui| {
            if ui
                .add(
                    Button::new(RichText::new("BOOL").color(Color32::GOLD))
                        .fill(Color32::TRANSPARENT),
                )
                .clicked()
            {
                response.replace(ToolBarResponse::ChangeFieldKind(
                    BoolField::default().boxed(),
                ));
            }
        });

        macro_rules! group_type {
            ($([$display:literal, $text_color:ty, $background:ty, $response_type:ty]),* $(,)?) => {
                ui.vertical(|ui| {
                    $(
                        if ui
                            .add(
                                Button::new(RichText::new(obfstr!($display)).color($text_color))
                                .fill($background),
                            )
                                .clicked()
                        {
                            response.replace(ToolBarResponse::ChangeFieldKind(
                                    $response_type.boxed(),
                            ));
                        }
                    )*
                });
            };
        }

        ui.vertical(|ui| {
            if ui
                .add(
                    Button::new(RichText::new("U8").color(Color32::GREEN))
                        .fill(Color32::TRANSPARENT),
                )
                .clicked()
            {
                response.replace(ToolBarResponse::ChangeFieldKind(
                    IntField::<1>::unsigned_default().boxed(),
                ));
            }
        });

        group_type! {
            ["U8", Color32::GREEN, Color32::TRANSPARENT, IntField::<1>::unsigned_default()]
        }
    }
}

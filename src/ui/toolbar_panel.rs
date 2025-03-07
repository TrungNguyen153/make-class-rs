use eframe::egui::{
    self, Button, Color32, Context, CornerRadius, Frame, Margin, RichText, TopBottomPanel,
};

use crate::field::{
    Field,
    boolean::BoolField,
    class_pointer::ClassPointerField,
    float::FloatField,
    hex::HexField,
    int::IntField,
    string::{PointerTextField, TextField},
    vector::VectorField,
};

pub enum ToolBarResponse {
    #[allow(unused)]
    ChangeFieldKind(Box<dyn Field>),
    /// Add bytes BELLOW selection
    AddBytes(usize),
    /// Insert bytes UPPER selection
    InsertBytes(usize),
    AlignHexFields,
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
        macro_rules! group_type {
            ($ui:ident, $([$display:ident, $text_color:expr, $background:expr, $response_type:expr]),* $(,)?) => {
                $(
                    if $ui
                    .add(
                        Button::new(RichText::new(obfstr!(stringify!($display))).color($text_color))
                        .fill($background),
                    )
                    .clicked()
                    {
                        response.replace(ToolBarResponse::ChangeFieldKind(
                                ($response_type)().boxed()
                        ));
                    }
                )*
            };
        }

        ui.vertical(|ui| {
            group_type! {
                ui,
                [hex8, Color32::GREEN, Color32::TRANSPARENT, || HexField::<8>::default()],
                [hex16, Color32::GREEN, Color32::TRANSPARENT, || HexField::<16>::default()],
                [hex32, Color32::GREEN, Color32::TRANSPARENT, || HexField::<32>::default()],
                [hex64, Color32::GREEN, Color32::TRANSPARENT, || HexField::<64>::default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [u8, Color32::GREEN, Color32::TRANSPARENT, || IntField::<8>::unsigned_default()],
                [u16, Color32::GREEN, Color32::TRANSPARENT, || IntField::<16>::unsigned_default()],
                [u32, Color32::GREEN, Color32::TRANSPARENT, || IntField::<32>::unsigned_default()],
                [u64, Color32::GREEN, Color32::TRANSPARENT, || IntField::<64>::unsigned_default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [i8, Color32::LIGHT_GREEN, Color32::TRANSPARENT, || IntField::<8>::signed_default()],
                [i16, Color32::LIGHT_GREEN, Color32::TRANSPARENT, || IntField::<16>::signed_default()],
                [i32, Color32::LIGHT_GREEN, Color32::TRANSPARENT, || IntField::<32>::signed_default()],
                [i64, Color32::LIGHT_GREEN, Color32::TRANSPARENT, || IntField::<64>::signed_default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [f32, Color32::GRAY, Color32::TRANSPARENT, || FloatField::<4>::default()],
                [f64, Color32::GRAY, Color32::TRANSPARENT, || FloatField::<8>::default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [bool, Color32::GOLD, Color32::TRANSPARENT, || BoolField::default()],
                [vec2, Color32::GREEN, Color32::TRANSPARENT, || VectorField::<2>::default()],
                [vec3, Color32::GREEN, Color32::TRANSPARENT, || VectorField::<3>::default()],
                [vec4, Color32::GREEN, Color32::TRANSPARENT, || VectorField::<4>::default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [utf8, Color32::GREEN, Color32::TRANSPARENT, || TextField::<8>::default()],
                [utf16, Color32::GREEN, Color32::TRANSPARENT, || TextField::<16>::default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [ptrUtf8, Color32::GREEN, Color32::TRANSPARENT, || PointerTextField::<8>::default()],
                [ptrUtf16, Color32::GREEN, Color32::TRANSPARENT, || PointerTextField::<16>::default()],
            }
        });

        ui.vertical(|ui| {
            group_type! {
                ui,
                [clsInst, Color32::GREEN, Color32::TRANSPARENT, || ClassPointerField::default()],
                [clsPtr, Color32::GREEN, Color32::TRANSPARENT, || ClassPointerField::default()],
            }
        });
    }
}

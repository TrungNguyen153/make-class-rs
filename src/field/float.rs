use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value, field_tag::FieldTag};

pub struct FloatField<const N: usize> {
    id: FieldId,
    state: FieldState,
}

impl<const N: usize> Default for FloatField<N> {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("float{N}")),
        }
    }
}

impl<const N: usize> FloatField<N> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(name),
        }
    }
}

impl<const N: usize> Field for FloatField<N> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_tag(&self) -> FieldTag {
        if N == 32 {
            FieldTag::Float32
        } else {
            FieldTag::Float64
        }
    }

    fn codegen(&self, generator: &mut dyn crate::generator::Generator) {
        generator.add_field(
            &self.state.name_state.borrow().name,
            self.field_tag(),
            self.field_size(),
            "",
        );
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        Some(&self.state)
    }

    fn field_size(&self) -> usize {
        N
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut buf = [0; N];
        let address = ctx.address + ctx.offset;
        global_state().memory.read_buf(address, &mut buf);

        let mut field_response = None;

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            self.display_field_prelude(ui, ctx, &mut job);

            let r = ui.add(Label::new(job).sense(Sense::click()));
            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            if let Some(r) = self.default_field_popup(ui, ctx, &r) {
                field_response.replace(r);
            }

            self.display_field_name(ui, ctx, &self.state, Color32::LIGHT_RED);
            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || (Value::F32(3.3), Color32::WHITE),
                |buf| match N {
                    4 => {
                        let v = buf.parse::<f32>()?;
                        eyre::bail!("{}{v}", obfstr!("unimplemented write float "))
                    }
                    8 => {
                        let v = buf.parse::<f64>()?;
                        eyre::bail!("{}{v}", obfstr!("unimplemented write float "))
                    }
                    _ => eyre::bail!("Unsupport Float size: {N}"),
                },
            );
        });

        ctx.offset += N;
        field_response
    }
}

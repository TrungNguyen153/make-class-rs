use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value, field_tag::FieldTag};

pub struct BoolField {
    id: FieldId,
    state: FieldState,
}

impl Default for BoolField {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(obfstr!("bool")),
        }
    }
}

impl BoolField {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(name),
        }
    }
}

impl Field for BoolField {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        Some(&self.state)
    }

    fn field_size(&self) -> usize {
        1
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut val = 0u8;
        let address = ctx.address + ctx.offset;

        global_state()
            .memory
            .read_buf(address, std::slice::from_mut(&mut val));

        let valid_bool = val == 0 || val == 1;
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

            self.display_field_name(ui, ctx, &self.state, Color32::GOLD);

            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || {
                    if !valid_bool {
                        (Value::U8(val), Color32::RED)
                    } else {
                        (Value::Bool(val == 1), Color32::GOLD)
                    }
                },
                |buf| match buf.to_lowercase().as_str() {
                    "1" | "true" | "yes" | "on" => {
                        // TODO write 1 into it
                        Ok(())
                    }
                    "0" | "false" | "no" | "off" => {
                        // TODO write 0 into it
                        Ok(())
                    }
                    _ => eyre::bail!("Unsupport boolean: {buf}"),
                },
            );
        });

        ctx.offset += self.field_size();
        field_response
    }

    fn field_tag(&self) -> FieldTag {
        FieldTag::Bool
    }

    fn codegen(&self, generator: &mut dyn crate::generator::Generator) {
        generator.add_field(
            &self.state.name_state.borrow().name,
            self.field_tag(),
            self.field_size(),
            "",
        );
    }
}

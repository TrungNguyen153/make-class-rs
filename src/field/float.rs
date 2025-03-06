use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value};

pub struct FloatField<const N: usize> {
    id: FieldId,
    state: FieldState,
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

    fn name(&self) -> Option<String> {
        Some(self.state.name_state.borrow().name.clone())
    }

    fn set_name(&self, new_name: String) {
        self.state.name_state.borrow_mut().name = new_name;
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

            self.display_field_name(ui, ctx, &self.state, Color32::LIGHT_RED);
            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                Color32::WHITE,
                || Value::F32(3.3),
                |buf| match N {
                    4 => {
                        let v = buf.parse::<f32>()?;
                        info!("{}{v}", obfstr!("Not implement write for: "));
                        // TODO write
                        Ok(())
                    }
                    8 => {
                        let v = buf.parse::<f64>()?;
                        info!("{}{v}", obfstr!("Not implement write for: "));
                        // TODO write
                        Ok(())
                    }
                    _ => eyre::bail!("Unsupport Float size: {N}"),
                },
            );
        });

        ctx.offset += N;
        field_response
    }
}

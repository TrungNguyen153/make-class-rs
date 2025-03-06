use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value};

pub struct VectorField<const N: usize> {
    id: FieldId,
    state: FieldState,
}

impl<const N: usize> Default for VectorField<N> {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("vec{N}")),
        }
    }
}

impl<const N: usize> VectorField<N> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(name),
        }
    }
}

impl<const N: usize> Field for VectorField<N> {
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
        N * 4
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut buf = [0_f32; N];
        let address = ctx.address + ctx.offset;
        global_state().memory.read_buf(address, unsafe {
            std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut f32 as *mut u8, buf.len() * 4)
        });

        let mut response = None;
        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            self.display_field_prelude(ui, ctx, &mut job);

            let r = ui.add(Label::new(job).sense(Sense::click()));
            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            self.display_field_name(ui, ctx, &self.state, Color32::LIGHT_GREEN);
            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || match N {
                    2 => (Value::Vec2(buf[0], buf[1]), Color32::WHITE),
                    3 => (Value::Vec3(buf[0], buf[1], buf[2]), Color32::WHITE),
                    4 => (Value::Vec4(buf[0], buf[1], buf[2], buf[3]), Color32::WHITE),
                    _ => (
                        Value::String(format!("Invalid IntField size {N}")),
                        Color32::RED,
                    ),
                },
                |_buf| eyre::bail!("unimplemented"),
            );
        });

        ctx.offset += self.field_size();
        response
    }
}

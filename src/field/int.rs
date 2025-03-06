use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value};

pub struct IntField<const N: usize> {
    id: FieldId,
    signed: bool,
    state: FieldState,
}

impl<const N: usize> IntField<N> {
    pub fn signed(name: impl Into<String>) -> Self {
        Self {
            id: FieldId::next_id(),
            signed: true,
            state: FieldState::new(name),
        }
    }

    pub fn unsigned(name: impl Into<String>) -> Self {
        Self {
            id: FieldId::next_id(),
            signed: false,
            state: FieldState::new(name),
        }
    }

    pub fn signed_default() -> Self {
        Self {
            id: FieldId::next_id(),
            signed: true,
            state: FieldState::new(format!("i{N}")),
        }
    }

    pub fn unsigned_default() -> Self {
        Self {
            id: FieldId::next_id(),
            signed: true,
            state: FieldState::new(format!("u{N}",)),
        }
    }
}

impl<const N: usize> Field for IntField<N> {
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
        N / 8
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut buf = vec![0; N / 8];
        let address = ctx.address + ctx.offset;
        global_state().memory.read_buf(address, &mut buf);

        let mut response = None;
        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            self.display_field_prelude(ui, ctx, &mut job);

            let r = ui.add(Label::new(job).sense(Sense::click()));
            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            self.display_field_name(
                ui,
                ctx,
                &self.state,
                if self.signed {
                    Color32::LIGHT_BLUE
                } else {
                    Color32::LIGHT_GREEN
                },
            );

            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || match N {
                    8 => {
                        if self.signed {
                            (Value::I8(buf[0] as i8), Color32::WHITE)
                        } else {
                            (Value::U8(buf[0]), Color32::WHITE)
                        }
                    }
                    16 => {
                        if self.signed {
                            (
                                Value::I16(i16::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        } else {
                            (
                                Value::U16(u16::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        }
                    }
                    32 => {
                        if self.signed {
                            (
                                Value::I32(i32::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        } else {
                            (
                                Value::U32(u32::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        }
                    }
                    64 => {
                        if self.signed {
                            (
                                Value::I64(i64::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        } else {
                            (
                                Value::U64(u64::from_ne_bytes(buf[..].try_into().unwrap())),
                                Color32::WHITE,
                            )
                        }
                    }
                    _ => (
                        Value::String(format!("Invalid IntField size {N}")),
                        Color32::RED,
                    ),
                },
                |_buf| eyre::bail!("Not implement write"),
            );
        });

        ctx.offset += self.field_size();
        response
    }
}

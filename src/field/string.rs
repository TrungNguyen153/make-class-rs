// 4 type string utf8/16
// local string
// ptr string

use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value};

pub struct TextField<const TEXT_KIND: usize, const BUFFER_SIZE: usize> {
    id: FieldId,
    state: FieldState,
}

impl<const TEXT_KIND: usize, const BUFFER_SIZE: usize> Default
    for TextField<TEXT_KIND, BUFFER_SIZE>
{
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("Utf{TEXT_KIND}")),
        }
    }
}

impl<const TEXT_KIND: usize, const BUFFER_SIZE: usize> Field for TextField<TEXT_KIND, BUFFER_SIZE> {
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
        if TEXT_KIND == 8 {
            BUFFER_SIZE
        } else {
            BUFFER_SIZE * 2
        }
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut buffer = vec![0u8; BUFFER_SIZE * 2];
        let address = ctx.address + ctx.offset;

        if TEXT_KIND == 8 {
            global_state()
                .memory
                .read_buf(address, &mut buffer[..BUFFER_SIZE]);
        } else {
            global_state().memory.read_buf(address, &mut buffer[..]);
        }

        let mut response = None;
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
                || match TEXT_KIND {
                    8 => {
                        let s1 = String::from_utf8_lossy(&buffer[..BUFFER_SIZE]);
                        (Value::String(s1.to_string()), Color32::LIGHT_BLUE)
                    }
                    16 => {
                        let (_, b, _) = unsafe { buffer.align_to::<u16>() };
                        let s1 = String::from_utf16_lossy(b);
                        (Value::String(s1), Color32::LIGHT_BLUE)
                    }
                    _ => (Value::String(format!("Invalid TextKind")), Color32::RED),
                },
                |_b| eyre::bail!("unimplemented"),
            );
        });
        ctx.offset += self.field_size();
        response
    }
}

pub struct PointerTextField<const TEXT_KIND: usize, const BUFFER_SIZE: usize> {
    id: FieldId,
    state: FieldState,
}

impl<const TEXT_KIND: usize, const BUFFER_SIZE: usize> Default
    for PointerTextField<TEXT_KIND, BUFFER_SIZE>
{
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("PtrUtf{TEXT_KIND}")),
        }
    }
}

impl<const TEXT_KIND: usize, const BUFFER_SIZE: usize> Field
    for PointerTextField<TEXT_KIND, BUFFER_SIZE>
{
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
        8
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let mut buffer = vec![0u8; BUFFER_SIZE * 2];
        let address = ctx.address + ctx.offset;

        let mut ptr_buf = [0u8; 8];
        global_state().memory.read_buf(address, &mut ptr_buf);

        let buf_addr = usize::from_ne_bytes(ptr_buf);
        if TEXT_KIND == 8 {
            global_state()
                .memory
                .read_buf(buf_addr, &mut buffer[..BUFFER_SIZE]);
        } else {
            global_state().memory.read_buf(buf_addr, &mut buffer[..]);
        }

        let mut response = None;
        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            self.display_field_prelude(ui, ctx, &mut job);

            let r = ui.add(Label::new(job).sense(Sense::click()));
            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            self.display_field_name(ui, ctx, &self.state, Color32::LIGHT_RED);

            self.display_ptr_arrow(ui, ctx, Color32::LIGHT_BLUE);

            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || match TEXT_KIND {
                    8 => {
                        let s1 = String::from_utf8_lossy(&buffer[..BUFFER_SIZE]);
                        (Value::String(s1.to_string()), Color32::LIGHT_BLUE)
                    }
                    16 => {
                        let (_, b, _) = unsafe { buffer.align_to::<u16>() };
                        let s1 = String::from_utf16_lossy(b);
                        (Value::String(s1), Color32::LIGHT_BLUE)
                    }
                    _ => (Value::String(format!("Invalid TextKind")), Color32::RED),
                },
                |_b| eyre::bail!("unimplemented"),
            );
        });

        ctx.offset += self.field_size();
        response
    }
}

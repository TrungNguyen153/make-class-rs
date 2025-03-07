// 4 type string utf8/16
// local string
// ptr string

use std::cell::{Cell, RefCell};

use eframe::egui::{Color32, Label, Sense, text::LayoutJob};

use crate::{global_state::global_state, value::Value};

use super::{Field, FieldId, FieldState, display_field_value};

pub struct TextField<const TEXT_KIND: usize> {
    id: FieldId,
    state: FieldState,
    buffer: RefCell<Vec<u8>>,
    char_count: Cell<usize>,
}

impl<const TEXT_KIND: usize> Default for TextField<TEXT_KIND> {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("utf{TEXT_KIND}")),
            buffer: RefCell::new(vec![]),
            char_count: 0.into(),
        }
    }
}

impl<const TEXT_KIND: usize> TextField<TEXT_KIND> {
    pub fn change_char_count(&self, new: usize) {
        self.char_count.set(new);
    }
}

impl<const TEXT_KIND: usize> Field for TextField<TEXT_KIND> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        Some(&self.state)
    }

    fn field_size(&self) -> usize {
        let char_count = self.char_count.get();
        if TEXT_KIND == 8 {
            char_count
        } else {
            char_count * 2
        }
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let address = ctx.address + ctx.offset;
        let mut alloc_size = self.char_count.get();

        if TEXT_KIND == 16 {
            alloc_size *= 2;
        }

        if self.buffer.borrow().len() != alloc_size {
            self.buffer.borrow_mut().resize(self.char_count.get(), 0);
        }

        global_state()
            .memory
            .read_buf(address, &mut *self.buffer.borrow_mut());

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
                || match TEXT_KIND {
                    8 => {
                        let b = self.buffer.borrow();
                        let s1 = String::from_utf8_lossy(b.as_slice());
                        (Value::String(s1.to_string()), Color32::LIGHT_BLUE)
                    }
                    16 => {
                        let b = self.buffer.borrow();
                        let (_, b, _) = unsafe { b.align_to::<u16>() };
                        let s1 = String::from_utf16_lossy(b);
                        (Value::String(s1), Color32::LIGHT_BLUE)
                    }
                    _ => (Value::String(format!("Invalid TextKind")), Color32::RED),
                },
                |_b| eyre::bail!("unimplemented"),
            );
        });
        ctx.offset += self.field_size();
        field_response
    }
}

pub struct PointerTextField<const TEXT_KIND: usize> {
    id: FieldId,
    state: FieldState,
    buffer: RefCell<Vec<u8>>,
    char_count: Cell<usize>,
}

impl<const TEXT_KIND: usize> Default for PointerTextField<TEXT_KIND> {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new(format!("ptr-utf{TEXT_KIND}")),
            buffer: vec![].into(),
            char_count: 0.into(),
        }
    }
}

impl<const TEXT_KIND: usize> PointerTextField<TEXT_KIND> {
    pub fn change_character_count(&self, new: usize) {
        self.char_count.set(new);
    }
}

impl<const TEXT_KIND: usize> Field for PointerTextField<TEXT_KIND> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        Some(&self.state)
    }

    fn field_size(&self) -> usize {
        8
    }

    fn draw(
        &self,
        ui: &mut eframe::egui::Ui,
        ctx: &mut crate::inspection::InspectorContext,
    ) -> Option<super::FieldResponse> {
        let address = ctx.address + ctx.offset;
        let mut alloc_size = self.char_count.get();
        if TEXT_KIND == 16 {
            alloc_size *= 2;
        }

        if self.buffer.borrow().len() != alloc_size {
            self.buffer.borrow_mut().resize(self.char_count.get(), 0);
        }

        let mut ptr_buf = [0u8; 8];
        global_state().memory.read_buf(address, &mut ptr_buf);

        let buf_addr = usize::from_ne_bytes(ptr_buf);

        global_state()
            .memory
            .read_buf(buf_addr, &mut *self.buffer.borrow_mut());

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

            self.display_ptr_arrow(ui, ctx, Color32::LIGHT_BLUE);

            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || match TEXT_KIND {
                    8 => {
                        let b = self.buffer.borrow();
                        let s1 = String::from_utf8_lossy(b.as_slice());
                        (Value::String(s1.to_string()), Color32::LIGHT_BLUE)
                    }
                    16 => {
                        let b = self.buffer.borrow();
                        let (_, b, _) = unsafe { b.align_to::<u16>() };
                        let s1 = String::from_utf16_lossy(b);
                        (Value::String(s1), Color32::LIGHT_BLUE)
                    }
                    _ => (Value::String(format!("Invalid TextKind")), Color32::RED),
                },
                |_b| eyre::bail!("unimplemented"),
            );
        });

        ctx.offset += self.field_size();
        field_response
    }
}

pub mod hex;

use std::sync::atomic::AtomicU64;

use eframe::egui::{self, Color32, Key, Modifiers, Stroke, text::LayoutJob};

use crate::{
    global_state::global_state,
    inspection::InspectorContext,
    styling::{create_text_format, want_display_offset_prelude},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct FieldId(u64);

static FIELD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl FieldId {
    pub fn next_id() -> FieldId {
        FieldId(FIELD_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
pub enum FieldResponse {
    //
}

pub trait Field {
    fn id(&self) -> FieldId;

    fn name(&self) -> Option<String>;

    fn set_name(&self, new_name: String);

    fn field_size(&self) -> usize;

    fn draw(&self, ui: &mut egui::Ui, ctx: &mut InspectorContext) -> Option<FieldResponse>;

    fn display_field_prelude(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        job: &mut LayoutJob,
    ) {
        let egui_ctx = ui.ctx();
        job.append(
            &if want_display_offset_prelude() {
                format!("{:04X}", ctx.offset)
            } else {
                format!("{:04}", ctx.offset)
            },
            0.,
            {
                let mut tf = create_text_format(ctx.is_selected(self.id()), Color32::KHAKI);
                // Highlight unaligned fields
                if ctx.offset % 8 != 0 {
                    tf.underline = Stroke::new(1., Color32::RED);
                }

                // Ctrl C
                // copy offset
                if egui_ctx.input(|i| {
                    i.key_pressed(Key::C) && i.modifiers.matches_logically(Modifiers::CTRL)
                }) && ctx.is_selected(self.id())
                {
                    egui_ctx.copy_text(format!("{:X}", ctx.address + ctx.offset));
                }

                // Ctrl+Shift C
                // copy 8 bytes at address
                if egui_ctx.input(|i| {
                    i.key_pressed(Key::C)
                        && i.modifiers
                            .matches_logically(Modifiers::CTRL | Modifiers::SHIFT)
                }) && ctx.is_selected(self.id())
                {
                    let mut buf = [0; 8];
                    global_state()
                        .memory
                        .read_buf(ctx.address + ctx.offset, &mut buf[..]);
                    egui_ctx.copy_text(format!("{:X}", usize::from_ne_bytes(buf)));
                }

                tf
            },
        );
        job.append(
            &format!("{:012X}", ctx.address + ctx.offset),
            8.,
            create_text_format(ctx.is_selected(self.id()), Color32::LIGHT_GREEN),
        );
    }
}

use eframe::egui::{Color32, Label, Sense, Ui, text::LayoutJob};

use crate::{
    global_state::global_state, inspection::InspectorContext, styling::create_text_format,
};

use super::{Field, FieldId, FieldResponse};

pub struct HexField<const N: usize> {
    id: FieldId,
}

impl<const N: usize> Default for HexField<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> HexField<N> {
    pub fn new() -> Self {
        Self {
            id: FieldId::next_id(),
        }
    }
    fn byte_view(&self, ctx: &mut InspectorContext, job: &mut LayoutJob, buf: &[u8]) {
        for (i, b) in buf.iter().enumerate() {
            let b = *b;
            // generate unique color for each byte
            // by it's seed
            let mut rng = fastrand::Rng::with_seed(b as _);
            let color = if b == 0 {
                Color32::GRAY
            } else {
                const MIN: std::ops::RangeFrom<u8> = 45..;
                Color32::from_rgb(rng.u8(MIN), rng.u8(MIN), rng.u8(MIN))
            };

            let leading_space = 4. + if i == 0 { 4. } else { 0. };

            job.append(
                &format!("{b:02X}"),
                leading_space,
                create_text_format(ctx.is_selected(self.id), color),
            );
        }
    }

    fn int_view(&self, ui: &mut Ui, ctx: &mut InspectorContext, buf: &[u8]) {
        let mut job = LayoutJob::default();
        let (mut high, mut low) = (0i64, 0i64);
        let displayed = if N == 8 {
            buf[0] as i64
        } else {
            let half = N / 8 / 2;
            (high, low) = int_high_low_from_le::<N>(&buf[..half], &buf[half..]);

            match N {
                16 => i16::from_le_bytes(buf[..].try_into().unwrap()) as i64,
                32 => i32::from_le_bytes(buf[..].try_into().unwrap()) as i64,
                64 => i64::from_le_bytes(buf[..].try_into().unwrap()),
                _ => unreachable!(),
            }
        };

        job.append(
            &format!("{}", displayed),
            4.,
            create_text_format(ctx.is_selected(self.id), Color32::LIGHT_BLUE),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            ctx.toggle_select(self.id);
        }

        if N != 8 {
            r.on_hover_text(format!("High: {high}\nLow: {low}"));
        }
    }

    fn float_view(&self, ui: &mut Ui, ctx: &mut InspectorContext, buf: &[u8]) {
        if N != 32 && N != 64 {
            return;
        }

        let mut job = LayoutJob::default();
        let displayed = if N == 32 {
            f32::from_ne_bytes(buf[..].try_into().unwrap()) as f64
        } else {
            f64::from_ne_bytes(buf[..].try_into().unwrap())
        };

        job.append(
            &format!("{:e}", displayed),
            4.,
            create_text_format(ctx.is_selected(self.id), Color32::LIGHT_RED),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            ctx.toggle_select(self.id);
        }

        if N == 64 {
            let (high, low) = (
                f32::from_ne_bytes(buf[..4].try_into().unwrap()),
                f32::from_ne_bytes(buf[4..].try_into().unwrap()),
            );

            r.on_hover_text(format!("Full:{displayed}\nHigh: {high}\nLow: {low}"));
        } else {
            r.on_hover_text(format!("Full:{displayed}"));
        }
    }

    fn pointer_view(
        &self,
        ui: &mut Ui,
        ctx: &mut InspectorContext,
        buf: &[u8],
        response: &mut Option<FieldResponse>,
    ) {
        if N != 64 {
            return;
        }
        let address = usize::from_ne_bytes(buf[..].try_into().unwrap());

        if global_state().memory.can_read(address) {
            let mut job = LayoutJob::default();
            job.append(
                &format!("-> {address:X}"),
                4.,
                create_text_format(ctx.is_selected(self.id), Color32::YELLOW),
            );

            let r = ui.add(Label::new(job).sense(Sense::click()));

            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            if r.hovered() {
                // todo: preview this
            }
        }
    }
}

impl<const N: usize> Field for HexField<N> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        None
    }

    fn field_size(&self) -> usize {
        N / 8
    }

    fn draw(&self, ui: &mut eframe::egui::Ui, ctx: &mut InspectorContext) -> Option<FieldResponse> {
        let mut buf = vec![0; N / 8];
        global_state()
            .memory
            .read_buf(ctx.address + ctx.offset, &mut buf);
        let mut field_response = None;
        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            self.display_field_prelude(ui, ctx, &mut job);
            self.byte_view(ctx, &mut job, &buf);

            let r = ui.add(Label::new(job).sense(Sense::click()));
            if r.clicked() {
                ctx.toggle_select(self.id);
            }

            if let Some(r) = self.default_field_popup(ui, ctx, &r) {
                field_response.replace(r);
            }

            self.int_view(ui, ctx, &buf);
            self.float_view(ui, ctx, &buf);
            self.pointer_view(ui, ctx, &buf, &mut field_response);
        });
        ctx.offset += self.field_size();
        field_response
    }
}

fn int_high_low_from_le<const N: usize>(high: &[u8], low: &[u8]) -> (i64, i64) {
    // info!("N={N} high={high:?} low={low:?}");
    match N {
        64 => (
            i32::from_ne_bytes(high.try_into().unwrap()) as _,
            i32::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        32 => (
            i16::from_ne_bytes(high.try_into().unwrap()) as _,
            i16::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        16 => (
            i8::from_ne_bytes(high.try_into().unwrap()) as _,
            i8::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        _ => unreachable!(),
    }
}

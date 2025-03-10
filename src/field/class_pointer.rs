use std::cell::Cell;

use eframe::egui::{
    self, Color32, Id, Label, Sense, TextFormat, collapsing_header::CollapsingState,
    popup_below_widget, text::LayoutJob,
};

use crate::{
    class::ClassId, global_state::global_state, inspection::InspectorContext,
    styling::create_text_format, value::Value,
};

use super::{Field, FieldId, FieldResponse, FieldState, display_field_value, field_tag::FieldTag};

pub struct ClassPointerField {
    id: FieldId,
    state: FieldState,
    class_id: Cell<ClassId>,
}

impl Default for ClassPointerField {
    fn default() -> Self {
        Self {
            id: FieldId::next_id(),
            state: FieldState::new("PtrClass"),
            class_id: ClassId::default().into(),
        }
    }
}

impl ClassPointerField {
    pub fn new_with_class_id(class_id: ClassId) -> Self {
        let s = Self::default();
        s.class_id.set(class_id);
        s
    }

    fn show_header(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        address: usize,
    ) -> Option<FieldResponse> {
        let class = ctx.class_list.get_class(self.class_id.get());

        let mut field_response = None;
        let (text, exists) = if let Some(cl) = class {
            (format!("[{}]", cl.name), true)
        } else {
            (format!("[C{:X}]", address), false)
        };

        let mut job = LayoutJob::default();

        self.display_field_prelude(ui, ctx, &mut job);
        job.append(" ", 0., TextFormat::default());

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            ctx.toggle_select(self.id);
        }

        if let Some(r) = self.default_field_popup(ui, ctx, &r) {
            field_response.replace(r);
        }
        self.display_field_name(ui, ctx, &self.state, Color32::GREEN);

        self.display_ptr_arrow(ui, ctx, Color32::YELLOW);

        display_field_value(
            self,
            ui,
            ctx,
            &self.state,
            || (Value::HexAddress(address), Color32::YELLOW),
            |_b| eyre::bail!("unimplemented"),
        );

        let mut job = LayoutJob::default();
        job.append(
            &text,
            4.,
            create_text_format(
                ctx.is_selected(self.id),
                if exists {
                    Color32::LIGHT_GRAY
                } else {
                    Color32::DARK_GRAY
                },
            ),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        let popup_select_exist_class_id = Id::new(format!(
            "{:?}{}{address}popup_select_exist_class",
            self.id, ctx.inspector_level,
        ));
        if r.secondary_clicked() {
            ui.memory_mut(|m| m.toggle_popup(popup_select_exist_class_id));
        } else if r.clicked() {
            ctx.toggle_select(self.id);
        }

        popup_below_widget(
            ui,
            popup_select_exist_class_id,
            &r,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_width(80.);
                ui.vertical_centered_justified(|ui| {
                    for cl in ctx.class_list.classes() {
                        if ui.button(&cl.name).clicked() {
                            self.class_id.set(cl.id());
                            ui.memory_mut(|m| m.toggle_popup(popup_select_exist_class_id));
                        }
                    }
                });
            },
        );
        field_response
    }

    fn show_body(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        address: usize,
    ) -> Option<FieldResponse> {
        let cid = self.class_id.get();
        let mut response = None;
        let Some(class) = ctx.class_list.get_class(cid) else {
            // TODO request make new class here
            return None;
        };

        let mut inner_ctx = InspectorContext {
            selection: ctx.selection,
            class_container: cid,
            address,
            offset: 0,
            class_list: ctx.class_list,
            toasts: ctx.toasts,
            inspector_level: ctx.inspector_level + 1,
        };

        for f in class.fields.iter() {
            response = response.or(f.draw(ui, &mut inner_ctx));
        }

        ctx.selection = inner_ctx.selection;

        response
    }
}

impl Field for ClassPointerField {
    fn id(&self) -> FieldId {
        self.id
    }

    fn field_tag(&self) -> FieldTag {
        FieldTag::ClassPointer
    }

    fn codegen(&self, generator: &mut dyn crate::generator::Generator) {
        let class_name = global_state()
            .class_list
            .get_class(self.class_id.get())
            .map(|c| c.name.clone());
        generator.add_field(
            &self.state.name_state.borrow().name,
            self.field_tag(),
            self.field_size(),
            &class_name.unwrap_or_default(),
        );
    }

    fn field_state(&self) -> Option<&super::FieldState> {
        Some(&self.state)
    }

    fn field_size(&self) -> usize {
        8
    }

    fn draw(&self, ui: &mut egui::Ui, ctx: &mut InspectorContext) -> Option<FieldResponse> {
        let mut response = None;

        // deref this ptr
        let address = ctx.address + ctx.offset;
        let mut ptr_buf = [0; 8];
        global_state().memory.read_buf(address, &mut ptr_buf);
        let deref_address = usize::from_ne_bytes(ptr_buf);

        let collapsing_id = Id::new(format!("{:?}{}{address}", self.id, ctx.inspector_level));

        let state = CollapsingState::load_with_default_open(ui.ctx(), collapsing_id, false);

        let body = state
            .show_header(ui, |ui| self.show_header(ui, ctx, deref_address))
            .body(|ui| self.show_body(ui, ctx, deref_address));

        if let Some(new) = body.2.and_then(|inner| inner.inner) {
            response = Some(new);
        }

        if let Some(new) = body.1.inner {
            response = Some(new);
        }

        ctx.offset += self.field_size();
        response
    }
}

pub mod boolean;
pub mod class_instance;
pub mod class_pointer;
pub mod field_tag;
pub mod float;
pub mod hex;
pub mod int;
pub mod string;
pub mod vector;

use std::{cell::RefCell, sync::atomic::AtomicU64};

use eframe::egui::{
    self, Color32, FontSelection, Id, Key, Label, Modifiers, Sense, TextEdit, popup_below_widget,
    text::LayoutJob,
};

use crate::{
    generator::Generator,
    global_state::global_state,
    inspection::InspectorContext,
    styling::{create_text_format, create_text_offset_format, get_current_font_size_hex_view},
    value::Value,
};

use self::{field_tag::FieldTag, hex::HexField};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct FieldId(u64);

static FIELD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl FieldId {
    pub fn next_id() -> FieldId {
        FieldId(FIELD_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

pub struct FieldMemoryEditingState {
    address: usize,
    buf: String,
    request_focus: bool,
}

#[derive(Default)]
pub struct FieldNameState {
    name: String,
    name_before_edit: String,
    request_focus_name_edit: bool,
    editing: bool,
}

impl FieldNameState {
    pub fn validate_consume_name(&mut self) -> eyre::Result<()> {
        // TODO Validate name
        self.name_before_edit.clear();
        Ok(())
    }
}

#[derive(Default)]
pub struct FieldState {
    name_state: RefCell<FieldNameState>,
    memory_editing_state: RefCell<Option<FieldMemoryEditingState>>,
}

impl FieldState {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name_state: RefCell::new(FieldNameState {
                name: name.into(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldResponse {
    AddBytes(usize),
    InsertBytes(usize),
    Delete,
    AddNBytes,
    InsertNBytes,
}

pub trait Field {
    fn id(&self) -> FieldId;

    fn field_tag(&self) -> FieldTag;
    fn codegen(&self, generator: &mut dyn Generator);

    fn field_state(&self) -> Option<&FieldState>;

    fn name(&self) -> Option<String> {
        if let Some(state) = self.field_state() {
            return Some(state.name_state.borrow().name.clone());
        }
        None
    }

    fn set_name(&self, new_name: String) {
        if let Some(state) = self.field_state() {
            state.name_state.borrow_mut().name = new_name;
        }
    }

    fn had_name(&self) -> bool {
        if let Some(state) = self.field_state() {
            return !state.name_state.borrow().name.is_empty();
        }
        false
    }

    fn field_size(&self) -> usize;

    fn boxed(self) -> Box<dyn Field>
    where
        Self: Sized + 'static,
    {
        Box::new(self) as Box<dyn Field>
    }

    fn draw(&self, ui: &mut egui::Ui, ctx: &mut InspectorContext) -> Option<FieldResponse>;

    fn display_field_prelude(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        job: &mut LayoutJob,
    ) {
        let egui_ctx = ui.ctx();
        job.append(&create_text_offset_format(ctx.offset), 0., {
            let tf = create_text_format(ctx.is_selected(self.id()), Color32::KHAKI);
            // Highlight unaligned fields
            // if ctx.offset % 8 != 0 {
            //     tf.underline = Stroke::new(1., Color32::RED);
            // }

            // Ctrl C
            // copy offset
            if egui_ctx
                .input(|i| i.key_pressed(Key::C) && i.modifiers.matches_logically(Modifiers::CTRL))
                && ctx.is_selected(self.id())
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
        });
        job.append(
            &format!("{:012X}", ctx.address + ctx.offset),
            8.,
            create_text_format(ctx.is_selected(self.id()), Color32::LIGHT_GREEN),
        );
    }

    fn display_field_name(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        state: &FieldState,
        color: Color32,
    ) {
        if state.name_state.borrow().editing {
            let name_state = &mut *state.name_state.borrow_mut();

            let w = name_state
                .name
                .chars()
                .map(|c| ui.fonts(|f| f.glyph_width(&get_current_font_size_hex_view(), c)))
                .sum::<f32>()
                .max(80.)
                + 32.;

            let r = TextEdit::singleline(&mut name_state.name)
                .desired_width(w)
                .font(FontSelection::FontId(get_current_font_size_hex_view()))
                .show(ui)
                .response;

            if r.clicked_elsewhere() {
                // cancel edit
                name_state.name = std::mem::take(&mut name_state.name_before_edit);
                name_state.editing = false;
            } else if r.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                // enter only
                if let Err(e) = name_state.validate_consume_name() {
                    ctx.toasts
                        .error(format!("{}\n{e}", obfstr!("Not a valid field name:")));
                    name_state.request_focus_name_edit = true;
                } else {
                    // finished editing
                    name_state.editing = false;
                }
            }

            if name_state.request_focus_name_edit {
                r.request_focus();
                name_state.request_focus_name_edit = false;
            }
        } else {
            // display normal text
            let name_state = &mut *state.name_state.borrow_mut();
            let mut job = LayoutJob::default();
            job.append(
                &name_state.name,
                0.,
                create_text_format(ctx.is_selected(self.id()), color),
            );

            let r = ui.add(Label::new(job).sense(Sense::click()));

            if r.double_clicked() {
                // renaming for double click
                name_state.editing = true;
                name_state.request_focus_name_edit = true;
                // prepare name before edit
                name_state.name_before_edit = name_state.name.clone();
            } else if r.clicked() {
                // or select with single click
                ctx.toggle_select(self.id());
            }
        }
    }

    fn display_ptr_arrow(&self, ui: &mut egui::Ui, ctx: &mut InspectorContext, color: Color32) {
        let mut job = LayoutJob::default();
        job.append("->", 0., create_text_format(false, color));

        let r = ui.add(Label::new(job).sense(Sense::click()));

        if r.clicked() {
            // or select with single click
            ctx.toggle_select(self.id());
        }
    }

    fn default_field_popup(
        &self,
        ui: &mut egui::Ui,
        ctx: &mut InspectorContext,
        below_widget_response: &egui::Response,
    ) -> Option<FieldResponse> {
        let mut response = None;

        let popup_id = Id::new(format!(
            "{:?}{}{:?}popup_widget",
            self.id(),
            ctx.inspector_level,
            ctx.class_container
        ));

        // TODO
        // wait for egui::popup::Popup
        // release
        popup_below_widget(
            ui,
            popup_id,
            below_widget_response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_width(80.);
                ui.menu_button(obfstr!("Insert Bytes"), |ui| {
                    for b in [4, 8, 64, 256, 1024, 2048, 4096usize] {
                        if ui.button(format!("Insert {b}")).clicked() {
                            response.replace(FieldResponse::InsertBytes(b));
                            ui.close_menu();
                            ui.memory_mut(|m| m.toggle_popup(popup_id));
                        }
                    }
                    if ui.button(obfstr!("Insert ... Bytes")).clicked() {
                        ui.close_menu();
                        ui.memory_mut(|m| m.toggle_popup(popup_id));
                        response.replace(FieldResponse::InsertNBytes);
                    }
                });

                ui.menu_button(obfstr!("Add Bytes"), |ui| {
                    for b in [4, 8, 64, 256, 1024, 2048, 4096usize] {
                        if ui.button(format!("Add {b}")).clicked() {
                            response.replace(FieldResponse::AddBytes(b));
                            ui.close_menu();
                            ui.memory_mut(|m| m.toggle_popup(popup_id));
                        }
                    }
                    if ui.button(obfstr!("Add ... Bytes")).clicked() {
                        ui.close_menu();
                        ui.memory_mut(|m| m.toggle_popup(popup_id));
                        response.replace(FieldResponse::AddNBytes);
                    }
                });

                if ui.button(obfstr!("Delete")).clicked() {
                    response.replace(FieldResponse::Delete);
                    ui.memory_mut(|m| m.toggle_popup(popup_id));
                }
            },
        );

        if below_widget_response.secondary_clicked() {
            ui.memory_mut(|m| m.toggle_popup(popup_id));
        }

        // let bellow for trigger select underline
        if below_widget_response.secondary_clicked() && !ctx.is_selected(self.id()) {
            ctx.toggle_select(self.id());
        }
        response
    }
}

// unable inside object safe for dyn Trait object
pub fn display_field_value(
    field: &dyn Field,
    ui: &mut egui::Ui,
    ctx: &mut InspectorContext,
    state: &FieldState,
    display_value_fn: impl FnOnce() -> (Value, Color32),
    writer_new_value_fn: impl FnOnce(&str) -> eyre::Result<()>,
) {
    let field_address = ctx.address + ctx.offset;
    let edit_state = &mut *state.memory_editing_state.borrow_mut();
    if let Some(FieldMemoryEditingState {
        address,
        buf,
        request_focus,
    }) = edit_state
    {
        // we in editing memory mode
        *address = field_address;

        let mut w = buf
            .chars()
            .map(|c| ui.fonts(|f| f.glyph_width(&get_current_font_size_hex_view(), c)))
            .sum::<f32>();
        if w > 80. {
            w += 10.
        } else {
            w = 80.
        };

        let r = TextEdit::singleline(buf).desired_width(w).show(ui).response;

        if *request_focus {
            r.request_focus();
            *request_focus = false;
        }

        if r.clicked_elsewhere() {
            // cancle edit
            edit_state.take();
        } else if r.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            // enter edit
            if let Err(e) = writer_new_value_fn(&buf) {
                // error edit
                // focus again
                ctx.toasts
                    .error(format!("{}\n{e}", obfstr!("Invalid value:")));
                *request_focus = true;
            } else {
                // edit valid
                // clear state edit
                edit_state.take();
            }
        }

        // skip bellow
        return;
    }

    let mut job = LayoutJob::default();

    let (v, color) = display_value_fn();

    job.append(
        &v.to_string(),
        0.,
        create_text_format(ctx.is_selected(field.id()), color),
    );

    let r = ui.add(Label::new(job).sense(Sense::click()));

    if r.double_clicked() {
        // enter edit mode
        edit_state.replace(FieldMemoryEditingState {
            address: field_address,
            buf: v.to_string(),
            request_focus: true,
        });
    } else if r.clicked() {
        ctx.toggle_select(field.id());
    }
}

pub fn allocate_padding(mut n: usize) -> Vec<Box<dyn Field>> {
    let mut fields = vec![];

    while n >= 8 {
        fields.push(Box::new(HexField::<64>::new()) as _);
        n -= 8;
    }

    while n >= 4 {
        fields.push(Box::new(HexField::<32>::new()) as _);
        n -= 4;
    }

    while n >= 2 {
        fields.push(Box::new(HexField::<16>::new()) as _);
        n -= 2;
    }

    while n > 0 {
        fields.push(Box::new(HexField::<8>::new()) as _);
        n -= 1;
    }

    fields
}

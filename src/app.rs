use eframe::egui::{Color32, Theme};

use crate::{
    field::{FieldResponse, allocate_padding},
    global_state::global_state,
    ui::{
        class_list_panel::ClassListPanel,
        inspector_panel::InspectorPanel,
        modals::{Modals, ModelResponse},
        toolbar_panel::{ToolBarPanel, ToolBarResponse},
    },
    utils::offset_align_to,
};

pub struct MakeClassApp {
    class_list_panel: ClassListPanel,
    inspector: InspectorPanel,
    toolbar: ToolBarPanel,
    modals: Modals,
}

impl MakeClassApp {
    pub fn new() -> Self {
        Self {
            class_list_panel: ClassListPanel::default(),
            inspector: InspectorPanel::default(),
            toolbar: ToolBarPanel::default(),
            modals: Modals::default(),
        }
    }

    fn progress_toolbar_response(&mut self, response: ToolBarResponse) {
        match response {
            ToolBarResponse::ChangeFieldKind(new) => {
                let Some(selected) = &mut global_state().selection_field else {
                    return;
                };

                let class_id = selected.class_id;
                let field_id = selected.field_id;

                let Some(class) = global_state().class_list.get_class_mut(class_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[ChangeField] Why class id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                let Some(field_pos) = class.field_pos(field_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[ChangeField] Why field id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                let old_size = class.fields[field_pos].field_size();
                let old_name = class.fields[field_pos].name();
                let new_size = new.field_size();
                let new_id = new.id();

                // load old name to it
                if let Some(old_name) = old_name {
                    new.set_name(old_name);
                }

                if old_size > new_size {
                    let mut padding = allocate_padding(old_size - new_size);
                    class.fields[field_pos] = new;
                    while let Some(pad) = padding.pop() {
                        class.fields.insert(field_pos + 1, pad);
                    }
                    selected.field_id = new_id;
                } else {
                    let (mut steal_size, mut steal_len) = (0, 0);
                    while steal_size < new_size {
                        let index = field_pos + steal_len;
                        if index > class.fields.len() {
                            // out of class size
                            break;
                        }

                        steal_size += class.fields[index].field_size();
                        steal_len += 1;
                    }

                    if steal_size < new_size {
                        global_state()
                            .toasts
                            .error(obfstr!("Not enough space for a new field"));
                    } else {
                        class.fields.drain(field_pos..field_pos + steal_len);

                        let mut padding = allocate_padding(steal_size - new_size);

                        class.fields.insert(field_pos, new);

                        while let Some(pad) = padding.pop() {
                            class.fields.insert(field_pos + 1, pad);
                        }

                        selected.field_id = new_id;
                    }
                }
            }
            ToolBarResponse::AddBytes(b) => {
                global_state()
                    .toasts
                    .info(format!("{} {b} bytes", obfstr!("[AddBytes]")));
                let Some(selected) = &mut global_state().selection_field else {
                    global_state()
                        .toasts
                        .error(obfstr!("[AddBytes] Why we not have selected field"));
                    return;
                };

                let class_id = selected.class_id;
                let field_id = selected.field_id;

                let Some(class) = global_state().class_list.get_class_mut(class_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[AddBytes] Why class id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                if let Err(e) = class.add_bytes(b, field_id) {
                    global_state().toasts.error(format!("{e}"));
                    global_state().selection_field.take();
                }
            }
            ToolBarResponse::InsertBytes(b) => {
                global_state()
                    .toasts
                    .info(format!("{} {b} bytes", obfstr!("[InsertBytes]")));
                let Some(selected) = &mut global_state().selection_field else {
                    global_state()
                        .toasts
                        .error(obfstr!("[InsertBytes] Why we not have selected field"));
                    return;
                };

                let class_id = selected.class_id;
                let field_id = selected.field_id;

                let Some(class) = global_state().class_list.get_class_mut(class_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[InsertBytes] Why class id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                if let Err(e) = class.insert_bytes(b, field_id) {
                    global_state().toasts.error(format!("{e}"));
                    global_state().selection_field.take();
                }
            }
            ToolBarResponse::AlignHexFields => {
                global_state().toasts.info(obfstr!("[AlignFields]"));
                let Some(class) = global_state().class_list.selected_class_mut() else {
                    global_state()
                        .toasts
                        .error(obfstr!("[AlignFields] no active class"));
                    return;
                };

                // start pos 1
                // pos 0 alway align
                let mut iter_pos = 0;
                let mut offset = 0;
                while iter_pos < class.field_len() - 1 {
                    let field = &class.fields[iter_pos];
                    let field_id = field.id();
                    let next_field = &class.fields[iter_pos + 1];
                    let next_field_id = next_field.id();

                    // next field is named
                    // cant add bytes
                    if next_field.had_name() {
                        info!("[{iter_pos}] next field named");
                        // move up 2
                        iter_pos += 2;
                        offset += field.field_size() + next_field.field_size();
                        continue;
                    }

                    // already align
                    if offset % 4 == 0 {
                        info!("[{iter_pos}] already align");
                        iter_pos += 1;
                        offset += field.field_size();
                        continue;
                    }

                    let missing = offset_align_to(offset, 4) - offset;

                    let next_field_size = next_field.field_size();

                    if missing != 0 && missing < next_field_size {
                        // we break next field for add align
                        // insert N bytes
                        // cut out N bytes
                        let steal_size = next_field_size - missing;
                        info!(
                            "[{iter_pos}] progressing: offset={offset} missing={missing} steal={steal_size}"
                        );
                        let added_index = class.insert_bytes(missing, field_id).unwrap();
                        offset += missing;
                        iter_pos += added_index;
                        class.remove_field_by_id(next_field_id).unwrap();
                        class.add_bytes(steal_size, field_id).unwrap();
                    } else {
                        if missing != 0 {
                            info!("Next field not enough byte");
                        }
                        info!("Go next");
                        // go next
                        iter_pos += 1;
                        offset += field.field_size();
                    }
                }

                // group field
                iter_pos = 0;
                while iter_pos < class.field_len() {
                    class.merge_hex_field(iter_pos);
                    iter_pos += 1;
                }
            }
            ToolBarResponse::DeleteField => {
                let Some(selected) = global_state().selection_field.take() else {
                    global_state()
                        .toasts
                        .error(obfstr!("[DeleteField] Why we not have selected field"));
                    return;
                };

                let Some(class) = global_state().class_list.get_class_mut(selected.class_id) else {
                    return;
                };

                if let Err(e) = class.remove_field_by_id(selected.field_id) {
                    global_state().toasts.error(format!("{e}"));
                }
            }
        };
    }
}
impl eframe::App for MakeClassApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(Theme::Dark);
        let mut toolbar_response = self.toolbar.show(ctx);

        self.class_list_panel.show(ctx);
        if let Some(r) = self.inspector.show(ctx) {
            match r {
                FieldResponse::AddBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::AddBytes(b));
                }
                FieldResponse::InsertBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::InsertBytes(b));
                }
                FieldResponse::Delete => {
                    toolbar_response.replace(ToolBarResponse::DeleteField);
                }
                FieldResponse::AddNBytes => {
                    self.modals.open_add_n_bytes = true;
                }
                FieldResponse::InsertNBytes => {
                    self.modals.open_insert_n_bytes = true;
                }
            }
        }

        if let Some(r) = self.modals.show(ctx) {
            match r {
                ModelResponse::AcceptAddNBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::AddBytes(b));
                }
                ModelResponse::AcceptInsertNBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::InsertBytes(b));
                }
            }
        }

        if let Some(toolbar_response) = toolbar_response {
            self.progress_toolbar_response(toolbar_response);
        }

        let mut style = (*ctx.style()).clone();
        let saved = style.clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x10, 0x10, 0x10);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::LIGHT_GRAY;
        ctx.set_style(style);

        global_state().toasts.show(ctx);
        ctx.set_style(saved);
    }
}

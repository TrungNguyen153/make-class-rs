use eframe::egui::{Color32, Theme};

use crate::{
    field::allocate_padding,
    global_state::global_state,
    ui::{
        class_list_panel::ClassListPanel,
        inspector_panel::InspectorPanel,
        toolbar_panel::{ToolBarPanel, ToolBarResponse},
    },
};

pub struct MakeClassApp {
    class_list_panel: ClassListPanel,
    inspector: InspectorPanel,
    toolbar: ToolBarPanel,
}

impl MakeClassApp {
    pub fn new() -> Self {
        Self {
            class_list_panel: ClassListPanel::default(),
            inspector: InspectorPanel::default(),
            toolbar: ToolBarPanel::default(),
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
                global_state().toasts.info(obfstr!("[AddBytes]"));
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

                let Some(field_pos) = class.field_pos(field_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[AddBytes] Why field id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                let padding = allocate_padding(b);

                let field_len = class.field_len();

                if field_pos == field_len {
                    // field at end
                    class.extend_fields(padding);
                } else {
                    for p in padding {
                        class.fields.insert(field_pos + 1, p);
                    }
                }
            }
            ToolBarResponse::InsertBytes(b) => {
                global_state().toasts.info(obfstr!("[InsertBytes]"));
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

                let Some(field_pos) = class.field_pos(field_id) else {
                    global_state()
                        .toasts
                        .error(obfstr!("[InsertBytes] Why field id not here here ??"));
                    global_state().selection_field.take();
                    return;
                };

                let padding = allocate_padding(b);
                for p in padding {
                    class.fields.insert(field_pos, p);
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
                crate::field::FieldResponse::AddBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::AddBytes(b));
                }
                crate::field::FieldResponse::InsertBytes(b) => {
                    toolbar_response.replace(ToolBarResponse::InsertBytes(b));
                }
                crate::field::FieldResponse::Delete => {
                    unimplemented!()
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

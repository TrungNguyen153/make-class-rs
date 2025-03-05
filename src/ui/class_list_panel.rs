use eframe::egui::{
    self, RichText, ScrollArea, SelectableLabel, SidePanel, TextBuffer, TextEdit, vec2,
};

use crate::{class::ClassId, global_state::global_state};

struct StateEditingClassName {
    request_focus_edit: bool,
    edit_buf: String,
    class_id: ClassId,
}

#[derive(Default)]
pub struct ClassListPanel {
    // User enter invalid Class name
    // we focus again
    request_focus_add_name: bool,
    new_class_buf: String,
    edit_class_name_state: Option<StateEditingClassName>,
}

impl ClassListPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        SidePanel::left("Class_List_Panel")
            .default_width(160.)
            .show(ctx, |ui| {
                ui.heading("Classes")
                    .on_hover_cursor(egui::CursorIcon::Default)
                    .on_hover_text("Press ENTER to create a new class");

                // input class name here
                let r = TextEdit::singleline(&mut self.new_class_buf)
                    .desired_width(f32::INFINITY)
                    .hint_text("Create new class")
                    .show(ui)
                    .response;

                if self.request_focus_add_name {
                    r.request_focus();
                    self.request_focus_add_name = false;
                }

                // resolve input new class
                if !self.new_class_buf.is_empty() {
                    // click outside input
                    if r.clicked_elsewhere() {
                        self.new_class_buf.clear();
                    }

                    // pressed escape
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) && r.lost_focus() {
                        self.new_class_buf.clear();
                    }

                    // enter
                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) && r.lost_focus() {
                        info!("Make class {}", self.new_class_buf);
                        // make class + clean buffer if ok
                        // validate input class name
                        // if not valid
                        // focus it again
                        if validate_class_name(&self.new_class_buf) {
                            global_state()
                                .class_list
                                .add_class(self.new_class_buf.take());
                        } else {
                            self.request_focus_add_name = true;
                        }
                    }
                }

                ui.separator();

                ui.vertical(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        let state = global_state();
                        let selected = state.class_list.selected();

                        enum ChangeEvent {
                            Select(ClassId),
                            Unselect,
                            Remove(ClassId),
                        }

                        let mut e = None;

                        for class in state.class_list.classes_mut() {
                            if let Some(StateEditingClassName {
                                request_focus_edit,
                                edit_buf,
                                class_id,
                            }) = &mut self.edit_class_name_state
                                && *class_id == class.id()
                            {
                                // we editing this class
                                let r = TextEdit::singleline(edit_buf)
                                    .desired_width(f32::INFINITY)
                                    .hint_text("New name")
                                    .show(ui)
                                    .response;

                                if *request_focus_edit {
                                    r.request_focus();
                                    *request_focus_edit = false;
                                }

                                if r.clicked_elsewhere() {
                                    self.edit_class_name_state = None;
                                } else if !edit_buf.is_empty() {
                                    // pressed escape
                                    if ui.input(|i| i.key_pressed(egui::Key::Escape))
                                        && r.lost_focus()
                                    {
                                        self.edit_class_name_state = None;
                                    }
                                    // enter
                                    else if ui.input(|i| i.key_pressed(egui::Key::Enter))
                                        && r.lost_focus()
                                    {
                                        info!("Renamed {edit_buf}");
                                        // make class + clean buffer if ok
                                        // validate input class name
                                        // if not valid
                                        // focus it again
                                        if validate_class_name(&edit_buf) {
                                            class.name = edit_buf.take();
                                            self.edit_class_name_state = None;
                                        } else {
                                            *request_focus_edit = true;
                                        }
                                    }
                                }
                                // skip bellow
                                continue;
                            };

                            let is_selected = selected.map(|s| s == class.id()).unwrap_or_default();
                            // main view
                            let r = ui.add_sized(
                                vec2(ui.available_width(), 24.),
                                SelectableLabel::new(is_selected, RichText::new(&class.name)),
                            );

                            // left click handle
                            if r.clicked() {
                                match is_selected {
                                    true => {
                                        // unselected;
                                        e.replace(ChangeEvent::Unselect);
                                    }
                                    false => {
                                        // selected;
                                        e.replace(ChangeEvent::Select(class.id()));
                                    }
                                }

                                // skip bellow
                                continue;
                            }

                            // right click context for class
                            r.context_menu(|ui| {
                                ui.set_width(80.);

                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("Rename").clicked() {
                                        ui.close_menu();

                                        self.edit_class_name_state.replace(StateEditingClassName {
                                            request_focus_edit: true,
                                            edit_buf: String::new(),
                                            class_id: class.id(),
                                        });
                                    }

                                    if ui.button("Delete").clicked() {
                                        ui.close_menu();
                                        // delete class it
                                        e.replace(ChangeEvent::Remove(class.id()));
                                    }
                                });
                            });
                        }

                        if let Some(e) = e {
                            match e {
                                ChangeEvent::Select(id) => {
                                    state.class_list.set_selected(id);
                                }
                                ChangeEvent::Unselect => {
                                    state.class_list.un_select();
                                }
                                ChangeEvent::Remove(id) => {
                                    state.class_list.remove_class(id);
                                }
                            }
                        }
                    });
                })
            });
    }
}

fn validate_class_name(_class_name: &str) -> bool {
    true
}

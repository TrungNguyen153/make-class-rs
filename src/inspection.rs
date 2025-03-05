use egui_notify::Toasts;

use crate::{
    class::{ClassId, class_list::ClassList},
    field::FieldId,
};

pub struct InspectorContext<'a> {
    pub selection: Option<InspectorSelection>,
    pub class_container: ClassId,

    pub address: usize,
    pub offset: usize,

    pub class_list: &'a ClassList,
    pub toasts: &'a mut Toasts,
}

#[derive(Clone, Copy)]
pub struct InspectorSelection {
    pub address: usize,
    pub class_id: ClassId,
    pub field_id: FieldId,
}

impl<'a> InspectorContext<'a> {
    pub fn toggle_select(&mut self, field_id: FieldId) {
        if self.is_selected(field_id) {
            self.selection.take();
        } else {
            self.selection.replace(InspectorSelection {
                address: self.address + self.offset,
                class_id: self.class_container,
                field_id,
            });
        }
    }

    pub fn is_selected(&mut self, field_id: FieldId) -> bool {
        if let Some(s) = self.selection
            && s.field_id == field_id
        {
            return s.address == self.address + self.offset;
        }

        false
    }
}

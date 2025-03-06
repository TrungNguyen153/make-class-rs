use crate::field::{
    Field, class_instance::ClassInstanceField, class_pointer::ClassPointerField, hex::HexField,
};

use super::{Class, ClassId};

pub struct ClassList {
    classes: Vec<Class>,
    selected: Option<ClassId>,
}

impl Default for ClassList {
    fn default() -> Self {
        let c1 = Class::new(0, "Dummy");
        let mut c2 = Class::new(1, "Dummy2");
        c2.add_field(ClassInstanceField::new_with_class_id(0.into()).boxed());
        c2.add_field(HexField::<4>::default().boxed());
        c2.add_field(HexField::<4>::default().boxed());
        let mut c3 = Class::new(2, "Dummy3");
        c3.add_field(ClassPointerField::new_with_class_id(1.into()).boxed());
        c3.add_field(HexField::<4>::default().boxed());
        c3.add_field(HexField::<8>::default().boxed());
        Self {
            classes: vec![c1, c2, c3],
            selected: Some(0.into()),
        }
    }
}

impl ClassList {
    pub fn classes(&self) -> &[Class] {
        &self.classes[..]
    }

    pub fn classes_mut(&mut self) -> &mut [Class] {
        &mut self.classes[..]
    }

    pub fn selected(&self) -> Option<ClassId> {
        self.selected
    }

    pub fn set_selected(&mut self, class_id: ClassId) {
        self.selected.replace(class_id);
    }

    pub fn un_select(&mut self) {
        self.selected.take();
    }

    pub fn selected_class(&self) -> Option<&Class> {
        if let Some(id) = self.selected {
            return self.classes.iter().find(|c| c.id() == id);
        }
        None
    }

    pub fn add_class(&mut self, name: impl Into<String>) -> ClassId {
        let id = fastrand::usize(..);
        self.classes.push(Class::new(id, name));
        id.into()
    }

    pub fn remove_class(&mut self, class_id: ClassId) {
        if let Some(p) = self.classes.iter().position(|c| c.id() == class_id) {
            self.classes.remove(p);
        }
    }

    pub fn get_class(&self, class_id: ClassId) -> Option<&Class> {
        self.classes.iter().find(|c| c.id() == class_id)
    }
}

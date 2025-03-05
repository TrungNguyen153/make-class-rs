use super::{Class, ClassId};

pub struct ClassList {
    classes: Vec<Class>,
    selected: Option<ClassId>,
}

impl Default for ClassList {
    fn default() -> Self {
        Self {
            classes: vec![Class::new(0, "Dummy")],
            selected: Some(0),
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

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn set_selected(&mut self, class_id: ClassId) {
        self.selected.replace(class_id);
    }

    pub fn un_select(&mut self) {
        self.selected.take();
    }

    pub fn add_class(&mut self, name: impl Into<String>) -> ClassId {
        let id = fastrand::usize(..);
        self.classes.push(Class::new(id, name));
        id
    }

    pub fn remove_class(&mut self, class_id: usize) {
        if let Some(p) = self.classes.iter().position(|c| c.id() == class_id) {
            self.classes.remove(p);
        }
    }
}

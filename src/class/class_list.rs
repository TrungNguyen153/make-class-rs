use super::{Class, ClassId};

pub struct ClassList {
    classes: Vec<Class>,
    selected: Option<ClassId>,
}

impl Default for ClassList {
    fn default() -> Self {
        Self {
            classes: vec![Class::new(0, "Dummy")],
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
}

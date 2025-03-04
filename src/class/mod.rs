use std::cell::Cell;

use crate::field::Field;

pub type ClassId = usize;

pub struct Class {
    id: ClassId,
    pub name: String,
    pub address: Cell<usize>,
    pub fields: Vec<Box<dyn Field>>,
}

impl Class {
    pub fn new(id: ClassId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            address: 0.into(),
            // TODO default showup some hex bytes
            fields: vec![],
        }
    }

    pub fn empty(id: ClassId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            address: 0.into(),
            fields: vec![],
        }
    }

    pub fn id(&self) -> ClassId {
        self.id
    }

    pub fn class_size(&self) -> usize {
        self.fields.iter().map(|f| f.field_size()).sum()
    }

    pub fn extend_fields(&mut self, fields: impl Into<Vec<Box<dyn Field>>>) {
        self.fields.extend(fields.into());
    }
}

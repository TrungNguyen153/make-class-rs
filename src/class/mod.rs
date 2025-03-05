pub mod class_list;

use std::cell::{Cell, RefCell};

use crate::{address::AddressString, field::Field};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct ClassId(usize);

impl From<usize> for ClassId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub struct Class {
    id: ClassId,
    pub name: String,
    pub address: RefCell<AddressString>,
    pub fields: Vec<Box<dyn Field>>,
}

impl Class {
    pub fn new(id: impl Into<ClassId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            address: RefCell::new(0.into()),
            // TODO default showup some hex bytes
            fields: vec![],
        }
    }

    pub fn empty(id: impl Into<ClassId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            address: RefCell::new(0.into()),
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

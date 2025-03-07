pub mod class_list;

use std::cell::RefCell;

use crate::{
    address::AddressString,
    field::{
        Field, FieldId,
        boolean::BoolField,
        float::FloatField,
        hex::HexField,
        int::IntField,
        string::{PointerTextField, TextField},
        vector::VectorField,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct ClassId(usize);

impl Default for ClassId {
    fn default() -> Self {
        Self(fastrand::usize(..))
    }
}

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

fn create_dummy_fields() -> Vec<Box<dyn Field>> {
    vec![
        HexField::<8>::default().boxed(),
        HexField::<16>::default().boxed(),
        HexField::<32>::default().boxed(),
        HexField::<64>::default().boxed(),
        FloatField::<4>::default().boxed(),
        BoolField::default().boxed(),
        IntField::<8>::signed_default().boxed(),
        IntField::<16>::signed_default().boxed(),
        IntField::<32>::signed_default().boxed(),
        IntField::<64>::signed_default().boxed(),
        IntField::<8>::unsigned_default().boxed(),
        IntField::<16>::unsigned_default().boxed(),
        IntField::<32>::unsigned_default().boxed(),
        IntField::<64>::unsigned_default().boxed(),
        VectorField::<2>::default().boxed(),
        VectorField::<3>::default().boxed(),
        VectorField::<4>::default().boxed(),
        TextField::<8>::default().boxed(),
        TextField::<16>::default().boxed(),
        PointerTextField::<8>::default().boxed(),
        PointerTextField::<16>::default().boxed(),
    ]
}

impl Class {
    pub fn new(id: impl Into<ClassId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            address: RefCell::new(0.into()),
            fields: create_dummy_fields(),
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

    pub fn add_field(&mut self, field: Box<dyn Field>) {
        self.fields.push(field);
    }

    pub fn field_pos(&self, field_id: FieldId) -> Option<usize> {
        self.fields.iter().position(|f| f.id() == field_id)
    }

    pub fn field_len(&self) -> usize {
        self.fields.len()
    }
}

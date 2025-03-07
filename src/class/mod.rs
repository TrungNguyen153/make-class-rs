pub mod class_list;

use std::cell::RefCell;

use crate::{
    address::AddressString,
    field::{
        Field, FieldId, allocate_padding,
        boolean::BoolField,
        float::FloatField,
        hex::HexField,
        int::IntField,
        string::{PointerTextField, TextField},
        vector::VectorField,
    },
    utils::offset_align_to,
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

    /// return number of field inserted
    pub fn insert_bytes(&mut self, byte_count: usize, at_field_id: FieldId) -> eyre::Result<usize> {
        let Some(field_pos) = self.field_pos(at_field_id) else {
            eyre::bail!("{}", obfstr!("[InsertBytes] Why field id not here here ??"))
        };

        let mut count = 0;

        let padding = allocate_padding(byte_count);
        for p in padding {
            count += 1;
            self.fields.insert(field_pos, p);
        }
        Ok(count)
    }

    /// return number of field added
    pub fn add_bytes(&mut self, byte_count: usize, at_field_id: FieldId) -> eyre::Result<usize> {
        let Some(field_pos) = self.field_pos(at_field_id) else {
            eyre::bail!("{}", obfstr!("[AddBytes] Why field id not here here ??"))
        };

        let padding = allocate_padding(byte_count);

        let field_len = self.field_len();

        let mut count = 0;
        if field_pos == field_len {
            // field at end
            count += padding.len();
            self.extend_fields(padding);
        } else {
            for p in padding {
                count += 1;
                self.fields.insert(field_pos + 1, p);
            }
        }
        Ok(count)
    }

    pub fn remove_field_by_id(&mut self, field_id: FieldId) -> eyre::Result<()> {
        if let Some(p) = self.fields.iter().position(|f| f.id() == field_id) {
            self.fields.remove(p);
            return Ok(());
        }
        eyre::bail!("{}", obfstr!("Field not found"))
    }

    pub fn merge_hex_field(&mut self, start_field_pos: usize) {
        let max_len = self.field_len();
        if start_field_pos >= max_len {
            // out of idx
            return;
        }

        let field = &self.fields[start_field_pos];
        let field_size = field.field_size();
        let field_id = field.id();

        if start_field_pos + 1 >= max_len {
            // no more to go
            return;
        }

        if field_size % 8 != 0 {
            let mut missing = offset_align_to(field_size, 8) - field_size;
            let mut stolen_bytes = 0;
            let mut merge_field_id = vec![];
            for idx in start_field_pos + 1..max_len {
                let next_field = &self.fields[idx];
                let next_field_id = next_field.id();
                if next_field.had_name() {
                    merge_field_id.clear();
                    stolen_bytes = 0;
                    break;
                }
                let next_field_size = next_field.field_size();

                // fit
                if missing == next_field_size {
                    merge_field_id.push(next_field_id);
                    stolen_bytes += next_field_size;
                    break;
                }

                // too enough
                if missing < next_field_size {
                    merge_field_id.push(next_field_id);
                    stolen_bytes += next_field_size;
                    break;
                }

                // grab this
                missing -= next_field_size;
                merge_field_id.push(next_field_id);
                stolen_bytes += next_field_size;
            }

            // ok we enought
            if stolen_bytes > 0 {
                stolen_bytes += field_size;
                // add byte then remove all old
                self.add_bytes(stolen_bytes, field_id).unwrap();
                self.remove_field_by_id(field_id).unwrap();
                for fid in merge_field_id {
                    self.remove_field_by_id(fid).unwrap();
                }
                return;
            }
        }

        if field_size % 4 != 0 {
            let mut missing = offset_align_to(field_size, 4) - field_size;
            let mut stolen_bytes = 0;
            let mut merge_field_id = vec![];
            for idx in start_field_pos + 1..max_len {
                let next_field = &self.fields[idx];
                let next_field_id = next_field.id();
                if next_field.had_name() {
                    merge_field_id.clear();
                    stolen_bytes = 0;
                    break;
                }
                let next_field_size = next_field.field_size();

                // fit
                if missing == next_field_size {
                    merge_field_id.push(next_field_id);
                    stolen_bytes += next_field_size;
                    break;
                }

                // too enough
                if missing < next_field_size {
                    merge_field_id.push(next_field_id);
                    stolen_bytes += next_field_size;
                    break;
                }

                // grab this
                missing -= next_field_size;
                merge_field_id.push(next_field_id);
                stolen_bytes += next_field_size;
            }

            // ok we enought
            if stolen_bytes > 0 {
                stolen_bytes += field_size;
                // add byte then remove all old
                self.add_bytes(stolen_bytes, field_id).unwrap();
                self.remove_field_by_id(field_id).unwrap();
                for fid in merge_field_id {
                    self.remove_field_by_id(fid).unwrap();
                }
            }
        }
    }
}

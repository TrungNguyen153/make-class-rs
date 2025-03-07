pub mod rust;

use crate::field::field_tag::FieldTag;

pub trait Generator {
    fn begin_class(&mut self, name: &str);
    fn end_class(&mut self);

    fn add_field(&mut self, name: &str, tag: FieldTag, field_size: usize, metadata: &str);
    fn add_offset(&mut self, offset: usize);

    fn finilize(&mut self) -> String;
}

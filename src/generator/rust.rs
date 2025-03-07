use crate::field::field_tag::FieldTag;

use super::Generator;

pub struct RustGenerator {
    text: String,
    last_offset: usize,
    offset: usize,
    stack_last_add_offset: usize,
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self {
            text: "// Generated by MakeClass 1.0\n\n".to_string(),
            last_offset: 0,
            offset: 0,
            stack_last_add_offset: 0,
        }
    }
}

impl Generator for RustGenerator {
    fn begin_class(&mut self, name: &str) {
        self.text += &format!("#[repr(C)]\npub struct {name} {{\n");
    }

    fn end_class(&mut self) {
        if self.stack_last_add_offset != 0 {
            self.text += &format!(
                "    _pad_at_{:#X}: [u8; 0x{:x}], // {:#X}\n",
                self.offset - self.stack_last_add_offset,
                self.stack_last_add_offset,
                self.offset - self.stack_last_add_offset
            );
            self.stack_last_add_offset = 0;
        }
        self.text += "}\n\n";
        self.offset = 0;
        self.last_offset = 0;
    }

    fn add_field(&mut self, name: &str, tag: FieldTag, size: usize, metadata: &str) {
        if self.offset != self.last_offset {
            self.text += &format!(
                "    _pad_at_{:#X}: [u8; 0x{:x}], // {:#X}\n",
                self.offset,
                self.offset - self.last_offset,
                self.offset
            );
        }

        self.text += &format!(
            "    pub {name}: {}, // {:#X}\n",
            tag_to_type(tag, metadata),
            self.offset
        );

        self.offset += size;
        self.last_offset = self.offset;
        self.stack_last_add_offset = 0;
    }

    fn add_offset(&mut self, offset: usize) {
        self.offset += offset;
        self.stack_last_add_offset += offset;
    }

    fn finilize(&mut self) -> String {
        std::mem::take(&mut self.text)
    }
}

fn tag_to_type(tag: FieldTag, metadata: &str) -> String {
    match tag {
        FieldTag::Bool => "bool".to_owned(),
        FieldTag::ClassInstance => metadata.to_owned(),
        FieldTag::ClassPointer => format!("&mut {metadata}"),
        FieldTag::Float32 => "f32".to_owned(),
        FieldTag::Float64 => "f64".to_owned(),
        FieldTag::I8 => "i8".to_owned(),
        FieldTag::I16 => "i16".to_owned(),
        FieldTag::I32 => "i32".to_owned(),
        FieldTag::I64 => "i64".to_owned(),
        FieldTag::U8 => "u8".to_owned(),
        FieldTag::U16 => "u16".to_owned(),
        FieldTag::U32 => "u32".to_owned(),
        FieldTag::U64 => "u64".to_owned(),
        FieldTag::Hex8 => "[u8; 1]".to_owned(),
        FieldTag::Hex16 => "[u8; 2]".to_owned(),
        FieldTag::Hex32 => "[u8; 4]".to_owned(),
        FieldTag::Hex64 => "[u8; 8]".to_owned(),
        FieldTag::Utf8 => format!("[u8; {metadata}]"),
        FieldTag::Utf16 => format!("[u16; {metadata}]"),
        FieldTag::PtrUtf8 => format!("&mut [u8; {metadata}]"),
        FieldTag::PtrUtf16 => format!("&mut [u16; {metadata}]"),
        FieldTag::Vec2 => "Vec2".to_owned(),
        FieldTag::Vec3 => "Vec3".to_owned(),
        FieldTag::Vec4 => "Vec4".to_owned(),
    }
}

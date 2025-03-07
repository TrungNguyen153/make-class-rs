use crate::{
    class::{Class, class_list::ClassList},
    field::{
        Field, allocate_padding,
        boolean::BoolField,
        class_instance::ClassInstanceField,
        class_pointer::ClassPointerField,
        field_tag::FieldTag,
        float::FloatField,
        hex::HexField,
        int::IntField,
        string::{PointerTextField, TextField},
        vector::VectorField,
    },
    generator::Generator,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct FieldData {
    name: String,
    offset: usize,
    field_size: usize,
    tag: FieldTag,
    metadata: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ClassData {
    name: String,
    fields: Vec<FieldData>,
}

#[derive(Default)]
struct ProjectDataGenerator {
    classes: Vec<ClassData>,
    offset: usize,
}

impl Generator for ProjectDataGenerator {
    fn begin_class(&mut self, name: &str) {
        self.classes.push(ClassData {
            name: name.to_owned(),
            fields: vec![],
        });
    }

    fn end_class(&mut self) {
        self.offset = 0;
    }

    fn add_field(&mut self, name: &str, tag: FieldTag, field_size: usize, metadata: &str) {
        self.classes.last_mut().unwrap().fields.push(FieldData {
            name: name.to_owned(),
            offset: self.offset,
            tag,
            metadata: metadata.to_owned(),
            field_size,
        });

        self.offset += field_size;
    }

    fn add_offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn finilize(&mut self) -> String {
        todo!()
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct ProjectData {
    classes: Vec<ClassData>,
}

impl ProjectData {
    pub fn store(classes: &[Class]) -> Self {
        let mut datagen = ProjectDataGenerator::default();
        let dynam = &mut datagen as &mut dyn Generator;

        for class in classes {
            dynam.begin_class(&class.name);
            for f in class.fields.iter() {
                f.codegen(dynam);
            }
            dynam.end_class();
        }

        Self {
            classes: datagen.classes,
        }
    }

    pub fn to_class_list(self) -> ClassList {
        let mut list = ClassList::EMPTY;

        self.classes
            .iter()
            .for_each(|cl| _ = list.add_class(cl.name.to_string()));
        self.classes.into_iter().for_each(|mut dataclass| {
            dataclass.fields.sort_by_key(|f| f.offset);

            let cid = list.get_class_by_name(&dataclass.name).unwrap().id();
            let mut current_offset = 0;

            for FieldData {
                name,
                offset: field_offset,
                tag,
                metadata,
                field_size,
            } in dataclass.fields
            {
                let class = list.get_class_mut(cid).unwrap();
                if field_offset > current_offset {
                    class
                        .fields
                        .extend(allocate_padding(field_offset - current_offset));
                }

                match tag {
                    FieldTag::Bool => BoolField::new(name).boxed(),
                    FieldTag::ClassInstance => {
                        let c = ClassInstanceField::new_with_class_id(
                            list.get_class_by_name(metadata).unwrap().id(),
                        );
                        c.set_name(name);
                        c.boxed()
                    }
                    FieldTag::ClassPointer => {
                        let c = ClassPointerField::new_with_class_id(
                            list.get_class_by_name(metadata).unwrap().id(),
                        );
                        c.set_name(name);
                        c.boxed()
                    }
                    FieldTag::Float32 => FloatField::<32>::new(name).boxed(),
                    FieldTag::Float64 => FloatField::<64>::new(name).boxed(),
                    FieldTag::I8 => IntField::<8>::signed(name).boxed(),
                    FieldTag::I16 => IntField::<16>::signed(name).boxed(),
                    FieldTag::I32 => IntField::<32>::signed(name).boxed(),
                    FieldTag::I64 => IntField::<64>::signed(name).boxed(),
                    FieldTag::U8 => IntField::<8>::unsigned(name).boxed(),
                    FieldTag::U16 => IntField::<16>::unsigned(name).boxed(),
                    FieldTag::U32 => IntField::<32>::unsigned(name).boxed(),
                    FieldTag::U64 => IntField::<64>::unsigned(name).boxed(),
                    FieldTag::Hex8 => HexField::<8>::new().boxed(),
                    FieldTag::Hex16 => HexField::<16>::new().boxed(),
                    FieldTag::Hex32 => HexField::<32>::new().boxed(),
                    FieldTag::Hex64 => HexField::<64>::new().boxed(),
                    FieldTag::Utf8 => {
                        let f = TextField::<8>::new(name);
                        f.change_char_count(metadata.parse().unwrap());
                        f.boxed()
                    }
                    FieldTag::Utf16 => {
                        let f = TextField::<16>::new(name);
                        f.change_char_count(metadata.parse().unwrap());
                        f.boxed()
                    }
                    FieldTag::PtrUtf8 => {
                        let f = PointerTextField::<8>::new(name);
                        f.change_character_count(metadata.parse().unwrap());
                        f.boxed()
                    }
                    FieldTag::PtrUtf16 => {
                        let f = PointerTextField::<16>::new(name);
                        f.change_character_count(metadata.parse().unwrap());
                        f.boxed()
                    }
                    FieldTag::Vec2 => VectorField::<2>::new(name).boxed(),
                    FieldTag::Vec3 => VectorField::<3>::new(name).boxed(),
                    FieldTag::Vec4 => VectorField::<4>::new(name).boxed(),
                };

                current_offset += field_offset + field_size;
            }
        });

        list
    }

    pub fn load() -> Self {
        let Ok(data) = std::fs::read_to_string("./data.ron") else {
            return Self::default();
        };

        let Ok(s) = ron::from_str(&data) else {
            return Self::default();
        };
        s
    }

    pub fn save(&self) {
        let data = ron::to_string(self).unwrap();
        std::fs::write("./data.ron", data).unwrap();
    }
}

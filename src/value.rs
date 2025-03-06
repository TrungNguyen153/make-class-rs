#[derive(Debug, Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    String(String),
    HexAddress(usize),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::U8(v) => write!(f, "{v}"),
            Value::U16(v) => write!(f, "{v}"),
            Value::U32(v) => write!(f, "{v}"),
            Value::U64(v) => write!(f, "{v}"),
            Value::I8(v) => write!(f, "{v}"),
            Value::I16(v) => write!(f, "{v}"),
            Value::I32(v) => write!(f, "{v}"),
            Value::I64(v) => write!(f, "{v}"),
            Value::F32(v) => write!(f, "{v}"),
            Value::F64(v) => write!(f, "{v}"),
            Value::Bool(v) => write!(f, "{}", if *v { "true" } else { "false" }),
            Value::Vec2(x, y) => write!(f, "({x}, {y})"),
            Value::Vec3(x, y, z) => write!(f, "({x}, {y}, {z})"),
            Value::Vec4(x, y, z, w) => write!(f, "({x}, {y}, {z}, {w})"),
            Value::String(v) => write!(f, "\"{v}\""),
            Value::HexAddress(v) => write!(f, "{v:#X}"),
        }
    }
}

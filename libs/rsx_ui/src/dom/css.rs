#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    HexColor(String),
    ArrayValue(Vec<Value>),
    FunCall(FunCallValue),
    StringLiteral(String),
    UnicodeCodepoint(i32),
    UnicodeRange(i32, i32),
    Number(f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Px,
    Em,
    Per,
    Rem,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn from_hex(str: &str) -> Self {
        let n = i32::from_str_radix(&str[1..], 16).unwrap();
        let r = (n >> 16) & 0xFF;
        let g = (n >> 8) & 0xFF;
        let b = (n/*>>0*/) & 0xFF;
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: 255,
        }
    }
    pub fn to_array(&self) -> [f32; 4] {
        [
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        ]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunCallValue {
    pub(crate) name: String,
    pub(crate) arguments: Vec<Value>,
}

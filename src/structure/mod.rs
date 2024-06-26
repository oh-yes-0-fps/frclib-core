//! This module contains an implementation of [WPIlib struct spec](https://github.com/wpilibsuite/allwpilib/blob/main/wpiutil/doc/struct.adoc)
//! for rust and a macro to generate the trait implementation for a given struct.

#[cfg(test)]
mod test;

mod prims;

// use logos::Logos;

use std::io::Cursor;

pub use inventory;

/// A description of a structure, used for serialization and deserialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrcStructDesc {
    /// A function that returns the schema of the structure,
    /// this is needed because the schema cannot be made in a const context
    pub schema_supplier: fn() -> String,
    /// The type of the structure, typically the name of the rust type
    pub type_str: &'static str,
    /// The size of the structure in bytes
    pub size: usize,
}

inventory::collect!(FrcStructDesc);

/// A global database of structure descriptions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrcStructDescDB;

impl FrcStructDescDB {
    /// Adds a structure description to the global database,
    /// this is a runtime equivalent of the [`inventory::submit!`] macro.
    #[cold]
    pub fn add(desc: FrcStructDesc) {
        if Self::contains_type(desc.type_str) {
            return;
        }
        let static_desc_ref = Box::leak(Box::new(desc));
        let node = inventory::Node {
            value: static_desc_ref,
            next: ::inventory::core::cell::UnsafeCell::new(::inventory::core::option::Option::None),
        };
        unsafe { inventory::ErasedNode::submit(node.value, Box::leak(Box::new(node))) }
    }

    /// Adds a structure description to the global database,
    /// this is a runtime equivalent of the [`inventory::submit!`] macro.
    #[cold]
    pub fn add_ref(desc: &'static FrcStructDesc) {
        if Self::contains_type(desc.type_str) {
            return;
        }
        let node = inventory::Node {
            value: desc,
            next: ::inventory::core::cell::UnsafeCell::new(::inventory::core::option::Option::None),
        };
        unsafe { inventory::ErasedNode::submit(node.value, Box::leak(Box::new(node))) }
    }

    /// Checks if the global database contains a structure description for a given type
    #[must_use]
    pub fn contains_type(type_str: &str) -> bool {
        inventory::iter::<FrcStructDesc>
            .into_iter()
            .any(|desc| desc.type_str == type_str)
    }

    /// Gets a structure description from the global database for a given type,
    /// returns None if the type is not found
    #[must_use]
    pub fn get(type_str: &str) -> Option<&'static FrcStructDesc> {
        inventory::iter::<FrcStructDesc>
            .into_iter()
            .find(|desc| desc.type_str == type_str)
    }
}

pub use frclib_structure_macros::FrcStructure;

/// A trait that allows serialization and deserialization of arbitrary structures
/// to and from a [``FrcValue``](crate::value::FrcValue)
pub trait FrcStructure
where
    Self: Sized + Copy,
{
    /// The schema of the structure,
    /// this is a string that describes the format of the structure
    const SCHEMA_SUPPLIER: fn() -> String;
    /// The type of the structure as a string,
    /// this is typically the name of the rust type
    const TYPE: &'static str;
    /// The size of the structure in bytes
    const SIZE: usize;
    /// An [``FrcStructDesc``](crate::structure::FrcStructDesc) that describes the structure,
    /// it's composed of [`SIZE`](crate::structure::FrcStructure::SIZE),
    /// [`TYPE`](crate::structure::FrcStructure::TYPE),
    /// and [`SCHEMA_SUPPLIER`](crate::structure::FrcStructure::SCHEMA_SUPPLIER)
    const DESCRIPTION: FrcStructDesc = FrcStructDesc {
        schema_supplier: Self::SCHEMA_SUPPLIER,
        type_str: Self::TYPE,
        size: Self::SIZE,
    };

    /// Packs the structure into a buffer
    fn pack(&self, buffer: &mut Vec<u8>);

    /// Unpacks the structure from a buffer
    fn unpack(buffer: &mut Cursor<&[u8]>) -> Self;

    #[must_use]
    #[doc(hidden)]
    fn format_field(field: &str) -> String {
        format!("{} {}", Self::TYPE, field)
    }
}

/// A way of defining any number of same typed [``FrcStructure``]s
/// in a single binary heap.
///
/// The type information and struct count is also coupled with the binary data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrcStructureBytes {
    /// The description of the structure types and layout
    pub desc: &'static FrcStructDesc,
    /// The number of structs packed into `data`
    pub count: usize,
    /// The binary data of the structs
    pub data: Box<[u8]>,
}
impl FrcStructureBytes {
    /// Creates a new [``FrcStructureBytes``] from a description, count, and data
    #[must_use]
    pub fn from_parts(desc: &'static FrcStructDesc, count: usize, data: Box<[u8]>) -> Self {
        Self { desc, count, data }
    }
}

/// A set length string of characters
pub type StructString<const N: usize> = [char; N];

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub(crate) enum StructureFieldTypes {
//     Bool(usize),
//     Char(usize),
//     Int8(usize),
//     Int16(usize),
//     Int32(usize),
//     Int64(usize),
//     UInt8(usize),
//     UInt16(usize),
//     UInt32(usize),
//     UInt64(usize),
//     Float32(usize),
//     Float64(usize),
// }

// impl StructureFieldTypes {
//     #[allow(clippy::match_same_arms)]
//     const fn base_size(&self) -> usize {
//         match self {
//             Self::Bool(_) => 1,
//             Self::Char(_) => 1,
//             Self::Int8(_) => 1,
//             Self::Int16(_) => 2,
//             Self::Int32(_) => 4,
//             Self::Int64(_) => 8,
//             Self::UInt8(_) => 1,
//             Self::UInt16(_) => 2,
//             Self::UInt32(_) => 4,
//             Self::UInt64(_) => 8,
//             Self::Float32(_) => 4,
//             Self::Float64(_) => 8,
//         }
//     }

//     const fn count(&self) -> usize {
//         match self {
//             Self::Bool(c)
//             | Self::Char(c)
//             | Self::Int8(c)
//             | Self::Int16(c)
//             | Self::Int32(c)
//             | Self::Int64(c)
//             | Self::UInt8(c)
//             | Self::UInt16(c)
//             | Self::UInt32(c)
//             | Self::UInt64(c)
//             | Self::Float32(c)
//             | Self::Float64(c) => *c,
//         }
//     }

//     const fn size(&self) -> usize {
//         self.base_size() * self.count()
//     }

//     fn from_type(type_name: &str, count: usize) -> Option<Self> {
//         match type_name {
//             "bool" => Some(Self::Bool(count)),
//             "char" => Some(Self::Char(count)),
//             "int8" => Some(Self::Int8(count)),
//             "int16" => Some(Self::Int16(count)),
//             "int32" => Some(Self::Int32(count)),
//             "int64" => Some(Self::Int64(count)),
//             "uint8" => Some(Self::UInt8(count)),
//             "uint16" => Some(Self::UInt16(count)),
//             "uint32" => Some(Self::UInt32(count)),
//             "uint64" => Some(Self::UInt64(count)),
//             "float" | "float32" => Some(Self::Float32(count)),
//             "double" | "float64" => Some(Self::Float64(count)),
//             _ => None,
//         }
//     }
// }

// #[derive(Default, Debug, Clone, PartialEq)]
// pub(crate) enum LexingError {
//     ParseNumberError,
//     EnumVariantError,
//     #[default]
//     Other,
// }
// impl From<std::num::ParseIntError> for LexingError {
//     fn from(_: std::num::ParseIntError) -> Self {
//         Self::ParseNumberError
//     }
// }

// #[derive(logos::Logos, Debug, PartialEq, Eq, PartialOrd, Ord)]
// #[logos(error = LexingError)]
// #[logos(skip r"[ \t\n\f]+")]
// pub(crate) enum Token<'a> {
//     #[regex(
//         r"bool|char|int8|int16|int32|int64|uint8|uint16|uint32|uint64|float32|float64|float|double",
//         |lex| lex.slice(), priority = 3)]
//     TypeName(&'a str),

//     #[token("enum")]
//     EnumKeyword,

//     #[regex(
//         r"[-a-zA-Z_][a-zA-Z0-9_-]*=-?[0-9]+",
//         |lex| {
//             let split = lex.slice().split('=').collect::<Vec<_>>();
//             Ok::<_, LexingError>((
//                 *split.first().ok_or(LexingError::EnumVariantError)?,
//                 split.get(1).ok_or(LexingError::EnumVariantError)?.parse::<i8>()?
//             ))
//         }, priority = 3)]
//     EnumVariant((&'a str, i8)),

//     #[regex(r"[0-9]+", |lex| lex.slice().parse(), priority = 2)]
//     Integer(u32),

//     #[regex(r"[-a-zA-Z_][a-zA-Z0-9_-]*", |lex| lex.slice())]
//     Ident(&'a str),

//     #[token("{")]
//     OpenBrace,
//     #[token("}")]
//     CloseBrace,
//     #[token("[")]
//     OpenBracket,
//     #[token("]")]
//     CloseBracket,
//     #[token(",")]
//     Comma,
//     #[token(";")]
//     Semicolon,
//     #[token(":")]
//     Colon,
// }

// #[allow(dead_code)]
// pub(crate) fn parse_schema_toplevel(
//     schema: &str,
// ) -> Vec<(String, usize, StructureFieldTypes)> {
//     parse_schema(schema, "", 0)
// }

// #[allow(dead_code)]
// pub(crate) fn parse_schema(
//     schema: &str,
//     prefix: &str,
//     offset: usize,
// ) -> Vec<(String, usize, StructureFieldTypes)> {
//     let lexer = Token::lexer(schema);
//     let tokens_collect: Vec<_> = lexer.collect();
//     for tok in &tokens_collect {
//         if tok.is_err() {
//             return vec![];
//         }
//     }
//     let tokens = tokens_collect.into_iter();
//     let mut cursor = offset;
//     tokens
//         .map(|token| token.expect("Lexing Token Slipped Past"))
//         .filter(|token| {
//             matches!(
//                 token,
//                 Token::Ident(_) | Token::Integer(_) | Token::TypeName(_) | Token::Semicolon
//             )
//         })
//         .collect::<Vec<_>>()
//         .split(|token| token == &Token::Semicolon)
//         .filter_map(|field_tokens| {
//             if field_tokens.len() < 2 || field_tokens.len() > 3 {
//                 return None;
//             }

//             let Token::Ident(ident) = field_tokens[1] else {
//                 return None;
//             };

//             match field_tokens[0] {
//                 Token::Ident(sub_struct) => {
//                     if let Some(desc) = FrcStructDescDB::get(sub_struct) {
//                         let ret = parse_schema(
//                             &(desc.schema_supplier)(),
//                             format!("{ident}.").as_str(),
//                             cursor,
//                         );
//                         cursor += desc.size;
//                         return Some(ret);
//                     }
//                 }
//                 Token::TypeName(type_name) => {
//                     let count = match field_tokens.get(2) {
//                         Some(Token::Integer(int)) => *int as usize,
//                         _ => 1,
//                     };
//                     if let Some(stype) = StructureFieldTypes::from_type(type_name, count) {
//                         let ret = vec![(format!("{prefix}{ident}"), cursor, stype)];
//                         cursor += stype.size();
//                         return Some(ret);
//                     }
//                 }
//                 _ => {}
//             }
//             None::<Vec<(String, usize, StructureFieldTypes)>>
//         })
//         .flatten()
//         .collect()
// }

// pub struct DynamicStructure {
//     desc: &'static FrcStructDesc,
//     buffer: BytesMut,
//     _map: HashMap<String, (usize, StructureFieldTypes), fxhash::FxBuildHasher>,
// }

// impl DynamicStructure {
//     pub fn try_new(desc: &'static FrcStructDesc, buffer: BytesMut) -> Result<Self, String> {
//         if buffer.len() != desc.size {
//             return Err(format!(
//                 "Buffer size ({}) does not match structure size ({})",
//                 buffer.len(),
//                 desc.size
//             ));
//         }
//         let mut map = HashMap::with_hasher(fxhash::FxBuildHasher::default());
//         for field in parse_schema_toplevel(desc.schema) {
//             map.insert(field.0, (field.1, field.2));
//         }
//         Ok(DynamicStructure {
//             desc,
//             buffer,
//             _map: map,
//         })
//     }

//     pub fn description(&self) -> &'static FrcStructDesc {
//         self.desc
//     }

//     pub fn update(&mut self, new: Box<Bytes>) {
//         debug_assert!(new.len() == self.buffer.len());
//         self.buffer[..].copy_from_slice(&new[..]);
//     }
// }

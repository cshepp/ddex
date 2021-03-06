#![allow(dead_code, unused_variables)]

use std::fmt;
use std::fmt::Display;
use crate::instructions::*;

pub type StringIndex = usize;
pub type TypeIndex   = usize;
pub type ProtoIndex  = usize;
pub type FieldIndex  = usize;
pub type MethodIndex = usize;
pub type ClassIndex  = usize;

#[derive(Debug)]
pub struct DexHeader {
    pub dex_version: String,
    pub checksum: u32,
    pub sha1: String,
    pub file_size: u32,
    pub header_size: u32,
    pub endianness: Endianness,
    pub link_size: u32,
    pub link_offset: u32,
    pub map_offset: u32,
    pub string_ids_size: u32,
    pub string_ids_offset: u32,
    pub type_ids_size: u32,
    pub type_ids_offset: u32,
    pub proto_ids_size: u32,
    pub proto_ids_offset: u32,
    pub field_ids_size: u32,
    pub field_ids_offset: u32,
    pub method_ids_size: u32,
    pub method_ids_offset: u32,
    pub class_defs_size: u32,
    pub class_defs_offset: u32,
    pub data_size: u32,
    pub data_offset: u32,
}

#[derive(Debug)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone)]
pub struct DexType {
    pub raw: String,
    pub parsed: TypeDescriptor,
}

#[derive(Debug, Clone)]
pub enum TypeDescriptor {
    Void,
    Boolean,
    Byte,
    Short,
    Char,
    Int,
    Long,
    Float,
    Double,
    Class(String),
    Array(Box<TypeDescriptor>),
}

#[derive(Debug)]
pub struct DexProto {
    pub shorty_idx: StringIndex,
    pub return_type_idx: TypeIndex,
    pub parameters_offset: u32,
    pub parameter_type_idx_list: Vec<TypeIndex>,
}

#[derive(Debug, Clone)]
pub struct DexField {
    pub class_idx: ClassIndex,
    pub type_idx: TypeIndex,
    pub name_idx: StringIndex,
}

#[derive(Debug)]
pub struct DexMethod {
    pub class_idx: TypeIndex,
    pub proto_idx: ProtoIndex,
    pub name_idx: StringIndex,
}

#[derive(Debug)]
pub struct DexClassDef {
    pub class_idx: TypeIndex,
    pub access_flags: u32,
    pub superclass_idx: TypeIndex,
    pub interfaces_offset: u32,
    pub source_file_idx: StringIndex,
    pub annotations_offset: u32,
    pub class_data_offset: u32,
    pub static_values_offset: u32,
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

#[derive(Debug)]
pub struct EncodedField {
    pub field_idx: FieldIndex,
    pub access_flags: u32,
}

#[derive(Debug)]
pub struct EncodedMethod {
    pub method_idx: MethodIndex,
    pub access_flags: u32,
    pub code_offset: u32,
    pub code_item: Option<CodeItem>,
}

#[derive(Debug, Clone)]
pub struct CodeItem {
    pub addr: u32,
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_offset: u32,
    pub instructions_size: u32,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ClassAccessLevel {
    Public     = 0x1,
    Private    = 0x2,
    Protected  = 0x4,
    Static     = 0x8,
    Final      = 0x10,
    Interface  = 0x200,
    Abstract   = 0x400,
    Synthetic  = 0x1000,
    Annotation = 0x2000,
    Enum       = 0x4000,
}

#[derive(Debug)]
pub enum FieldAccessLevel {
    Public    = 0x1,
    Private   = 0x2,
    Protected = 0x4,
    Static    = 0x8,
    Final     = 0x10,
    Volatile  = 0x40,
    Transient = 0x80,
    Synthetic = 0x1000,
    Enum      = 0x4000,
}

#[derive(Debug)]
pub enum MethodAccessLevel {
    Public               = 0x1,
    Private              = 0x2,
    Protected            = 0x4,
    Static               = 0x8,
    Final                = 0x10,
    Synchronized         = 0x20,
    Bridge               = 0x40,
    VarArgs              = 0x80,
    Native               = 0x100,
    Abstract             = 0x400,
    Strict               = 0x800,
    Synthetic            = 0x1000,
    Constructor          = 0x10000,
    DeclaredSynchronized = 0x20000,
}

impl Display for Endianness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Endianness::LittleEndian => write!(f, "little endian"),
            Endianness::BigEndian => write!(f, "big endian"),
        }
    }
}

impl Display for DexHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
r#"dex version        {}
checksum           {}
sha1               {}
file size          {} bytes
header size        {} bytes
endianness         {}
link size          {} bytes
link offset        {:#x}
map offset         {:#x}
string IDs size    {} bytes
string IDs offset  {:#x}
type IDs size      {} bytes
type IDs offset    {:#x}
proto IDs size     {} bytes
proto IDs offset   {:#x}
field IDs size     {} bytes
field IDs offset   {:#x}
method IDs size    {} bytes
method IDs offset  {:#x}
class defs size    {} bytes
class defs offset  {:#x}
data size          {} bytes
data offset        {:#x}"#, 
        self.dex_version, 
        self.checksum, 
        self.sha1, 
        self.file_size,
        self.header_size,
        self.endianness,
        self.link_size,
        self.link_offset,
        self.map_offset,
        self.string_ids_size,
        self.string_ids_offset,
        self.type_ids_size,
        self.type_ids_offset,
        self.proto_ids_size,
        self.proto_ids_offset,
        self.field_ids_size,
        self.field_ids_offset,
        self.method_ids_size,
        self.method_ids_offset,
        self.class_defs_size,
        self.class_defs_offset,
        self.data_size,
        self.data_offset
        )
    }
}

impl Display for TypeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeDescriptor::Void => write!(f, "void"),
            TypeDescriptor::Boolean => write!(f, "boolean"),
            TypeDescriptor::Byte => write!(f, "byte"),
            TypeDescriptor::Short => write!(f, "short"),
            TypeDescriptor::Char => write!(f, "char"),
            TypeDescriptor::Int => write!(f, "int"),
            TypeDescriptor::Long => write!(f, "long"),
            TypeDescriptor::Float => write!(f, "float"),
            TypeDescriptor::Double => write!(f, "double"),
            TypeDescriptor::Class(x) => write!(f, "{}", x.to_string().replace("/", ".")),
            TypeDescriptor::Array(b) => {
                write!(f, "{}[]", b)
            },
        }
    }
}
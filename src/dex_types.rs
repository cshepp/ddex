#![allow(dead_code, unused_variables)]

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

#[derive(Debug)]
pub struct DexType {
    pub raw: String,
    pub parsed: TypeDescriptor,
}

#[derive(Debug)]
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
    pub shorty_idx: u32,
    pub return_type_idx: u32,
    pub parameters_offset: u32,
    pub parameter_type_idx_list: Vec<u16>,
}

#[derive(Debug)]
pub struct DexField {
    pub class_idx: u32,
    pub type_idx: u32,
    pub name_idx: u32,
}

#[derive(Debug)]
pub struct DexMethod {
    pub class_idx: u32, // index into Types??? dunno? Maybe ClassDefs?
    pub proto_idx: u32, // index into Protos
    pub name_idx: u32,  // index into Strings
}

#[derive(Debug)]
pub struct DexClassDef {
    pub class_idx: u32, // index into type_ids
    pub access_flags: u32,
    pub superclass_idx: u32, // index into type_ids
    pub interfaces_offset: u32,
    pub source_file_idx: u32, // index into string_ids
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
    pub field_idx: u32,
    pub access_flags: u32,
}

#[derive(Debug)]
pub struct EncodedMethod {
    pub method_idx: u32,
    pub access_flags: u32,
    pub code_offset: u32,
}

#[derive(Debug)]
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
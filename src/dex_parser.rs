
use crate::binary_parser::BinaryParser;
use crate::util::{to_decimal, to_ascii, print_ascii, print_hex};

pub struct DexParser {
    parser: BinaryParser,
    pub header: Option<DexHeader>,
    pub strings: Vec<DexString>,
    pub types: Vec<DexType>,
    pub class_defs: Vec<DexClassDef>,
}

impl DexParser {
    pub fn new(buffer: Vec<u8>) -> DexParser {
        DexParser {
            parser: BinaryParser::new(buffer),
            header: None,
            strings: Vec::new(),
            types: Vec::new(),
            class_defs: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_header();
        self.parse_strings();
        self.parse_types();
        self.parse_class_defs();
    }
}

impl DexParser {
    fn parse_header(&mut self) {
        self.parser.expect_many(vec![0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x38, 0x00]);
        let checksum = self.parser.take(4);
        let sha1 = self.parser.take(20);
        let file_size = self.parser.take(4);
        let header_size = self.parser.take(4);
        let endian_constant = self.parser.take(4);
        let link_size = self.parser.take(4);
        let link_offset = self.parser.take(4);
        let map_offset = self.parser.take(4);
        let string_ids_size = self.parser.take(4);
        let string_ids_offset = self.parser.take(4);
        let type_ids_size = self.parser.take(4);
        let type_ids_offset = self.parser.take(4);
        let proto_ids_size = self.parser.take(4);
        let proto_ids_offset = self.parser.take(4);
        let field_ids_size = self.parser.take(4);
        let field_ids_offset = self.parser.take(4);
        let method_ids_size = self.parser.take(4);
        let method_ids_offset = self.parser.take(4);
        let class_defs_size = self.parser.take(4);
        let class_defs_offset = self.parser.take(4);
        let data_size = self.parser.take(4);
        let data_offset = self.parser.take(4);

        let header = DexHeader {
            dex_version: 0,
            checksum: to_decimal(&checksum),
            sha1: to_ascii(&sha1),
            file_size: to_decimal(&file_size),
            header_size: to_decimal(&header_size),
            endianness: Endianness::LittleEndian,
            link_size: to_decimal(&link_size),
            link_offset: to_decimal(&link_offset),
            map_offset: to_decimal(&map_offset),
            string_ids_size: to_decimal(&string_ids_size),
            string_ids_offset: to_decimal(&string_ids_offset),
            type_ids_size: to_decimal(&type_ids_size),
            type_ids_offset: to_decimal(&type_ids_offset),
            proto_ids_size: to_decimal(&proto_ids_size),
            proto_ids_offset: to_decimal(&proto_ids_offset),
            field_ids_size: to_decimal(&field_ids_size),
            field_ids_offset: to_decimal(&field_ids_offset),
            method_ids_size: to_decimal(&method_ids_size),
            method_ids_offset: to_decimal(&method_ids_offset),
            class_defs_size: to_decimal(&class_defs_size),
            class_defs_offset: to_decimal(&class_defs_offset),
            data_size: to_decimal(&data_size),
            data_offset: to_decimal(&data_offset),
        };

        self.header = Some(header);
    }

    fn parse_strings(&mut self) {
        let mut result: Vec<DexString> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            None => return,
        };
        let offset = header.string_ids_offset as usize;
        let size = (header.string_ids_size * 4) as usize;

        self.parser.seek_to(offset);
        let mut strings = self.parser.take(size);
        let mut string_parser = BinaryParser::new(strings);

        loop {
            if string_parser.is_it_the_end() {
                break;
            }

            let start_hex = &string_parser.take(4);
            let start_addr = to_decimal(start_hex) as usize;

            self.parser.seek_to(start_addr as usize);
            self.parser.drop(1); // not quite correct... should instead read the size of the UTF16 string (should be a u32 in uleb128 format)
            let mut s = self.parser.take_until(0x00);
            self.parser.expect(0x00);

            result.push(DexString{ address: start_addr, value: to_ascii(&s) });
        }

        self.strings = result;
    }

    fn parse_types(&mut self) {
        let mut result: Vec<DexType> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            None => return,
        };
        let offset = header.type_ids_offset as usize;
        let size = (header.type_ids_size * 4) as usize;
        
        self.parser.seek_to(offset);

        loop {
            let addr = self.parser.current_location();

            if addr >= offset + size {
                break;
            }

            let idx = to_decimal(&self.parser.take(4));
            result.push(DexType{ address: addr, descriptor_index: idx as usize });
        }

        self.types = result;
    }


    fn parse_class_defs(&mut self) {
        let mut result: Vec<DexClassDef> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            None => return,
        };
        let offset = header.class_defs_offset as usize;
        let size = (header.class_defs_size * 4 * 8) as usize;

        self.parser.seek_to(offset);

        loop {
            let addr = self.parser.current_location();

            if addr >= offset + size {
                break;
            }

            let class_idx = to_decimal(&self.parser.take(4));
            let access_flags = to_decimal(&self.parser.take(4));
            let superclass_idx = to_decimal(&self.parser.take(4));
            let interfaces_offset = to_decimal(&self.parser.take(4));
            let source_file_idx = to_decimal(&self.parser.take(4));
            let annotations_offset = to_decimal(&self.parser.take(4));
            let class_data_offset = to_decimal(&self.parser.take(4));
            let static_values_offset = to_decimal(&self.parser.take(4));

            let def = DexClassDef {
                address: addr,
                class_idx,
                access_flags,
                superclass_idx,
                interfaces_offset,
                source_file_idx,
                annotations_offset,
                class_data_offset,
                static_values_offset,
            };

            result.push(def);
        }

        self.class_defs = result;
    }
}

#[derive(Debug)]
pub struct DexHeader {
    pub dex_version: u8,
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
pub struct DexString {
    pub address: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct DexType {
    pub address: usize,
    pub descriptor_index: usize, // index into the string_ids list
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

pub struct DexProto {

}

#[derive(Debug)]
pub struct DexClassDef {
    pub address: usize,
    pub class_idx: u32, // index into type_ids
    pub access_flags: u32,
    pub superclass_idx: u32, // index into type_ids
    pub interfaces_offset: u32,
    pub source_file_idx: u32, // index into string_ids
    pub annotations_offset: u32,
    pub class_data_offset: u32,
    pub static_values_offset: u32,
}
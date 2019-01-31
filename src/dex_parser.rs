use crate::binary_parser::BinaryParser;
use crate::util::{to_decimal, to_decimal_short, to_ascii, to_utf8, print_hex, to_hex_string};
use crate::dex_types::*;
use std::io::{Cursor, Read};
use std::iter::{Iterator};

pub struct DexParser {
    parser: BinaryParser,
    pub header: Option<DexHeader>,
    pub strings: Vec<String>,
    pub types: Vec<DexType>,
    pub protos: Vec<DexProto>,
    pub fields: Vec<DexField>,
    pub methods: Vec<DexMethod>,
    pub class_defs: Vec<DexClassDef>,
}

impl DexParser {
    pub fn new(buffer: Vec<u8>) -> DexParser {
        DexParser {
            parser: BinaryParser::new(buffer),
            header: None,
            strings: Vec::new(),
            types: Vec::new(),
            protos: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            class_defs: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_header();
        self.parse_strings();
        self.parse_types();
        self.parse_protos();
        self.parse_fields();
        self.parse_methods();
        self.parse_class_defs();
    }
}

// TODO -  these methods should be converted to pure functions
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

        let endianness = match endian_constant.as_slice() {
            [0x78, 0x56, 0x34, 0x12] => Endianness::LittleEndian, 
            [0x12, 0x34, 0x56, 0x78] => Endianness::BigEndian,
            _ => {
                print_hex(&endian_constant);
                println!("Invalid endian constant");
                return;
            },
        };

        let header = DexHeader {
            dex_version: 0,                     // FIXME - grab actual version from dex_file_magic
            checksum: to_decimal(&checksum),    // FIXME - should this checksum be a decimal? hex?
            sha1: to_hex_string(&sha1).replace(" ", ""),
            file_size: to_decimal(&file_size),
            header_size: to_decimal(&header_size),
            endianness,
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
        let mut result: Vec<String> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            None => return,
        };
        let offset = header.string_ids_offset as usize;
        let size = (header.string_ids_size * 4) as usize;

        self.parser.seek_to(offset);
        let strings = self.parser.take(size);
        let mut string_parser = BinaryParser::new(strings);

        loop {
            if string_parser.is_it_the_end() {
                break;
            }

            let start_hex = &string_parser.take(4);
            let start_addr = to_decimal(start_hex) as usize;

            self.parser.seek_to(start_addr as usize);
            let length = self.parser.parse_uleb128();

            let s = self.parser.take_until(0x00);
            self.parser.expect(0x00);

            let actual_string = to_utf8(&s);
            result.push(actual_string);
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
            let s = self.strings[idx as usize].clone();
            let t = DexType {
                raw: s.clone(),
                parsed: parse_type_descriptor(s),
            };

            result.push(t);
        }

        self.types = result;
    }

    fn parse_protos(&mut self) {
        let mut result: Vec<DexProto> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            _ => return,
        };
        let offset = header.proto_ids_offset as usize;
        let size = (header.proto_ids_size * 12) as usize; // 3 fields, 4 bytes each

        self.parser.seek_to(offset);

        loop {
            let addr = self.parser.current_location();
            if addr >= offset + size {
                break;
            }

            let shorty_idx = to_decimal(&self.parser.take(4));
            let return_type_idx = to_decimal(&self.parser.take(4));
            let parameters_offset = to_decimal(&self.parser.take(4));
            
            let mut parameter_type_idx_list: Vec<u16> = Vec::new();
            if parameters_offset != 0 { // offset of 0 indicates no parameter_type_idx_list
                self.parser.seek_to(parameters_offset as usize);
                let parameter_count = to_decimal(&self.parser.take(4));

                for _ in 0..parameter_count {
                    let param_idx = to_decimal_short(&self.parser.take(2));
                    parameter_type_idx_list.push(param_idx);
                }
            }

            let proto = DexProto {
                shorty_idx,
                return_type_idx,
                parameters_offset,
                parameter_type_idx_list,
            };

            result.push(proto);
            self.parser.seek_to(addr + 12);
        }

        self.protos = result;
    }

    fn parse_fields(&mut self) {
        let mut result: Vec<DexField> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            _ => return,
        };
        let offset = header.field_ids_offset as usize;
        let size = (header.field_ids_size * 8) as usize; // 2 ushorts, 1 uint

        self.parser.seek_to(offset);

        loop {
            let addr = self.parser.current_location();

            if addr >= offset + size {
                break;
            }

            let class_idx = to_decimal_short(&self.parser.take(2));
            let type_idx = to_decimal_short(&self.parser.take(2));
            let name_idx = to_decimal(&self.parser.take(4));

            let field = DexField {
                class_idx: class_idx as u32,
                type_idx: type_idx as u32,
                name_idx,
            };

            result.push(field);
        }

        self.fields = result;
    }

    fn parse_methods(&mut self) {
        let mut result: Vec<DexMethod> = Vec::new();
        let header = match &self.header {
            Some(h) => h,
            _ => return,
        };
        let offset = header.method_ids_offset as usize;
        let size = (header.method_ids_size * 8) as usize; // 2 ushorts, 1 uint

        self.parser.seek_to(offset);

        loop {
            let addr = self.parser.current_location();

            if addr >= offset + size {
                break;
            }

            let class_idx = to_decimal_short(&self.parser.take(2));
            let proto_idx = to_decimal_short(&self.parser.take(2));
            let name_idx = to_decimal(&self.parser.take(4));

            let method = DexMethod {
                class_idx: class_idx as u32,
                proto_idx: proto_idx as u32,
                name_idx,
            };

            result.push(method);
        }

        self.methods = result;
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

            // Consuming 32 bytes here
            let class_idx = to_decimal(&self.parser.take(4));
            let access_flags = to_decimal(&self.parser.take(4));
            let superclass_idx = to_decimal(&self.parser.take(4));
            let interfaces_offset = to_decimal(&self.parser.take(4));
            let source_file_idx = to_decimal(&self.parser.take(4));
            let annotations_offset = to_decimal(&self.parser.take(4));
            let class_data_offset = to_decimal(&self.parser.take(4));
            let static_values_offset = to_decimal(&self.parser.take(4));

            if class_data_offset == 0 {
                continue;
            }

            // parse class data
            self.parser.seek_to(class_data_offset as usize);
            let static_fields_size = self.parser.parse_uleb128();
            let instance_fields_size = self.parser.parse_uleb128();
            let direct_methods_size = self.parser.parse_uleb128();
            let virtual_methods_size = self.parser.parse_uleb128();

            /*
                TODO - Refactor the following two chunks into functions:
                    1) Parse encoded fields
                    2) Parse encoded methods
            */
            let mut last_static_field_idx: Option<u32> = None;
            let mut static_fields: Vec<EncodedField> = Vec::new();
            for _ in 0..static_fields_size {
                let field_idx_diff = self.parser.parse_uleb128();
                let field_idx = match last_static_field_idx {
                    Some(idx) => idx + field_idx_diff,
                    None => field_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                static_fields.push(EncodedField{ field_idx, access_flags });
                last_static_field_idx = Some(field_idx);
            }

            let mut last_instance_field_idx: Option<u32> = None;
            let mut instance_fields: Vec<EncodedField> = Vec::new();
            for _ in 0..instance_fields_size {
                let field_idx_diff = self.parser.parse_uleb128();
                let field_idx = match last_instance_field_idx {
                    Some(idx) => idx + field_idx_diff,
                    None => field_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                instance_fields.push(EncodedField{ field_idx, access_flags });
                last_instance_field_idx = Some(field_idx);
            }

            let mut last_direct_method_idx: Option<u32> = None;
            let mut direct_methods: Vec<EncodedMethod> = Vec::new();
            for _ in 0..direct_methods_size {
                let method_idx_diff = self.parser.parse_uleb128();
                let method_idx = match last_direct_method_idx {
                    Some(idx) => idx + method_idx_diff,
                    None => method_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                let code_offset = self.parser.parse_uleb128();

                direct_methods.push(EncodedMethod{ method_idx, access_flags, code_offset });
                last_direct_method_idx = Some(method_idx);
            }

            let mut last_virtual_method_idx: Option<u32> = None;
            let mut virtual_methods: Vec<EncodedMethod> = Vec::new();
            for _ in 0..virtual_methods_size {
                let method_idx_diff = self.parser.parse_uleb128();
                let method_idx = match last_virtual_method_idx {
                    Some(idx) => idx + method_idx_diff,
                    None => method_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                let code_offset = self.parser.parse_uleb128();

                virtual_methods.push(EncodedMethod{ method_idx, access_flags, code_offset });
                last_virtual_method_idx = Some(method_idx);
            }

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
                static_fields,
                instance_fields,
                direct_methods,
                virtual_methods,
            };

            result.push(def);

            // Reset the parser back into class_defs, 32 bytes after 
            // the starting address of this def.
            self.parser.seek_to(addr + 32);
        }

        self.class_defs = result;
    }
}

fn parse_type_descriptor(s: String) -> TypeDescriptor {
    match s.chars().map(|x| x.clone()).collect::<Vec<char>>().as_slice() {
        ['V'] => TypeDescriptor::Void,
        ['Z'] => TypeDescriptor::Boolean,
        ['B'] => TypeDescriptor::Byte,
        ['S'] => TypeDescriptor::Short,
        ['C'] => TypeDescriptor::Char,
        ['I'] => TypeDescriptor::Int,
        ['J'] => TypeDescriptor::Long,
        ['F'] => TypeDescriptor::Float,
        ['D'] => TypeDescriptor::Double,
        c if *c.first().unwrap() == '[' => {
            let rest = c.iter().skip(1).collect::<String>();
            let nested_descriptor = parse_type_descriptor(rest);
            TypeDescriptor::Array(Box::new(nested_descriptor))
        },
        c if *c.first().unwrap() == 'L' => {
            let mut class_name = c.iter().skip(1).collect::<String>();
            class_name.pop(); // last char is always ';' so drop it
            TypeDescriptor::Class(class_name)
        }
        _ => TypeDescriptor:: Void,
    }
}

fn parse_encoded_fields() {}

fn parse_encoded_methods() {}

fn parse_header(parser: &mut BinaryParser) -> DexHeader {
    parser.seek_to(0);
    let dex_magic = parser.take(8);

    // assert!() that dex magic makes sense

    let checksum = parser.take(4);
    let sha1 = parser.take(20);
    let file_size = parser.take(4);
    let header_size = parser.take(4);
    let endian_constant = parser.take(4);
    let link_size = parser.take(4);
    let link_offset = parser.take(4);
    let map_offset = parser.take(4);
    let string_ids_size = parser.take(4);
    let string_ids_offset = parser.take(4);
    let type_ids_size = parser.take(4);
    let type_ids_offset = parser.take(4);
    let proto_ids_size = parser.take(4);
    let proto_ids_offset = parser.take(4);
    let field_ids_size = parser.take(4);
    let field_ids_offset = parser.take(4);
    let method_ids_size = parser.take(4);
    let method_ids_offset = parser.take(4);
    let class_defs_size = parser.take(4);
    let class_defs_offset = parser.take(4);
    let data_size = parser.take(4);
    let data_offset = parser.take(4);

    let endianness = match endian_constant.as_slice() {
        [0x78, 0x56, 0x34, 0x12] => Endianness::LittleEndian, 
        _ => Endianness::BigEndian,
    };

    DexHeader {
        dex_version: 0,                     // FIXME - grab actual version from dex_file_magic
        checksum: to_decimal(&checksum),    // FIXME - should this checksum be a decimal? hex?
        sha1: to_hex_string(&sha1).replace(" ", ""),
        file_size: to_decimal(&file_size),
        header_size: to_decimal(&header_size),
        endianness,
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
    }
}

// fn parse_strings(parser: &mut BinaryParser, offset: u32, list_size: u32) -> Vec<String> {
    
// }
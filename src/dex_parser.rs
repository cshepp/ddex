use crate::binary_parser::BinaryParser;
use crate::util::{to_decimal, to_decimal_short, to_ascii, print_hex};

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
            dex_version: 0, // FIXME - grab actual version from dex_file_magic
            checksum: to_decimal(&checksum), // FIXME - should this checksum be a decimal? hex?
            sha1: to_ascii(&sha1), // FIXME - this should probably be hex instead of ASCII
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
            self.parser.drop(1); // not quite correct... should instead read the size of the UTF16 string (should be a u32 in uleb128 format)
            // let mut e = self.parser.take_while(Box::new(|i: u8| {
            //     i & 0b0000_0001 == 1
            // }));
            // e.append(&mut self.parser.take(1));

            let s = self.parser.take_until(0x00);
            self.parser.expect(0x00);

            // print_hex(&e);
            // let len = to_ascii(&s).len();
            // println!("{}, {}", decode_uleb128(&e), len);
            // if len as u32 != decode_uleb128(&e) {
            //     //panic!("");
            // }
            result.push(to_ascii(&s));
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

            let shorty = self.strings[shorty_idx as usize].clone();
            let return_type = self.types[return_type_idx as usize].clone();
            
            let mut parameters: Vec<DexType> = Vec::new();
            
            if parameters_offset != 0 { // offset of 0 indicates no parameters
                self.parser.seek_to(parameters_offset as usize);
                let parameter_count = to_decimal(&self.parser.take(4));

                for _ in 0..parameter_count {
                    let param_idx = to_decimal_short(&self.parser.take(2));
                    parameters.push(self.types[param_idx as usize].clone());
                }
            }

            let proto = DexProto {
                shorty_idx,
                return_type_idx,
                parameters_offset,
                shorty,
                return_type,
                parameters,
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

            let field_type = self.types[type_idx as usize].clone();
            let field_name = self.strings[name_idx as usize].clone();

            let field = DexField {
                class_idx: class_idx as u32,
                type_idx: type_idx as u32,
                name_idx,
                field_type,
                field_name,
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

            let proto = self.protos[proto_idx as usize].clone();
            let method_name = self.strings[name_idx as usize].clone();

            let method = DexMethod {
                class_idx: class_idx as u32,
                proto_idx: proto_idx as u32,
                name_idx,
                proto,
                method_name,
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

            let class_idx = to_decimal(&self.parser.take(4));
            let access_flags = to_decimal(&self.parser.take(4));
            let superclass_idx = to_decimal(&self.parser.take(4));
            let interfaces_offset = to_decimal(&self.parser.take(4));
            let source_file_idx = to_decimal(&self.parser.take(4));
            let annotations_offset = to_decimal(&self.parser.take(4));
            let class_data_offset = to_decimal(&self.parser.take(4));
            let static_values_offset = to_decimal(&self.parser.take(4));

            let class_type = self.types[class_idx as usize].clone();
            let superclass_type = self.types[superclass_idx as usize].clone();
            let source_file = if source_file_idx == 4294967295 { // -1 is NO_INDEX
                "".to_string()
            } else {
                self.strings[source_file_idx as usize].clone()
            };

            println!("{:?} ------------------------------------------", class_type.parsed);
            println!("{:01$x}", addr, 2);

            // parse class data
            self.parser.seek_to(class_data_offset as usize);
            let static_fields_size = self.parser.parse_uleb128();
            let instance_fields_size = self.parser.parse_uleb128();
            let direct_methods_size = self.parser.parse_uleb128();
            let virtual_methods_size = self.parser.parse_uleb128();

            println!("Parsing {} static fields", static_fields_size);
            let mut last_static_field_idx: Option<u32> = None;
            for _ in 0..static_fields_size {
                let field_idx_diff = self.parser.parse_uleb128();
                let field_idx = match last_static_field_idx {
                    Some(idx) => idx + field_idx_diff,
                    None => field_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                
                let field = self.fields[field_idx as usize].clone();
                //println!("{:?} {}", field, access_flags);
                last_static_field_idx = Some(field_idx);
            }

            println!("Parsing {} instance fields", instance_fields_size);
            let mut last_instance_field_idx: Option<u32> = None;
            for _ in 0..instance_fields_size {
                let field_idx_diff = self.parser.parse_uleb128();
                let field_idx = match last_instance_field_idx {
                    Some(idx) => idx + field_idx_diff,
                    None => field_idx_diff,
                };
                let access_flags = self.parser.parse_uleb128();
                
                let field = self.fields[field_idx as usize].clone();
                //println!("{:?} {}", field, access_flags);
                last_instance_field_idx = Some(field_idx);
            }

            println!("Parsing {} direct methods", direct_methods_size);
            let mut last_direct_method_idx: Option<u32> = None;
            for _ in 0..direct_methods_size {
                let method_idx_diff = self.parser.parse_uleb128();
                let method_idx = match last_direct_method_idx {
                    Some(idx) => idx + method_idx_diff,
                    None => method_idx_diff,
                };

                if method_idx_diff == 0 {
                    println!("Method idx diff is 0 -------------------------------------------------------------- :(");
                }

                let access_flags = self.parser.parse_uleb128();
                let code_offset = self.parser.parse_uleb128();

                let method = self.methods[method_idx as usize].clone();
                println!("{:?}, {}, {}", method, access_flags, code_offset);

                last_direct_method_idx = Some(method_idx);
            }

            // for _ in 0..virtual_methods_size {
            //     let method_idx_diff = self.parser.parse_uleb128();
            //     let access_flags = self.parser.parse_uleb128();
            //     let code_offset = self.parser.parse_uleb128();
            // }

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
                class_type,
                superclass_type,
                source_file,
            };

            result.push(def);
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

#[derive(Debug, Clone)]
pub struct DexProto {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_offset: u32,
    pub shorty: String,
    pub return_type: DexType,
    pub parameters: Vec<DexType>,
}

#[derive(Debug, Clone)]
pub struct DexField {
    class_idx: u32,
    type_idx: u32,
    name_idx: u32,
    field_type: DexType,
    field_name: String,
}

#[derive(Debug, Clone)]
pub struct DexMethod {
    class_idx: u32,
    proto_idx: u32,
    name_idx: u32,
    proto: DexProto,
    method_name: String,
}

#[derive(Debug)]
pub struct DexClassDef {
    address: usize,
    class_idx: u32, // index into type_ids
    access_flags: u32,
    superclass_idx: u32, // index into type_ids
    interfaces_offset: u32,
    source_file_idx: u32, // index into string_ids
    annotations_offset: u32,
    class_data_offset: u32,
    static_values_offset: u32,
    //
    pub class_type: DexType,
    pub superclass_type: DexType,
    pub source_file: String,
}

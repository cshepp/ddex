use std::fs::File;
use std::io::Read;
use std::env;

mod util;
mod binary_parser;
mod dex_types;
mod dex_parser;
use crate::binary_parser::BinaryParser;
use crate::dex_parser::{parse_header, parse_strings, parse_types, parse_protos, parse_fields, parse_methods, parse_class_defs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].clone();

    let mut file = File::open(x).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut parser = BinaryParser::new(buf);
    let header = parse_header(&mut parser);
    let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
    let _types = parse_types(&mut parser, header.type_ids_offset as usize, header.type_ids_size as usize, strings);
    let _protos = parse_protos(&mut parser, header.proto_ids_offset as usize, header.proto_ids_size as usize);
    let _fields = parse_fields(&mut parser, header.field_ids_offset as usize, header.field_ids_size as usize);
    let _methods = parse_methods(&mut parser, header.method_ids_offset as usize, header.method_ids_size as usize);
    let _classes = parse_class_defs(&mut parser, header.class_defs_offset as usize, header.class_defs_size as usize);

    println!("{:?}", header);
}

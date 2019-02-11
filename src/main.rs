use std::fs::File;
use std::io::Read;
use std::env;

mod analysis;
mod binary_parser;
mod dex_parser;
mod dex_types;
mod instructions;
mod printer;
mod util;
use crate::analysis::*;
use crate::binary_parser::BinaryParser;
use crate::dex_parser::{parse_header, parse_strings, parse_types, parse_protos, parse_fields, parse_methods, parse_class_defs};
use crate::printer::Printer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].clone();

    let mut file = File::open(x).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut parser = BinaryParser::new(buf);
    let header  = parse_header(&mut parser);
    let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
    let types   = parse_types(&mut parser, header.type_ids_offset as usize, header.type_ids_size as usize, &strings);
    let protos  = parse_protos(&mut parser, header.proto_ids_offset as usize, header.proto_ids_size as usize);
    let fields  = parse_fields(&mut parser, header.field_ids_offset as usize, header.field_ids_size as usize);
    let methods = parse_methods(&mut parser, header.method_ids_offset as usize, header.method_ids_size as usize);
    let classes = parse_class_defs(&mut parser, header.class_defs_offset as usize, header.class_defs_size as usize);

    for c in classes.iter() {
        println!("{:?}", c);
    }

    let mut printer = Printer {
        strings,
        types,
        protos,
        fields,
        methods,
        classes,
        parser,
    };

    //printer.print_classes();
}

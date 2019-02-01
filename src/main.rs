use std::fs::File;
use std::io::Read;
use std::env;

mod util;
mod binary_parser;
mod dex_types;
mod dex_parser;
use crate::binary_parser::BinaryParser;
use crate::dex_parser::{parse_header, parse_class_defs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].clone();

    let mut file = File::open(x).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut parser = BinaryParser::new(buf);
    let header = parse_header(&mut parser);
    let _strings = parse_class_defs(&mut parser, header.class_defs_offset as usize, header.class_defs_size as usize);

}

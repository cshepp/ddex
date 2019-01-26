use std::fs::File;
use std::io::Read;
use std::env;


mod binary_parser;
use crate::binary_parser::BinaryParser;

mod dex_parser;
use crate::dex_parser::{DexParser, TypeDescriptor};

mod util;
use crate::util::{print_ascii, print_hex, to_decimal, decode_uleb128};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let x = args[1].clone();

    let mut file = File::open(x).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut dex_parser = DexParser::new(buf);
    dex_parser.parse();

    for c in dex_parser.class_defs {
        println!("{:?}", c);
    }
}

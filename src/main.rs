use std::fs::File;
use std::io::Read;
use std::env;

mod util;
mod binary_parser;
mod dex_types;
mod dex_parser;
use crate::dex_parser::{DexParser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].clone();

    let mut file = File::open(x).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut dex_parser = DexParser::new(buf);
    dex_parser.parse();

    println!("{:?}", dex_parser.header);
}

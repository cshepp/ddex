use std::fs::File;
use std::io::Read;
use std::env;

use clap::{Arg, App, SubCommand, AppSettings};

mod analysis;
mod binary_parser;
mod dex_parser;
mod dex_types;
mod instructions;
mod printer;
mod util;
use crate::binary_parser::BinaryParser;
use crate::dex_parser::{parse_header, parse_strings, parse_types, parse_protos, parse_fields, parse_methods, parse_class_defs};
use crate::instructions::Instruction;
use crate::printer::get_type_descriptor_string;

fn main() {

    let mut app = App::new("ddex")
        .version("0.1")
        .author("Cody Shepp <me@codyshepp.com>")
        .about("A suite of dex/dalvik tools")
        .arg(Arg::with_name("FILE")
            .index(1)
            .required(true))
        .subcommand(SubCommand::with_name("header")
            .about("Prints header information from the dex file"))
        .subcommand(SubCommand::with_name("strings")
            .about("Prints the strings contained within the dex file"))
        .subcommand(SubCommand::with_name("types")
            .about("Prints the names of the types contained within the dex file"))
        .subcommand(SubCommand::with_name("classes")
            .about("Prints the names of the classes contained within the dex file"))
        .subcommand(SubCommand::with_name("disassemble"));

    let args: Vec<String> = env::args().collect();
    let matches = match app.get_matches_from_safe_borrow(args) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let file_path = matches.value_of("FILE").unwrap();
    let mut file = File::open(file_path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut parser = BinaryParser::new(buf);

    match matches.subcommand_name() {
        Some("header") => {
            let header  = parse_header(&mut parser);
            println!("{}", header);
        }
        Some("strings") => {
            let header  = parse_header(&mut parser);
            let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
            for s in strings {
                println!("{}", s);
            }
        }
        Some("types") => {
            let header  = parse_header(&mut parser);
            let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
            let types   = parse_types(&mut parser, header.type_ids_offset as usize, header.type_ids_size as usize, &strings);
            for t in types {
                println!("{}", get_type_descriptor_string(&t.parsed));
            }
        }
        Some("classes") => {
            let header  = parse_header(&mut parser);
            let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
            let types   = parse_types(&mut parser, header.type_ids_offset as usize, header.type_ids_size as usize, &strings);
            let classes = parse_class_defs(&mut parser, header.class_defs_offset as usize, header.class_defs_size as usize);
            for c in classes {
                let class_name = &types[c.class_idx as usize];
                println!("{}", get_type_descriptor_string(&class_name.parsed));
            }
        }
        Some("disassemble") => {
            let header  = parse_header(&mut parser);
            let strings = parse_strings(&mut parser, header.string_ids_offset as usize, header.string_ids_size as usize);
            let types   = parse_types(&mut parser, header.type_ids_offset as usize, header.type_ids_size as usize, &strings);
            let classes = parse_class_defs(&mut parser, header.class_defs_offset as usize, header.class_defs_size as usize);
            let mut instructions: Vec<Instruction> = Vec::new();
            for c in classes {
                for d in c.direct_methods {
                    match d.code_item {
                        Some(mut code) => {
                            instructions.append(&mut code.instructions);
                        }
                        None => {}
                    }
                }
            }

            for i in instructions {
                println!("{}", i);
            }
        }
        Some(_) | None => app.print_help().expect(""),
    }

    // let protos  = parse_protos(&mut parser, header.proto_ids_offset as usize, header.proto_ids_size as usize);
    // let fields  = parse_fields(&mut parser, header.field_ids_offset as usize, header.field_ids_size as usize);
    // let methods = parse_methods(&mut parser, header.method_ids_offset as usize, header.method_ids_size as usize);
}
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
use crate::instructions::Instruction;
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

    // let mut printer = Printer {
    //     strings,
    //     types,
    //     protos,
    //     fields,
    //     methods,
    //     classes,
    //     parser,
    // };

    let mut instructions: Vec<Instruction> = Vec::new();

    for c in classes.iter() {
        for m in c.direct_methods.iter() {
            match &m.code_item {
                Some(a) => {
                    instructions.append(&mut a.instructions.clone());
                },
                None => {}
            }
        }
    }

    let g = control_flow_graph(&instructions);

    for e in &g.edges {
        let b = g.edges.clone();
        let (i, _) = e;
        let graph = walk(*i, &b);

        if graph.len() > 2 {
            println!("-----------");
            for n in graph {
                println!("{:?}", n);
            }
        }
    }
}

fn walk(i: usize, edges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    //println!("walking starting at {}", i);

    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut connections = edges.iter().filter(|(a, b)| *a == i).map(|(a, b)| (*a, *b)).collect::<Vec<(usize, usize)>>();
    //println!("found: {:?}" , connections);
    result.append(&mut connections.clone());
    for (a, b) in connections {
        if b == i {
            //println!("{} == {}", b, i);
            continue;
        }
        //println!("walking subgraph");
        let mut others = walk(b, edges);
        result.append(&mut others);
    }

    //println!("returning {:?}", result);

    return result;
}
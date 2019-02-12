
use crate::instructions::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub starting_addr: usize,
}

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: Vec<T>,
    pub edges: Vec<(usize, usize)>,
}

pub fn control_flow_graph(ins: &Vec<Instruction>) -> Graph<Block> {
    let mut nodes: Vec<Block> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut node_lookup: HashMap<usize, usize> = HashMap::new();

    let mut block_start_addrs: Vec<usize> = Vec::new();

    let mut flag = true;
    for i in ins.iter() {

        if flag {
            //println!("ADDR: {}", i.addr);
            block_start_addrs.push(i.addr);
            flag = false;
        }

        match i.kind {
              InstructionKind::GoTo(o)
            | InstructionKind::GoTo16(o)
            | InstructionKind::GoTo32(o)
            | InstructionKind::IfEq(_,_,o)
            | InstructionKind::IfNe(_,_,o)
            | InstructionKind::IfLt(_,_,o)
            | InstructionKind::IfGe(_,_,o)
            | InstructionKind::IfGt(_,_,o)
            | InstructionKind::IfLe(_,_,o)
            | InstructionKind::IfEqZ(_,o)
            | InstructionKind::IfNeZ(_,o)
            | InstructionKind::IfLtZ(_,o)
            | InstructionKind::IfGeZ(_,o)
            | InstructionKind::IfGtZ(_,o)
            | InstructionKind::IfLeZ(_,o) => {
                block_start_addrs.push((i.addr as i32 + o) as usize);
                //println!("{} {:?}", i.addr as i64 + (o as i64 * 2), i.kind);
                flag = true;
            }
            InstructionKind::ReturnVoid => {
                //println!("{:?}", i.kind);
                flag = true;
            }
            _ => {}
        };
    }

    block_start_addrs.sort_unstable();
    block_start_addrs.dedup();
    block_start_addrs.reverse();

    let mut buffer: Vec<Instruction> = Vec::new();
    for inst in ins {
        //println!("addr: {}, {}", inst.addr, block_start_addrs.last().unwrap());

        if inst.addr > *block_start_addrs.last().unwrap() {
            break;
        }

        if inst.addr == *block_start_addrs.last().unwrap() {
            let addr = *block_start_addrs.last().unwrap();
            let block = Block {
                starting_addr: addr,
                instructions: buffer,
            };
            node_lookup.insert(addr, nodes.len());
            nodes.push(block);
            buffer = Vec::new();

            if block_start_addrs.len() == 0 {
                break;
            }

            let _ = block_start_addrs.pop().unwrap();
        } else {
            buffer.push(inst.clone());
        }
    }

    let mut misses: Vec<Instruction> = Vec::new();
    for (i,n) in nodes.iter().enumerate() {
        if n.instructions.len() == 0 {
            continue;
        }


        let last_instruction = n.instructions.last().unwrap();
        match last_instruction.kind {
              InstructionKind::GoTo(o)
            | InstructionKind::GoTo16(o)
            | InstructionKind::GoTo32(o)
            | InstructionKind::IfEq(_,_,o)
            | InstructionKind::IfNe(_,_,o)
            | InstructionKind::IfLt(_,_,o)
            | InstructionKind::IfGe(_,_,o)
            | InstructionKind::IfGt(_,_,o)
            | InstructionKind::IfLe(_,_,o)
            | InstructionKind::IfEqZ(_,o)
            | InstructionKind::IfNeZ(_,o)
            | InstructionKind::IfLtZ(_,o)
            | InstructionKind::IfGeZ(_,o)
            | InstructionKind::IfGtZ(_,o)
            | InstructionKind::IfLeZ(_,o) => {
                let destination = last_instruction.addr as i32 + o;
                let destination_idx = node_lookup.get(&(destination as usize));
                match destination_idx {
                    Some(idx) => {
                        edges.push((i, *idx));
                    }
                    None => {
                        misses.push(last_instruction.clone());
                    }
                }
            }
            _ => {}
        }
    }

    for x in misses {
        println!("{:?}", x);
    }

    return Graph {
        nodes,
        edges,
    };
}
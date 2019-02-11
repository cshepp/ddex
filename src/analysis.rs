
use crate::instructions::*;
use std::collections::HashMap;

#[derive(Debug)]
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

    let mut buffer: Vec<Instruction> = Vec::new();
    for i in ins.iter() {
        buffer.push(i.clone());
        match i.kind {
              InstructionKind::GoTo(_)
            | InstructionKind::GoTo16(_)
            | InstructionKind::GoTo32(_)
            | InstructionKind::IfEq(_,_,_)
            | InstructionKind::IfNe(_,_,_)
            | InstructionKind::IfLt(_,_,_)
            | InstructionKind::IfGe(_,_,_)
            | InstructionKind::IfGt(_,_,_)
            | InstructionKind::IfLe(_,_,_)
            | InstructionKind::IfEqZ(_,_)
            | InstructionKind::IfNeZ(_,_)
            | InstructionKind::IfLtZ(_,_)
            | InstructionKind::IfGeZ(_,_)
            | InstructionKind::IfGtZ(_,_)
            | InstructionKind::IfLeZ(_,_) => {
                let block = Block {
                    starting_addr: buffer[0].addr,
                    instructions: buffer,
                };
                node_lookup.insert(block.starting_addr, nodes.len());
                nodes.push(block);
                buffer = Vec::new();
            }
            _ => {}
        };
    }

    for (i,n) in nodes.iter().enumerate() {
        let last_instruction = n.instructions.last().unwrap();
        match last_instruction.kind {
            InstructionKind::GoTo(x) => {
                let destination = last_instruction.addr + x as usize;
                let destination_idx = node_lookup.get(&destination);
                match destination_idx {
                    Some(idx) => {
                        edges.push((i, *idx));
                    }
                    None => {}
                }
            }
            _ => {}
        }
    }


    return Graph {
        nodes,
        edges,
    };
}

use crate::instructions::*;

pub struct Block {
    pub instructions: Vec<Instruction>,
}

pub struct Graph<T> {
    pub nodes: Vec<T>,
    pub edges: Vec<(usize, usize)>,
}

pub fn code_flow_graph(ins: Vec<Instruction>) -> Graph<Block> {
    let mut nodes: Vec<Block> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    let mut buffer: Vec<Instruction> = Vec::new();
    for i in ins {
        buffer.push(i.clone());
        match i {
              Instruction::GoTo(_)
            | Instruction::GoTo16(_)
            | Instruction::GoTo32(_)
            | Instruction::IfEq(_,_,_)
            | Instruction::IfNe(_,_,_)
            | Instruction::IfLt(_,_,_)
            | Instruction::IfGe(_,_,_)
            | Instruction::IfGt(_,_,_)
            | Instruction::IfLe(_,_,_)
            | Instruction::IfEqZ(_,_)
            | Instruction::IfNeZ(_,_)
            | Instruction::IfLtZ(_,_)
            | Instruction::IfGeZ(_,_)
            | Instruction::IfGtZ(_,_)
            | Instruction::IfLeZ(_,_) => {
                let block = Block {
                    instructions: buffer,
                };
                nodes.push(block);
                buffer = Vec::new();
            }
            _ => {}
        };
    }


    return Graph {
        nodes,
        edges,
    };
}
use crate::util::{to_decimal, to_decimal_short};
use crate::dex_types::*;

type RegisterIndex = u32;

pub enum Instruction {
    Nop,
    Move(u8, u8),
    MoveFrom16(u8, u16),
    Move16(u16, u16),
    MoveWide(u8, u8),
    MoveWideFrom16(u8, u16),
    MoveWide16(u16, u16),
    MoveObject(u8, u8),
    MoveObjectFrom16(u8, u16),
    MoveObject16(u16, u16),
    MoveResult(u8),
    MoveResultWide(u8),
    MoveResultObject(u8),
    MoveException(u8),
    ReturnVoid,
    Return(u8),
    ReturnWide(u8),
    ReturnObject(u8),
    Const4(u8, u8),
    Const16(u8, u16),
    Const(u8, u32),
    ConstHigh16(u8, u16),
    ConstWide16(u8, u16),
    ConstWide32(u8, u32),
    ConstWide(u8, u64),
    ConstWideHigh16(u8, u16),
    ConstString(u8, u16),
    ConstStringJumbo(u8, u32),
    ConstClass(u8, u16),
    MonitorEnter(u8),
    MonitorExit(u8),
    CheckCast(u8, u16),
    InstanceOf(u8, u8, u16),
    ArrayLength(u8, u8),
    NewInstance(RegisterIndex, TypeIndex),
    NewArray(u8, u8, u16),
    FilledNewArray,      // TODO
    FilledNewArrayRange, // TODO
    FillArrayData,       // TODO
    Throw(u8),
    GoTo(i8),
    GoTo16(i16),
    GoTo32(i32),
    PackedSwitch(u8, i32),
    SpareSwitch(u8, i32),
    CmpLFloat(u8, u8, u8),
    CmpGFloat(u8, u8, u8),
    CmpLDouble(u8, u8, u8),
    CmpGDouble(u8, u8, u8),
    CmpLong(u8, u8, u8),
    IfEq(u8, u8, i16),
    IfNe(u8, u8, i16),
    IfLt(u8, u8, i16),
    IfGe(u8, u8, i16),
    IfGt(u8, u8, i16),
    IfLe(u8, u8, i16),
    IfEqZ(u8, i16),
    IfNeZ(u8, i16),
    IfLtZ(u8, i16),
    IfGeZ(u8, i16),
    IfGtZ(u8, i16),
    IfLeZ(u8, i16),
    AGet(u8, u8, u8),
    AGetWide(u8, u8, u8),
    AGetObject(u8, u8, u8),
    AGetBoolean(u8, u8, u8),
    AGetByte(u8, u8, u8),
    AGetChar(u8, u8, u8),
    AGetShort(u8, u8, u8),
    APut(u8, u8, u8),
    APutWidth(u8, u8, u8),
    APutObject(u8, u8, u8),
    APutBoolean(u8, u8, u8),
    APutByte(u8, u8, u8),
    APutChar(u8, u8, u8),
    APutShort(u8, u8, u8),
    IGet(u8, u8, u16),
    IGetWide(u8, u8, u16),
    IGetObject(u8, u8, u16),
    IGetBoolean(u8, u8, u16),
    IGetByte(u8, u8, u16),
    IGetChar(u8, u8, u16),
    IGetShort(u8, u8, u16),
    IPut(u8, u8, u16),
    IPutWide(u8, u8, u16),
    IPutObject(u8, u8, u16),
    IPutBoolean(u8, u8, u16),
    IPutByte(u8, u8, u16),
    IPutChar(u8, u8, u16),
    IPutShort(u8, u8, u16),
    SGet(u8, u16),
    SGetWide(u8, u16),
    SGetObject(u8, u16),
    SGetBoolean(u8, u16),
    SGetByte(u8, u16),
    SGetChar(u8, u16),
    SGetShort(u8, u16),
    SPut(u8, u16),
    SPutWide(u8, u16),
    SPutObject(u8, u16),
    SPutBoolean(u8, u16),
    SPutByte(u8, u16),
    SPutChar(u8, u16),
    SPutShort(u8, u16),
    InvokeVirtual,          // TODO 
    InvokeSuper,            // TODO 
    InvokeDirect,           // TODO 
    InvokeStatic,           // TODO 
    InvokeInterface,        // TODO 
    InvokeVirtualRange,     // TODO 
    InvokeSuperRange,       // TODO 
    InvokeDirectRange,      // TODO 
    InvokeStaticRange,      // TODO 
    InvokeInterfaceRange,   // TODO 
    NegInt(u8, u8),
    NotInt(u8, u8),
    NegLong(u8, u8),
    NotLong(u8, u8),
    NegFloat(u8, u8),
    NegDouble(u8, u8),
    IntToLong(u8, u8),
    IntToFloat(u8, u8),
    IntToDouble(u8, u8),
    LongToInt(u8, u8),
    LongToFloat(u8, u8),
    LongToDouble(u8, u8),
    FloatToInt(u8, u8),
    FloatToLong(u8, u8),
    FloatToDouble(u8, u8),
    DoubleToInt(u8, u8),
    DoubleToLong(u8, u8),
    DoubleToFloat(u8, u8),
    IntToByte(u8, u8),
    IntToChar(u8, u8),
    IntToShort(u8, u8),
    AddInt(u8, u8, u8),
    SubInt(u8, u8, u8),
    MulInt(u8, u8, u8),
    DivInt(u8, u8, u8),
    RemInt(u8, u8, u8),
    AndInt(u8, u8, u8),
    OrInt(u8, u8, u8),
    XorInt(u8, u8, u8),
    ShlInt(u8, u8, u8),
    ShrInt(u8, u8, u8),
    UShrInt(u8, u8, u8),
    AddLong(u8, u8, u8),
    SubLong(u8, u8, u8),
    MulLong(u8, u8, u8),
    DivLong(u8, u8, u8),
    RemLong(u8, u8, u8),
    AndLong(u8, u8, u8),
    OrLong(u8, u8, u8),
    XorLong(u8, u8, u8),
    ShlLong(u8, u8, u8),
    ShrLong(u8, u8, u8),
    UShrLong(u8, u8, u8),
    AddFloat(u8, u8, u8),
    SubFloat(u8, u8, u8),
    MulFloat(u8, u8, u8),
    DivFloat(u8, u8, u8),
    RemFloat(u8, u8, u8),
    AddDouble(u8, u8, u8),
    SubDouble(u8, u8, u8),
    MulDouble(u8, u8, u8),
    DivDouble(u8, u8, u8),
    RemDouble(u8, u8, u8),
    AddInt2Addr(u8, u8),
    SubInt2Addr(u8, u8),
    MulInt2Addr(u8, u8),
    DivInt2Addr(u8, u8),
    RemInt2Addr(u8, u8),
    AndInt2Addr(u8, u8),
    OrInt2Addr(u8, u8),
    XorInt2Addr(u8, u8),
    ShlInt2Addr(u8, u8),
    ShrInt2Addr(u8, u8),
    UShrInt2Addr(u8, u8),
    AddLong2Addr(u8, u8),
    SubLong2Addr(u8, u8),
    MulLong2Addr(u8, u8),
    DivLong2Addr(u8, u8),
    RemLong2Addr(u8, u8),
    AndLong2Addr(u8, u8),
    OrLong2Addr(u8, u8),
    XorLong2Addr(u8, u8),
    ShlLong2Addr(u8, u8),
    ShrLong2Addr(u8, u8),
    UShrLong2Addr(u8, u8),
    AddFloat2Addr(u8, u8),
    SubFloat2Addr(u8, u8),
    MulFloat2Addr(u8, u8),
    DivFloat2Addr(u8, u8),
    RemFloat2Addr(u8, u8),
    AddDouble2Addr(u8, u8),
    SubDouble2Addr(u8, u8),
    MulDouble2Addr(u8, u8),
    DivDouble2Addr(u8, u8),
    RemDouble2Addr(u8, u8),
    AddIntLit16(u8, u8, i16),
    RSubIntLit16(u8, u8, i16),
    MulIntLit16(u8, u8, i16),
    DivIntLit16(u8, u8, i16),
    RemIntLit16(u8, u8, i16),
    AndIntLit16(u8, u8, i16),
    OrIntLit16(u8, u8, i16),
    XorIntLit16(u8, u8, i16),
    AddIntLit8(u8, u8, i8),
    RSubIntLit8(u8, u8, i8),
    MulIntLit8(u8, u8, i8),
    DivIntLit8(u8, u8, i8),
    RemIntLit8(u8, u8, i8),
    AndIntLit8(u8, u8, i8),
    OrIntLit8(u8, u8, i8),
    XorIntLit8(u8, u8, i8),
    ShlIntLit8(u8, u8, i8),
    ShrIntLit8(u8, u8, i8),
    UShrIntLit8(u8, u8, i8),
    InvokePolymorphic,      // TODO 
    InvokePolymorphicRange, // TODO 
    InvokeCustom,           // TODO 
    InvokeCustomRange,      // TODO 
    ConstMethodHandle,      // TODO 
    ConstMethodType,        // TODO 
}

pub fn parse_bytecode(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    let mut v = bytes;
    loop {
        if v.len() == 0 {
            break;
        }

        let (i, n) = bytecode_to_instruction(&v);
        result.push(i);
        let x = n as usize;
        v = v[x..].to_vec();
    }

    return result;
}

pub fn instruction_to_string(i: Instruction) -> String {
    match i {
        Instruction::Nop => "nop".to_string(),
        Instruction::Move(a, b) => format!("move v{} v{}", a, b),
        Instruction::MoveFrom16(a, b) => format!("move/from16 v{} v{}", a, b),
        _ => "".to_string(),
    }
}

// returns instruction and # bytes consumed
fn bytecode_to_instruction(x: &Vec<u8>) -> (Instruction, u32) {
    match x[0] {
        0x00 => (Instruction::Nop, 1),
        0x01 => (Instruction::Move(x[1], x[2]), 3),
        0x02 => (Instruction::MoveFrom16(x[1], to_u16(&x[2..4])), 4),
        0x03 => (Instruction::Move16(to_u16(&x[1..3]), to_u16(&x[3..5])), 5),
        0x04 => (Instruction::MoveWide(x[1], x[2]), 3),
        0x05 => (Instruction::MoveWideFrom16(x[1], to_u16(&x[2..4])), 4),
        0x06 => (Instruction::MoveWide16(to_u16(&x[1..3]), to_u16(&x[3..5])), 5),
        0x07 => (Instruction::MoveObject(x[1], x[2]), 3),
        0x08 => (Instruction::MoveObjectFrom16(x[1], to_u16(&x[2..4])), 4),
        0x09 => (Instruction::MoveObject16(to_u16(&x[1..3]), to_u16(&x[3..5])), 5),
        0x0a => (Instruction::MoveResult(x[1]), 2),
        0x0b => (Instruction::MoveResultWide(x[1]), 2),
        0x0c => (Instruction::MoveResultObject(x[1]), 2),
        0x0d => (Instruction::MoveException(x[1]), 2),
        0x0e => (Instruction::ReturnVoid, 1),
        0x0f => (Instruction::Return(x[1]), 2),
        0x10 => (Instruction::ReturnWide(x[1]), 2),
        0x11 => (Instruction::ReturnObject(x[1]), 2),
        //0x12 => (Instruction::Const4()) // TODO - split 1st byte into 4 bits
        0x13 => (Instruction::Const16(x[1], to_u16(&x[2..4])), 4),
        0x14 => (Instruction::Const(x[1], to_u32(&x[2..10])), 10),
        0x15 => (Instruction::ConstHigh16(x[1], to_u16(&x[2..10])), 10), // TODO is this right? 2nd should be i16...
        0x16 => (Instruction::ConstWide16(x[1], to_u16(&x[2..4])), 4),
        0x17 => (Instruction::ConstWide32(x[1], to_u32(&x[2..10])), 10),
        0x18 => (Instruction::ConstWide(x[1], to_u32(&x[2..18]) as u64), 18),
        0x19 => (Instruction::ConstWideHigh16(x[1], to_u16(&x[2..18])), 18),
        0x1a => (Instruction::ConstString(x[1], to_u16(&x[2..6])), 6),
        0x1b => (Instruction::ConstStringJumbo(x[1], to_u32(&x[2..10])), 10),
        0x1c => (Instruction::ConstClass(x[1], to_u16(&x[2..6])), 6),
        0x1d => (Instruction::MonitorEnter(x[1]), 2),
        0x1e => (Instruction::MonitorExit(x[1]), 2),
        0x1f => (Instruction::CheckCast(x[1], to_u16(&x[2..6])), 6),
        //0x20 => (Instruction::InstanceOf()) - again, split first byte into 4 bits
        0x21 => (Instruction::ArrayLength(x[1], x[2]), 3),
        0x22 => (Instruction::NewInstance(vA(x[1]), type_index(&x[2..6]) ), 6),
        0x23 => (Instruction::NewArray(x[1], x[2], to_u16(&x[3..7])), 7),
        //0x24 => (Instruction::FilledNewArray()),
        //0x25 => (Instruction::FilledNewArrayRange()),
        //0x26 => (Instruction::FillArrayData(x[1], to_i32(&x[2..10])), 10),
        0x27 => (Instruction::Throw(x[1]), 2),
        //0x28 => (Instruction::GoTo(x[1]), 2),
        //0x29 => (Instruction::GoTo16(to_u16(&x[1..3])), 3),
        //0x2a => (Instruction::GoTo32(to_u32(&x[1..9])), 9),
        //0x2b => (Instruction::PackedSwitch(x[1], to_i32(&x[2..10])), 10),

        _ => (Instruction::Nop, 1),
    }
}

fn vA(v: u8) -> RegisterIndex {
    v as RegisterIndex
}

fn vAA(v: &[u8]) -> RegisterIndex {
    to_decimal(&v.to_vec()) as RegisterIndex
}

fn type_index(s: &[u8]) -> TypeIndex {
    to_decimal(&s.to_vec()) as TypeIndex
}

fn to_u16(s: &[u8]) -> u16 {
    to_decimal_short(&s.to_vec())
}

fn to_u32(s: &[u8]) -> u32 {
    to_decimal(&s.to_vec())
}

fn to_i32(s: &[u8]) -> i32 {
    0
    // TODO
}

#[test]
pub fn test_stuff() {
    let bytecode = vec![0x02, 0x01, 0x03, 0x01];

    let (instruction, n) = bytecode_to_instruction(&bytecode);
    assert_eq!(n, 4);
    match instruction {
        Instruction::MoveFrom16(a, b) => {
            assert_eq!(a, 1);
            assert_eq!(b, 259);
        },
        _ => panic!()
    }
}


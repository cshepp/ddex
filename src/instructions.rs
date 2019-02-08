use crate::util::{to_decimal, to_decimal_short, to_hex_string};
use crate::dex_types::*;
use crate::binary_parser::BinaryParser;

type Register = u32;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Move(Register, Register),
    MoveFrom16(Register, Register),
    Move16(Register, Register),
    MoveWide(Register, Register),
    MoveWideFrom16(Register, Register),
    MoveWide16(Register, Register),
    MoveObject(Register, Register),
    MoveObjectFrom16(Register, Register),
    MoveObject16(Register, Register),
    MoveResult(Register),
    MoveResultWide(Register),
    MoveResultObject(Register),
    MoveException(Register),
    ReturnVoid,
    Return(Register),
    ReturnWide(Register),
    ReturnObject(Register),
    Const4(Register, i32),
    Const16(Register, i32),
    Const(Register, i32),
    ConstHigh16(Register, i32),
    ConstWide16(Register, i64),
    ConstWide32(Register, i64),
    ConstWide(Register, i64),
    ConstWideHigh16(Register, i64),
    ConstString(Register, StringIndex),
    ConstStringJumbo(Register, StringIndex),
    ConstClass(Register, TypeIndex),
    MonitorEnter(Register),
    MonitorExit(Register),
    CheckCast(Register, TypeIndex),
    InstanceOf(Register, Register, TypeIndex),
    ArrayLength(Register, Register),
    NewInstance(Register, TypeIndex),
    NewArray(Register, Register, TypeIndex),
    FilledNewArray,      // TODO
    FilledNewArrayRange, // TODO
    FillArrayData,       // TODO
    Throw(Register),
    GoTo(i32),
    GoTo16(i32),
    GoTo32(i32),
    PackedSwitch,// TODO
    SpareSwitch, // TODO
    CmpLFloat(Register, Register, Register),
    CmpGFloat(Register, Register, Register),
    CmpLDouble(Register, Register, Register),
    CmpGDouble(Register, Register, Register),
    CmpLong(Register, Register, Register),
    IfEq(Register, Register, i32),
    IfNe(Register, Register, i32),
    IfLt(Register, Register, i32),
    IfGe(Register, Register, i32),
    IfGt(Register, Register, i32),
    IfLe(Register, Register, i32),
    IfEqZ(Register, i32),
    IfNeZ(Register, i32),
    IfLtZ(Register, i32),
    IfGeZ(Register, i32),
    IfGtZ(Register, i32),
    IfLeZ(Register, i32),
    AGet(Register, Register, Register),
    AGetWide(Register, Register, Register),
    AGetObject(Register, Register, Register),
    AGetBoolean(Register, Register, Register),
    AGetByte(Register, Register, Register),
    AGetChar(Register, Register, Register),
    AGetShort(Register, Register, Register),
    APut(Register, Register, Register),
    APutWide(Register, Register, Register),
    APutObject(Register, Register, Register),
    APutBoolean(Register, Register, Register),
    APutByte(Register, Register, Register),
    APutChar(Register, Register, Register),
    APutShort(Register, Register, Register),
    IGet(Register, Register, FieldIndex),
    IGetWide(Register, Register, FieldIndex),
    IGetObject(Register, Register, FieldIndex),
    IGetBoolean(Register, Register, FieldIndex),
    IGetByte(Register, Register, FieldIndex),
    IGetChar(Register, Register, FieldIndex),
    IGetShort(Register, Register, FieldIndex),
    IPut(Register, Register, FieldIndex),
    IPutWide(Register, Register, FieldIndex),
    IPutObject(Register, Register, FieldIndex),
    IPutBoolean(Register, Register, FieldIndex),
    IPutByte(Register, Register, FieldIndex),
    IPutChar(Register, Register, FieldIndex),
    IPutShort(Register, Register, FieldIndex),
    SGet(Register, FieldIndex),
    SGetWide(Register, FieldIndex),
    SGetObject(Register, FieldIndex),
    SGetBoolean(Register, FieldIndex),
    SGetByte(Register, FieldIndex),
    SGetChar(Register, FieldIndex),
    SGetShort(Register, FieldIndex),
    SPut(Register, FieldIndex),
    SPutWide(Register, FieldIndex),
    SPutObject(Register, FieldIndex),
    SPutBoolean(Register, FieldIndex),
    SPutByte(Register, FieldIndex),
    SPutChar(Register, FieldIndex),
    SPutShort(Register, FieldIndex),
    InvokeVirtual(Vec<Register>, MethodIndex),
    InvokeSuper(Vec<Register>, MethodIndex),
    InvokeDirect(Vec<Register>, MethodIndex),
    InvokeStatic(Vec<Register>, MethodIndex),
    InvokeInterface(Vec<Register>, MethodIndex),
    InvokeVirtualRange,     // TODO 
    InvokeSuperRange,       // TODO 
    InvokeDirectRange,      // TODO 
    InvokeStaticRange,      // TODO 
    InvokeInterfaceRange,   // TODO 
    NegInt(Register, Register),
    NotInt(Register, Register),
    NegLong(Register, Register),
    NotLong(Register, Register),
    NegFloat(Register, Register),
    NegDouble(Register, Register),
    IntToLong(Register, Register),
    IntToFloat(Register, Register),
    IntToDouble(Register, Register),
    LongToInt(Register, Register),
    LongToFloat(Register, Register),
    LongToDouble(Register, Register),
    FloatToInt(Register, Register),
    FloatToLong(Register, Register),
    FloatToDouble(Register, Register),
    DoubleToInt(Register, Register),
    DoubleToLong(Register, Register),
    DoubleToFloat(Register, Register),
    IntToByte(Register, Register),
    IntToChar(Register, Register),
    IntToShort(Register, Register),
    AddInt(Register, Register, Register),
    SubInt(Register, Register, Register),
    MulInt(Register, Register, Register),
    DivInt(Register, Register, Register),
    RemInt(Register, Register, Register),
    AndInt(Register, Register, Register),
    OrInt(Register, Register, Register),
    XorInt(Register, Register, Register),
    ShlInt(Register, Register, Register),
    ShrInt(Register, Register, Register),
    UShrInt(Register, Register, Register),
    AddLong(Register, Register, Register),
    SubLong(Register, Register, Register),
    MulLong(Register, Register, Register),
    DivLong(Register, Register, Register),
    RemLong(Register, Register, Register),
    AndLong(Register, Register, Register),
    OrLong(Register, Register, Register),
    XorLong(Register, Register, Register),
    ShlLong(Register, Register, Register),
    ShrLong(Register, Register, Register),
    UShrLong(Register, Register, Register),
    AddFloat(Register, Register, Register),
    SubFloat(Register, Register, Register),
    MulFloat(Register, Register, Register),
    DivFloat(Register, Register, Register),
    RemFloat(Register, Register, Register),
    AddDouble(Register, Register, Register),
    SubDouble(Register, Register, Register),
    MulDouble(Register, Register, Register),
    DivDouble(Register, Register, Register),
    RemDouble(Register, Register, Register),
    AddInt2Addr(Register, Register),
    SubInt2Addr(Register, Register),
    MulInt2Addr(Register, Register),
    DivInt2Addr(Register, Register),
    RemInt2Addr(Register, Register),
    AndInt2Addr(Register, Register),
    OrInt2Addr(Register, Register),
    XorInt2Addr(Register, Register),
    ShlInt2Addr(Register, Register),
    ShrInt2Addr(Register, Register),
    UShrInt2Addr(Register, Register),
    AddLong2Addr(Register, Register),
    SubLong2Addr(Register, Register),
    MulLong2Addr(Register, Register),
    DivLong2Addr(Register, Register),
    RemLong2Addr(Register, Register),
    AndLong2Addr(Register, Register),
    OrLong2Addr(Register, Register),
    XorLong2Addr(Register, Register),
    ShlLong2Addr(Register, Register),
    ShrLong2Addr(Register, Register),
    UShrLong2Addr(Register, Register),
    AddFloat2Addr(Register, Register),
    SubFloat2Addr(Register, Register),
    MulFloat2Addr(Register, Register),
    DivFloat2Addr(Register, Register),
    RemFloat2Addr(Register, Register),
    AddDouble2Addr(Register, Register),
    SubDouble2Addr(Register, Register),
    MulDouble2Addr(Register, Register),
    DivDouble2Addr(Register, Register),
    RemDouble2Addr(Register, Register),
    AddIntLit16(Register, Register, i32),
    RSubIntLit16(Register, Register, i32),
    MulIntLit16(Register, Register, i32),
    DivIntLit16(Register, Register, i32),
    RemIntLit16(Register, Register, i32),
    AndIntLit16(Register, Register, i32),
    OrIntLit16(Register, Register, i32),
    XorIntLit16(Register, Register, i32),
    AddIntLit8(Register, Register, i32),
    RSubIntLit8(Register, Register, i32),
    MulIntLit8(Register, Register, i32),
    DivIntLit8(Register, Register, i32),
    RemIntLit8(Register, Register, i32),
    AndIntLit8(Register, Register, i32),
    OrIntLit8(Register, Register, i32),
    XorIntLit8(Register, Register, i32),
    ShlIntLit8(Register, Register, i32),
    ShrIntLit8(Register, Register, i32),
    UShrIntLit8(Register, Register, i32),
    InvokePolymorphic,      // TODO 
    InvokePolymorphicRange, // TODO 
    InvokeCustom,           // TODO 
    InvokeCustomRange,      // TODO 
    ConstMethodHandle,      // TODO 
    ConstMethodType,        // TODO 
}

pub fn parse_bytecode(mut bytes: &mut BinaryParser, start: usize, instructions_count: usize) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    bytes.seek_to(start);

    loop {
        if result.len() == instructions_count {
            break;
        }

        let instruction = bytecode_to_instruction(&mut bytes);
        result.push(instruction);
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

fn bytecode_to_instruction(x: &mut BinaryParser) -> Instruction {
    let ins = x.next();
    //println!("INS: {}", to_hex_string(&vec![ins]));
    let res = match ins {
        0x00 => Instruction::Nop,
        0x01 => Instruction::Move(vA1(x), vA2(x)),
        0x02 => Instruction::MoveFrom16(vAA(x), vAAAA(x)),
        0x03 => Instruction::Move16(vAAAA(x), vAAAA(x)),
        0x04 => Instruction::MoveWide(vA1(x), vA2(x)),
        0x05 => Instruction::MoveWideFrom16(vAA(x), vAAAA(x)),
        0x06 => Instruction::MoveWide16(vAAAA(x), vAAAA(x)),
        0x07 => Instruction::MoveObject(vA1(x), vA2(x)),
        0x08 => Instruction::MoveObjectFrom16(vAA(x), vAAAA(x)),
        0x09 => Instruction::MoveObject16(vAAAA(x), vAAAA(x)),
        0x0a => Instruction::MoveResult(vAA(x)),
        0x0b => Instruction::MoveResultWide(vAA(x)),
        0x0c => Instruction::MoveResultObject(vAA(x)),
        0x0d => Instruction::MoveException(vAA(x)),
        0x0e => { x.take(1); Instruction::ReturnVoid},
        0x0f => Instruction::Return(vAA(x)),
        0x10 => Instruction::ReturnWide(vAA(x)),
        0x11 => Instruction::ReturnObject(vAA(x)),
        0x12 => Instruction::Const4(vA1(x), slA(x)),
        0x13 => Instruction::Const16(vAA(x), slAAAA(x)),
        0x14 => Instruction::Const(vAA(x), slAAAAAAAA(x)),
        0x15 => Instruction::ConstHigh16(vAA(x), slAAAA0000(x)),
        0x16 => Instruction::ConstWide16(vAA(x), slAAAA(x) as i64), 
        0x17 => Instruction::ConstWide32(vAA(x), slAAAAAAAA(x) as i64),
        0x18 => Instruction::ConstWide(vAA(x), slAAAAAAAAAAAAAAAA(x)),
        0x19 => Instruction::ConstWideHigh16(vAA(x), slAAAA000000000000(x)),
        0x1a => Instruction::ConstString(vAA(x), stringAAAA(x)),
        0x1b => Instruction::ConstStringJumbo(vAA(x), stringAAAAAAAA(x)),
        0x1c => Instruction::ConstClass(vAA(x), typeAAAA(x)),
        0x1d => Instruction::MonitorEnter(vAA(x)),
        0x1e => Instruction::MonitorExit(vAA(x)),
        0x1f => Instruction::CheckCast(vAA(x), typeAAAA(x)),
        0x20 => Instruction::InstanceOf(vA1(x), vA2(x), typeAAAA(x)),
        0x21 => Instruction::ArrayLength(vA1(x), vA2(x)),
        0x22 => Instruction::NewInstance(vAA(x), typeAAAA(x)),
        0x23 => Instruction::NewArray(vA1(x), vA2(x), typeAAAA(x)),
        0x24 => Instruction::FilledNewArray,
        0x25 => Instruction::FilledNewArrayRange,
        0x26 => Instruction::FillArrayData,
        0x27 => Instruction::Throw(vAA(x)),
        0x28 => Instruction::GoTo(slAA(x)),
        0x29 => Instruction::GoTo16(slAAAA(x)),
        0x2a => Instruction::GoTo32(slAAAAAAAA(x)),
        0x2b => Instruction::PackedSwitch,
        0x2c => Instruction::SpareSwitch,
        0x2d => Instruction::CmpLFloat(vAA(x), vAA(x), vAA(x)),
        0x2e => Instruction::CmpGFloat(vAA(x), vAA(x), vAA(x)),
        0x2f => Instruction::CmpLDouble(vAA(x), vAA(x), vAA(x)),
        0x30 => Instruction::CmpGDouble(vAA(x), vAA(x), vAA(x)),
        0x31 => Instruction::CmpLong(vAA(x), vAA(x), vAA(x)),
        0x32 => Instruction::IfEq(vA1(x), vA2(x), slAAAA(x)),
        0x33 => Instruction::IfNe(vA1(x), vA2(x), slAAAA(x)),
        0x34 => Instruction::IfLt(vA1(x), vA2(x), slAAAA(x)),
        0x35 => Instruction::IfGe(vA1(x), vA2(x), slAAAA(x)),
        0x36 => Instruction::IfGt(vA1(x), vA2(x), slAAAA(x)),
        0x37 => Instruction::IfLe(vA1(x), vA2(x), slAAAA(x)),
        0x38 => Instruction::IfEqZ(vAA(x), slAAAA(x)),
        0x39 => Instruction::IfNeZ(vAA(x), slAAAA(x)),
        0x3a => Instruction::IfLtZ(vAA(x), slAAAA(x)),
        0x3b => Instruction::IfGeZ(vAA(x), slAAAA(x)),
        0x3c => Instruction::IfGtZ(vAA(x), slAAAA(x)),
        0x3d => Instruction::IfLeZ(vAA(x), slAAAA(x)),
        0x44 => Instruction::AGet(vAA(x), vAA(x), vAA(x)),
        0x45 => Instruction::AGetWide(vAA(x), vAA(x), vAA(x)),
        0x46 => Instruction::AGetObject(vAA(x), vAA(x), vAA(x)),
        0x47 => Instruction::AGetBoolean(vAA(x), vAA(x), vAA(x)),
        0x48 => Instruction::AGetByte(vAA(x), vAA(x), vAA(x)),
        0x49 => Instruction::AGetChar(vAA(x), vAA(x), vAA(x)),
        0x4a => Instruction::AGetShort(vAA(x), vAA(x), vAA(x)),
        0x4b => Instruction::APut(vAA(x), vAA(x), vAA(x)),
        0x4c => Instruction::APutWide(vAA(x), vAA(x), vAA(x)),
        0x4d => Instruction::APutObject(vAA(x), vAA(x), vAA(x)),
        0x4e => Instruction::APutBoolean(vAA(x), vAA(x), vAA(x)),
        0x4f => Instruction::APutByte(vAA(x), vAA(x), vAA(x)),
        0x50 => Instruction::APutChar(vAA(x), vAA(x), vAA(x)),
        0x51 => Instruction::APutShort(vAA(x), vAA(x), vAA(x)),
        0x52 => Instruction::IGet(vA1(x), vA2(x), fieldAAAA(x)),
        0x53 => Instruction::IGetWide(vA1(x), vA2(x), fieldAAAA(x)),
        0x54 => Instruction::IGetObject(vA1(x), vA2(x), fieldAAAA(x)),
        0x55 => Instruction::IGetBoolean(vA1(x), vA2(x), fieldAAAA(x)),
        0x56 => Instruction::IGetByte(vA1(x), vA2(x), fieldAAAA(x)),
        0x57 => Instruction::IGetChar(vA1(x), vA2(x), fieldAAAA(x)),
        0x58 => Instruction::IGetShort(vA1(x), vA2(x), fieldAAAA(x)),
        0x59 => Instruction::IPut(vA1(x), vA2(x), fieldAAAA(x)),
        0x5a => Instruction::IPutWide(vA1(x), vA2(x), fieldAAAA(x)),
        0x5b => Instruction::IPutObject(vA1(x), vA2(x), fieldAAAA(x)),
        0x5c => Instruction::IPutBoolean(vA1(x), vA2(x), fieldAAAA(x)),
        0x5d => Instruction::IPutByte(vA1(x), vA2(x), fieldAAAA(x)),
        0x5e => Instruction::IPutChar(vA1(x), vA2(x), fieldAAAA(x)),
        0x5f => Instruction::IPutShort(vA1(x), vA2(x), fieldAAAA(x)),
        0x60 => Instruction::SGet(vAA(x), fieldAAAA(x)),
        0x61 => Instruction::SGetWide(vAA(x), fieldAAAA(x)),
        0x62 => Instruction::SGetObject(vAA(x), fieldAAAA(x)),
        0x63 => Instruction::SGetBoolean(vAA(x), fieldAAAA(x)),
        0x64 => Instruction::SGetByte(vAA(x), fieldAAAA(x)),
        0x65 => Instruction::SGetChar(vAA(x), fieldAAAA(x)),
        0x66 => Instruction::SGetShort(vAA(x), fieldAAAA(x)),
        0x67 => Instruction::SPut(vAA(x), fieldAAAA(x)),
        0x68 => Instruction::SPutWide(vAA(x), fieldAAAA(x)),
        0x69 => Instruction::SPutObject(vAA(x), fieldAAAA(x)),
        0x6a => Instruction::SPutBoolean(vAA(x), fieldAAAA(x)),
        0x6b => Instruction::SPutByte(vAA(x), fieldAAAA(x)),
        0x6c => Instruction::SPutChar(vAA(x), fieldAAAA(x)),
        0x6d => Instruction::SPutShort(vAA(x), fieldAAAA(x)),
        0x6e => { let (args, method) = args(x); Instruction::InvokeVirtual(args, method) }
        0x6f => { let (args, method) = args(x); Instruction::InvokeSuper(args, method) }
        0x70 => { let (args, method) = args(x); Instruction::InvokeDirect(args, method) }
        0x71 => { let (args, method) = args(x); Instruction::InvokeStatic(args, method) }
        0x72 => { let (args, method) = args(x); Instruction::InvokeInterface(args, method) }
        0x73 => Instruction::InvokeVirtualRange,
        0x74 => Instruction::InvokeSuperRange,
        0x75 => Instruction::InvokeDirectRange,
        0x76 => Instruction::InvokeStaticRange,
        0x77 => Instruction::InvokeInterfaceRange,
        0x7b => Instruction::NegInt(vA1(x), vA2(x)),
        0x7c => Instruction::NotInt(vA1(x), vA2(x)),
        0x7d => Instruction::NegLong(vA1(x), vA2(x)),
        0x7e => Instruction::NotLong(vA1(x), vA2(x)),
        0x7f => Instruction::NegFloat(vA1(x), vA2(x)),
        0x80 => Instruction::NegDouble(vA1(x), vA2(x)),
        0x81 => Instruction::IntToLong(vA1(x), vA2(x)),
        0x82 => Instruction::IntToFloat(vA1(x), vA2(x)),
        0x83 => Instruction::IntToDouble(vA1(x), vA2(x)),
        0x84 => Instruction::LongToInt(vA1(x), vA2(x)),
        0x85 => Instruction::LongToFloat(vA1(x), vA2(x)),
        0x86 => Instruction::LongToDouble(vA1(x), vA2(x)),
        0x87 => Instruction::FloatToInt(vA1(x), vA2(x)),
        0x88 => Instruction::FloatToLong(vA1(x), vA2(x)),
        0x89 => Instruction::FloatToDouble(vA1(x), vA2(x)),
        0x8a => Instruction::DoubleToInt(vA1(x), vA2(x)),
        0x8b => Instruction::DoubleToLong(vA1(x), vA2(x)),
        0x8c => Instruction::DoubleToFloat(vA1(x), vA2(x)),
        0x8d => Instruction::IntToByte(vA1(x), vA2(x)),
        0x8e => Instruction::IntToChar(vA1(x), vA2(x)),
        0x8f => Instruction::IntToShort(vA1(x), vA2(x)),
        0x90 => Instruction::AddInt(vAA(x), vAA(x), vAA(x)),
        0x91 => Instruction::SubInt(vAA(x), vAA(x), vAA(x)),
        0x92 => Instruction::MulInt(vAA(x), vAA(x), vAA(x)),
        0x93 => Instruction::DivInt(vAA(x), vAA(x), vAA(x)),
        0x94 => Instruction::RemInt(vAA(x), vAA(x), vAA(x)),
        0x95 => Instruction::AndInt(vAA(x), vAA(x), vAA(x)),
        0x96 => Instruction::OrInt(vAA(x), vAA(x), vAA(x)),
        0x97 => Instruction::XorInt(vAA(x), vAA(x), vAA(x)),
        0x98 => Instruction::ShlInt(vAA(x), vAA(x), vAA(x)),
        0x99 => Instruction::ShrInt(vAA(x), vAA(x), vAA(x)),
        0x9a => Instruction::UShrInt(vAA(x), vAA(x), vAA(x)),
        0x9b => Instruction::AddLong(vAA(x), vAA(x), vAA(x)),
        0x9c => Instruction::SubLong(vAA(x), vAA(x), vAA(x)),
        0x9d => Instruction::MulLong(vAA(x), vAA(x), vAA(x)),
        0x9e => Instruction::DivLong(vAA(x), vAA(x), vAA(x)),
        0x9f => Instruction::RemLong(vAA(x), vAA(x), vAA(x)),
        0xa0 => Instruction::AndLong(vAA(x), vAA(x), vAA(x)),
        0xa1 => Instruction::OrLong(vAA(x), vAA(x), vAA(x)),
        0xa2 => Instruction::XorLong(vAA(x), vAA(x), vAA(x)),
        0xa3 => Instruction::ShlLong(vAA(x), vAA(x), vAA(x)),
        0xa4 => Instruction::ShrLong(vAA(x), vAA(x), vAA(x)),
        0xa5 => Instruction::UShrLong(vAA(x), vAA(x), vAA(x)),
        0xa6 => Instruction::AddFloat(vAA(x), vAA(x), vAA(x)),
        0xa7 => Instruction::SubFloat(vAA(x), vAA(x), vAA(x)),
        0xa8 => Instruction::MulFloat(vAA(x), vAA(x), vAA(x)),
        0xa9 => Instruction::DivFloat(vAA(x), vAA(x), vAA(x)),
        0xaa => Instruction::RemFloat(vAA(x), vAA(x), vAA(x)),
        0xab => Instruction::AddDouble(vAA(x), vAA(x), vAA(x)),
        0xac => Instruction::SubDouble(vAA(x), vAA(x), vAA(x)),
        0xad => Instruction::MulDouble(vAA(x), vAA(x), vAA(x)),
        0xae => Instruction::DivDouble(vAA(x), vAA(x), vAA(x)),
        0xaf => Instruction::RemDouble(vAA(x), vAA(x), vAA(x)),
        0xb0 => Instruction::AddInt2Addr(vA1(x), vA2(x)),
        0xb1 => Instruction::SubInt2Addr(vA1(x), vA2(x)),
        0xb2 => Instruction::MulInt2Addr(vA1(x), vA2(x)),
        0xb3 => Instruction::DivInt2Addr(vA1(x), vA2(x)),
        0xb4 => Instruction::RemInt2Addr(vA1(x), vA2(x)),
        0xb5 => Instruction::AndInt2Addr(vA1(x), vA2(x)),
        0xb6 => Instruction::OrInt2Addr(vA1(x), vA2(x)),
        0xb7 => Instruction::XorInt2Addr(vA1(x), vA2(x)),
        0xb8 => Instruction::ShlInt2Addr(vA1(x), vA2(x)),
        0xb9 => Instruction::ShrInt2Addr(vA1(x), vA2(x)),
        0xba => Instruction::UShrInt2Addr(vA1(x), vA2(x)),
        0xbb => Instruction::AddLong2Addr(vA1(x), vA2(x)),
        0xbc => Instruction::SubLong2Addr(vA1(x), vA2(x)),
        0xbd => Instruction::MulLong2Addr(vA1(x), vA2(x)),
        0xbe => Instruction::DivLong2Addr(vA1(x), vA2(x)),
        0xbf => Instruction::RemLong2Addr(vA1(x), vA2(x)),
        0xc0 => Instruction::AndLong2Addr(vA1(x), vA2(x)),
        0xc1 => Instruction::OrLong2Addr(vA1(x), vA2(x)),
        0xc2 => Instruction::XorLong2Addr(vA1(x), vA2(x)),
        0xc3 => Instruction::ShlLong2Addr(vA1(x), vA2(x)),
        0xc4 => Instruction::ShrLong2Addr(vA1(x), vA2(x)),
        0xc5 => Instruction::UShrLong2Addr(vA1(x), vA2(x)),
        0xc6 => Instruction::AddFloat2Addr(vA1(x), vA2(x)),
        0xc7 => Instruction::SubFloat2Addr(vA1(x), vA2(x)),
        0xc8 => Instruction::MulFloat2Addr(vA1(x), vA2(x)),
        0xc9 => Instruction::DivFloat2Addr(vA1(x), vA2(x)),
        0xca => Instruction::RemFloat2Addr(vA1(x), vA2(x)),
        0xcb => Instruction::AddDouble2Addr(vA1(x), vA2(x)),
        0xcc => Instruction::SubDouble2Addr(vA1(x), vA2(x)),
        0xcd => Instruction::MulDouble2Addr(vA1(x), vA2(x)),
        0xce => Instruction::DivDouble2Addr(vA1(x), vA2(x)),
        0xcf => Instruction::RemDouble2Addr(vA1(x), vA2(x)),
        0xd0 => Instruction::AddIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd1 => Instruction::RSubIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd2 => Instruction::MulIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd3 => Instruction::DivIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd4 => Instruction::RemIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd5 => Instruction::AndIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd6 => Instruction::OrIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd7 => Instruction::XorIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd8 => Instruction::AddIntLit8(vAA(x), vAA(x), slAA(x)),
        0xd9 => Instruction::RSubIntLit8(vAA(x), vAA(x), slAA(x)),
        0xda => Instruction::MulIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdb => Instruction::DivIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdc => Instruction::RemIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdd => Instruction::AndIntLit8(vAA(x), vAA(x), slAA(x)),
        0xde => Instruction::OrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdf => Instruction::XorIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe0 => Instruction::ShlIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe1 => Instruction::ShrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe2 => Instruction::UShrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xfa => Instruction::InvokePolymorphic,
        0xfb => Instruction::InvokePolymorphicRange,
        0xfc => Instruction::InvokeCustom,
        0xfd => Instruction::InvokeCustomRange,
        0xfe => Instruction::ConstMethodHandle,
        0xff => Instruction::ConstMethodType,
        _ => Instruction::Nop,
    };
    //println!("{:?}", res);
    return res;
}

fn vA1(v: &mut BinaryParser) -> Register {
    (v.peek(1)[0] & 0b00001111) as Register
}

fn vA2(v: &mut BinaryParser) -> Register {
    (v.next() >> 4 & 0b00001111) as Register
}

fn vAA(v: &mut BinaryParser) -> Register {
    v.next() as Register
}

fn vAAAA(v: &mut BinaryParser) -> Register {
    let x = v.take(2);
    to_decimal_short(&x) as Register
}

fn slA(v: &mut BinaryParser) -> i32 {
    (v.next() & 0b11110000) as i32
}

fn slAA(v: &mut BinaryParser) -> i32 {
    v.next() as i32
}

fn slAAAA(v: &mut BinaryParser) -> i32 {
    let x = v.take(2);
    to_decimal_short(&x) as i32
}

fn slAAAAAAAA(v: &mut BinaryParser) -> i32 {
    let x = v.take(4);
    to_decimal(&x) as i32
}

fn slAAAA0000(v: &mut BinaryParser) -> i32 {
    0 // TODO
}

fn slAAAAAAAAAAAAAAAA(v: &mut BinaryParser) -> i64 {
    0 // TODO
}

fn slAAAA000000000000(v: &mut BinaryParser) -> i64 {
    0 // TODO
}

fn stringAAAA(v: &mut BinaryParser) -> StringIndex {
    let x = v.take(2);
    to_decimal_short(&x) as StringIndex
}

fn stringAAAAAAAA(v: &mut BinaryParser) -> StringIndex {
    0 as StringIndex // TODO
}

fn typeAAAA(v: &mut BinaryParser) -> TypeIndex {
    let x = v.take(2);
    to_decimal_short(&x) as TypeIndex
}

fn fieldAAAA(v: &mut BinaryParser) -> FieldIndex {
    let x = v.take(2);
    to_decimal_short(&x) as FieldIndex
}

fn methodAAAA(v: &mut BinaryParser) -> MethodIndex {
    let x = v.take(2);
    to_decimal_short(&x) as MethodIndex
}

fn args(v: &mut BinaryParser) -> (Vec<Register>, MethodIndex) {
    let first_byte = v.next();
    let addr = v.take(2);

    let mut args: Vec<Register> = Vec::new();
    let arg_count = first_byte >> 4 & 0b00001111;

    let mut arg_bytes = v.take(2);
    if arg_count > 0 {
        for i in 0..(arg_count - 1) {
            let b = arg_bytes[0];
            if i % 2 == 0 {
                args.push((b & 0b00001111) as Register);
            } else {
                args.push((b >> 4 & 0b00001111) as Register);
                arg_bytes.drain(0..1);
            }
        }
    }

    if arg_count == 1 {
        let b = arg_bytes[0];
        args.push((b & 0b00001111) as Register);
    }

    if arg_count == 5 {
        let last_arg = first_byte & 0b00001111;
        args.push(last_arg as Register);
    }

    return (args, to_decimal_short(&addr) as MethodIndex);
}


#[test]
pub fn test_stuff() {
    let mut bytecode = vec![0x01, 0x01];
    let mut parser = BinaryParser::new(bytecode);

    let instruction = bytecode_to_instruction(&mut parser);
    match instruction {
        Instruction::Move(a, b) => {
            assert_eq!(a, 1);
            assert_eq!(b, 0);
        },
        _ => panic!()
    }
}

#[test]
pub fn test_args_arity_one() {
    let mut bytecode = vec![0x10, 0xff, 0xff, 0x04, 0x00];
    let mut parser = BinaryParser::new(bytecode);

    let (args, i) = args(&mut parser);
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], 4);
}

#[test]
pub fn test_args_arity_two() {
    let mut bytecode = vec![0x5f, 0x2c, 0x00, 0xb0, 0x5f];
    let mut parser = BinaryParser::new(bytecode);

    let (args, i) = args(&mut parser);
    assert_eq!(args.len(), 5);
    assert_eq!(args, vec![0, 11, 15, 5, 15]);
}


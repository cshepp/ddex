
use crate::binary_parser::BinaryParser;
use crate::dex_types::*;
use crate::util::{to_decimal, to_decimal_short, to_hex_string, to_i8, to_i16};

pub type Register = u32;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub addr: usize,
    pub kind: InstructionKind,
    pub bytecode: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionKind {
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
    FilledNewArray(Vec<Register>, TypeIndex),
    FilledNewArrayRange(Register, Register, TypeIndex),
    FillArrayData(Register, i32),
    Throw(Register),
    GoTo(i32),
    GoTo16(i32),
    GoTo32(i32),
    PackedSwitch(Register, i32),
    SparseSwitch(Register, i32),
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
    InvokeVirtualRange(Register, Register, MethodIndex), 
    InvokeSuperRange(Register, Register, MethodIndex),
    InvokeDirectRange(Register, Register, MethodIndex),
    InvokeStaticRange(Register, Register, MethodIndex),
    InvokeInterfaceRange(Register, Register, MethodIndex),
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
    Unused,
    Stop,
}

pub fn parse_bytecode(mut bytes: &mut BinaryParser, start: usize, instructions_count: usize) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    bytes.seek_to(start);

    loop {
        if bytes.current_location() >= start + instructions_count * 2 {
            break;
        }

        let addr = bytes.current_location();
        match bytecode_to_instruction_kind(&mut bytes) {
            Some(kind) => {

                let end_addr = bytes.current_location();
                let diff = end_addr - addr;

                bytes.seek_to(addr);
                let bytecode = bytes.take(diff);

                result.push(Instruction{
                    addr,
                    kind,
                    bytecode,
                });
            }
            None => break
        }
    }

    return result;
}

fn bytecode_to_instruction_kind(x: &mut BinaryParser) -> Option<InstructionKind> {
    let ins = x.next();
    let res: InstructionKind = match ins {
        0x00 => { 
            match x.next() {
                0x00 => InstructionKind::Nop,
                0x01 => {
                    let size = slAA(x);
                    let payload = size * 2 + 4;
                    //x.take(payload as usize);
                    return None;
                }
                0x02 => {
                    let size = slAA(x);
                    let payload = size * 4 + 2;
                    //x.take(payload as usize);
                    return None;
                }
                0x03 => {
                    // fill-array-data-payload
                    let u = x.next() as u32;
                    let v = x.next() as u32;
                    let elem_width = u | (v << 8);

                    let (a, b, c, d) = (x.next() as u32, x.next() as u32, x.next() as u32, x.next() as u32);
                    let array_size = a | (b << 8) | (c << 16) | (d << 24);
                    //println!("{} {}", elem_width, array_size);
                    let payload = (array_size as i64 * elem_width as i64) / 4;
                    //println!("fill array payload of {} bytes", payload);
                    //x.take(payload as usize);

                    if x.current_location() - 1 == 644995 {
                        panic!("");
                    }
                    return None;
                }
                _ => InstructionKind::Nop
            }
        },
        0x01 => InstructionKind::Move(vA1(x), vA2(x)),
        0x02 => InstructionKind::MoveFrom16(vAA(x), vAAAA(x)),
        0x03 => { x.take(1); InstructionKind::Move16(vAAAA(x), vAAAA(x))},
        0x04 => InstructionKind::MoveWide(vA1(x), vA2(x)),
        0x05 => InstructionKind::MoveWideFrom16(vAA(x), vAAAA(x)),
        0x06 => { x.take(1); InstructionKind::MoveWide16(vAAAA(x), vAAAA(x))},
        0x07 => InstructionKind::MoveObject(vA1(x), vA2(x)),
        0x08 => InstructionKind::MoveObjectFrom16(vAA(x), vAAAA(x)),
        0x09 => {x.take(1); InstructionKind::MoveObject16(vAAAA(x), vAAAA(x))},
        0x0a => InstructionKind::MoveResult(vAA(x)),
        0x0b => InstructionKind::MoveResultWide(vAA(x)),
        0x0c => InstructionKind::MoveResultObject(vAA(x)),
        0x0d => InstructionKind::MoveException(vAA(x)),
        0x0e => { x.take(1); InstructionKind::ReturnVoid},
        0x0f => InstructionKind::Return(vAA(x)),
        0x10 => InstructionKind::ReturnWide(vAA(x)),
        0x11 => InstructionKind::ReturnObject(vAA(x)),
        0x12 => InstructionKind::Const4(vA1(x), slA(x)),
        0x13 => InstructionKind::Const16(vAA(x), slAAAA(x)),
        0x14 => InstructionKind::Const(vAA(x), slAAAAAAAA(x)),
        0x15 => InstructionKind::ConstHigh16(vAA(x), slAAAA0000(x)),
        0x16 => InstructionKind::ConstWide16(vAA(x), slAAAA(x) as i64), 
        0x17 => InstructionKind::ConstWide32(vAA(x), slAAAAAAAA(x) as i64),
        0x18 => InstructionKind::ConstWide(vAA(x), slAAAAAAAAAAAAAAAA(x)),
        0x19 => InstructionKind::ConstWideHigh16(vAA(x), slAAAA000000000000(x)),
        0x1a => InstructionKind::ConstString(vAA(x), stringAAAA(x)),
        0x1b => InstructionKind::ConstStringJumbo(vAA(x), stringAAAAAAAA(x)),
        0x1c => InstructionKind::ConstClass(vAA(x), typeAAAA(x)),
        0x1d => InstructionKind::MonitorEnter(vAA(x)),
        0x1e => InstructionKind::MonitorExit(vAA(x)),
        0x1f => InstructionKind::CheckCast(vAA(x), typeAAAA(x)),
        0x20 => InstructionKind::InstanceOf(vA1(x), vA2(x), typeAAAA(x)),
        0x21 => InstructionKind::ArrayLength(vA1(x), vA2(x)),
        0x22 => InstructionKind::NewInstance(vAA(x), typeAAAA(x)),
        0x23 => InstructionKind::NewArray(vA1(x), vA2(x), typeAAAA(x)),
        0x24 => { let (args, t) = invoke_kind(x); InstructionKind::FilledNewArray(args, t as TypeIndex) },
        0x25 => { let (r1, r2, t) = invoke_kind_range(x); InstructionKind::FilledNewArrayRange(r1, r2, t as TypeIndex) }
        0x26 => InstructionKind::FillArrayData(vAA(x), slAAAAAAAA(x)),
        0x27 => InstructionKind::Throw(vAA(x)),
        0x28 => {
            let mut offset = slAA(x);
            if offset == 0 { offset = 1; }
            InstructionKind::GoTo(offset * 2)
        },
        0x29 => {x.take(1); InstructionKind::GoTo16((slAAAA(x) as i64 * 2) as i32)},
        0x2a => {x.take(1); InstructionKind::GoTo32((slAAAAAAAA(x) as i64 * 2) as i32)},
        0x2b => InstructionKind::PackedSwitch(vAA(x), slAAAAAAAA(x)),
        0x2c => InstructionKind::SparseSwitch(vAA(x), slAAAAAAAA(x)),
        0x2d => InstructionKind::CmpLFloat(vAA(x), vAA(x), vAA(x)),
        0x2e => InstructionKind::CmpGFloat(vAA(x), vAA(x), vAA(x)),
        0x2f => InstructionKind::CmpLDouble(vAA(x), vAA(x), vAA(x)),
        0x30 => InstructionKind::CmpGDouble(vAA(x), vAA(x), vAA(x)),
        0x31 => InstructionKind::CmpLong(vAA(x), vAA(x), vAA(x)),
        0x32 => InstructionKind::IfEq(vA1(x), vA2(x), slAAAA(x) * 2),
        0x33 => InstructionKind::IfNe(vA1(x), vA2(x), slAAAA(x) * 2),
        0x34 => InstructionKind::IfLt(vA1(x), vA2(x), slAAAA(x) * 2),
        0x35 => InstructionKind::IfGe(vA1(x), vA2(x), slAAAA(x) * 2),
        0x36 => InstructionKind::IfGt(vA1(x), vA2(x), slAAAA(x) * 2),
        0x37 => InstructionKind::IfLe(vA1(x), vA2(x), slAAAA(x) * 2),
        0x38 => InstructionKind::IfEqZ(vAA(x), slAAAA(x) * 2),
        0x39 => InstructionKind::IfNeZ(vAA(x), slAAAA(x) * 2),
        0x3a => InstructionKind::IfLtZ(vAA(x), slAAAA(x) * 2),
        0x3b => InstructionKind::IfGeZ(vAA(x), slAAAA(x) * 2),
        0x3c => InstructionKind::IfGtZ(vAA(x), slAAAA(x) * 2),
        0x3d => InstructionKind::IfLeZ(vAA(x), slAAAA(x) * 2),
        0x44 => InstructionKind::AGet(vAA(x), vAA(x), vAA(x)),
        0x45 => InstructionKind::AGetWide(vAA(x), vAA(x), vAA(x)),
        0x46 => InstructionKind::AGetObject(vAA(x), vAA(x), vAA(x)),
        0x47 => InstructionKind::AGetBoolean(vAA(x), vAA(x), vAA(x)),
        0x48 => InstructionKind::AGetByte(vAA(x), vAA(x), vAA(x)),
        0x49 => InstructionKind::AGetChar(vAA(x), vAA(x), vAA(x)),
        0x4a => InstructionKind::AGetShort(vAA(x), vAA(x), vAA(x)),
        0x4b => InstructionKind::APut(vAA(x), vAA(x), vAA(x)),
        0x4c => InstructionKind::APutWide(vAA(x), vAA(x), vAA(x)),
        0x4d => InstructionKind::APutObject(vAA(x), vAA(x), vAA(x)),
        0x4e => InstructionKind::APutBoolean(vAA(x), vAA(x), vAA(x)),
        0x4f => InstructionKind::APutByte(vAA(x), vAA(x), vAA(x)),
        0x50 => InstructionKind::APutChar(vAA(x), vAA(x), vAA(x)),
        0x51 => InstructionKind::APutShort(vAA(x), vAA(x), vAA(x)),
        0x52 => InstructionKind::IGet(vA1(x), vA2(x), fieldAAAA(x)),
        0x53 => InstructionKind::IGetWide(vA1(x), vA2(x), fieldAAAA(x)),
        0x54 => InstructionKind::IGetObject(vA1(x), vA2(x), fieldAAAA(x)),
        0x55 => InstructionKind::IGetBoolean(vA1(x), vA2(x), fieldAAAA(x)),
        0x56 => InstructionKind::IGetByte(vA1(x), vA2(x), fieldAAAA(x)),
        0x57 => InstructionKind::IGetChar(vA1(x), vA2(x), fieldAAAA(x)),
        0x58 => InstructionKind::IGetShort(vA1(x), vA2(x), fieldAAAA(x)),
        0x59 => InstructionKind::IPut(vA1(x), vA2(x), fieldAAAA(x)),
        0x5a => InstructionKind::IPutWide(vA1(x), vA2(x), fieldAAAA(x)),
        0x5b => InstructionKind::IPutObject(vA1(x), vA2(x), fieldAAAA(x)),
        0x5c => InstructionKind::IPutBoolean(vA1(x), vA2(x), fieldAAAA(x)),
        0x5d => InstructionKind::IPutByte(vA1(x), vA2(x), fieldAAAA(x)),
        0x5e => InstructionKind::IPutChar(vA1(x), vA2(x), fieldAAAA(x)),
        0x5f => InstructionKind::IPutShort(vA1(x), vA2(x), fieldAAAA(x)),
        0x60 => InstructionKind::SGet(vAA(x), fieldAAAA(x)),
        0x61 => InstructionKind::SGetWide(vAA(x), fieldAAAA(x)),
        0x62 => InstructionKind::SGetObject(vAA(x), fieldAAAA(x)),
        0x63 => InstructionKind::SGetBoolean(vAA(x), fieldAAAA(x)),
        0x64 => InstructionKind::SGetByte(vAA(x), fieldAAAA(x)),
        0x65 => InstructionKind::SGetChar(vAA(x), fieldAAAA(x)),
        0x66 => InstructionKind::SGetShort(vAA(x), fieldAAAA(x)),
        0x67 => InstructionKind::SPut(vAA(x), fieldAAAA(x)),
        0x68 => InstructionKind::SPutWide(vAA(x), fieldAAAA(x)),
        0x69 => InstructionKind::SPutObject(vAA(x), fieldAAAA(x)),
        0x6a => InstructionKind::SPutBoolean(vAA(x), fieldAAAA(x)),
        0x6b => InstructionKind::SPutByte(vAA(x), fieldAAAA(x)),
        0x6c => InstructionKind::SPutChar(vAA(x), fieldAAAA(x)),
        0x6d => InstructionKind::SPutShort(vAA(x), fieldAAAA(x)),
        0x6e => { let (args, method) = invoke_kind(x); InstructionKind::InvokeVirtual(args, method) }
        0x6f => { let (args, method) = invoke_kind(x); InstructionKind::InvokeSuper(args, method) }
        0x70 => { let (args, method) = invoke_kind(x); InstructionKind::InvokeDirect(args, method) }
        0x71 => { let (args, method) = invoke_kind(x); InstructionKind::InvokeStatic(args, method) }
        0x72 => { let (args, method) = invoke_kind(x); InstructionKind::InvokeInterface(args, method) }
        0x74 => { let (r1, r2, method) = invoke_kind_range(x); InstructionKind::InvokeVirtualRange(r1, r2, method) }
        0x75 => { let (r1, r2, method) = invoke_kind_range(x); InstructionKind::InvokeSuperRange(r1, r2, method) }
        0x76 => { let (r1, r2, method) = invoke_kind_range(x); InstructionKind::InvokeDirectRange(r1, r2, method) }
        0x77 => { let (r1, r2, method) = invoke_kind_range(x); InstructionKind::InvokeStaticRange(r1, r2, method) }
        0x78 => { let (r1, r2, method) = invoke_kind_range(x); InstructionKind::InvokeInterfaceRange(r1, r2, method) }
        0x7b => InstructionKind::NegInt(vA1(x), vA2(x)),
        0x7c => InstructionKind::NotInt(vA1(x), vA2(x)),
        0x7d => InstructionKind::NegLong(vA1(x), vA2(x)),
        0x7e => InstructionKind::NotLong(vA1(x), vA2(x)),
        0x7f => InstructionKind::NegFloat(vA1(x), vA2(x)),
        0x80 => InstructionKind::NegDouble(vA1(x), vA2(x)),
        0x81 => InstructionKind::IntToLong(vA1(x), vA2(x)),
        0x82 => InstructionKind::IntToFloat(vA1(x), vA2(x)),
        0x83 => InstructionKind::IntToDouble(vA1(x), vA2(x)),
        0x84 => InstructionKind::LongToInt(vA1(x), vA2(x)),
        0x85 => InstructionKind::LongToFloat(vA1(x), vA2(x)),
        0x86 => InstructionKind::LongToDouble(vA1(x), vA2(x)),
        0x87 => InstructionKind::FloatToInt(vA1(x), vA2(x)),
        0x88 => InstructionKind::FloatToLong(vA1(x), vA2(x)),
        0x89 => InstructionKind::FloatToDouble(vA1(x), vA2(x)),
        0x8a => InstructionKind::DoubleToInt(vA1(x), vA2(x)),
        0x8b => InstructionKind::DoubleToLong(vA1(x), vA2(x)),
        0x8c => InstructionKind::DoubleToFloat(vA1(x), vA2(x)),
        0x8d => InstructionKind::IntToByte(vA1(x), vA2(x)),
        0x8e => InstructionKind::IntToChar(vA1(x), vA2(x)),
        0x8f => InstructionKind::IntToShort(vA1(x), vA2(x)),
        0x90 => InstructionKind::AddInt(vAA(x), vAA(x), vAA(x)),
        0x91 => InstructionKind::SubInt(vAA(x), vAA(x), vAA(x)),
        0x92 => InstructionKind::MulInt(vAA(x), vAA(x), vAA(x)),
        0x93 => InstructionKind::DivInt(vAA(x), vAA(x), vAA(x)),
        0x94 => InstructionKind::RemInt(vAA(x), vAA(x), vAA(x)),
        0x95 => InstructionKind::AndInt(vAA(x), vAA(x), vAA(x)),
        0x96 => InstructionKind::OrInt(vAA(x), vAA(x), vAA(x)),
        0x97 => InstructionKind::XorInt(vAA(x), vAA(x), vAA(x)),
        0x98 => InstructionKind::ShlInt(vAA(x), vAA(x), vAA(x)),
        0x99 => InstructionKind::ShrInt(vAA(x), vAA(x), vAA(x)),
        0x9a => InstructionKind::UShrInt(vAA(x), vAA(x), vAA(x)),
        0x9b => InstructionKind::AddLong(vAA(x), vAA(x), vAA(x)),
        0x9c => InstructionKind::SubLong(vAA(x), vAA(x), vAA(x)),
        0x9d => InstructionKind::MulLong(vAA(x), vAA(x), vAA(x)),
        0x9e => InstructionKind::DivLong(vAA(x), vAA(x), vAA(x)),
        0x9f => InstructionKind::RemLong(vAA(x), vAA(x), vAA(x)),
        0xa0 => InstructionKind::AndLong(vAA(x), vAA(x), vAA(x)),
        0xa1 => InstructionKind::OrLong(vAA(x), vAA(x), vAA(x)),
        0xa2 => InstructionKind::XorLong(vAA(x), vAA(x), vAA(x)),
        0xa3 => InstructionKind::ShlLong(vAA(x), vAA(x), vAA(x)),
        0xa4 => InstructionKind::ShrLong(vAA(x), vAA(x), vAA(x)),
        0xa5 => InstructionKind::UShrLong(vAA(x), vAA(x), vAA(x)),
        0xa6 => InstructionKind::AddFloat(vAA(x), vAA(x), vAA(x)),
        0xa7 => InstructionKind::SubFloat(vAA(x), vAA(x), vAA(x)),
        0xa8 => InstructionKind::MulFloat(vAA(x), vAA(x), vAA(x)),
        0xa9 => InstructionKind::DivFloat(vAA(x), vAA(x), vAA(x)),
        0xaa => InstructionKind::RemFloat(vAA(x), vAA(x), vAA(x)),
        0xab => InstructionKind::AddDouble(vAA(x), vAA(x), vAA(x)),
        0xac => InstructionKind::SubDouble(vAA(x), vAA(x), vAA(x)),
        0xad => InstructionKind::MulDouble(vAA(x), vAA(x), vAA(x)),
        0xae => InstructionKind::DivDouble(vAA(x), vAA(x), vAA(x)),
        0xaf => InstructionKind::RemDouble(vAA(x), vAA(x), vAA(x)),
        0xb0 => InstructionKind::AddInt2Addr(vA1(x), vA2(x)),
        0xb1 => InstructionKind::SubInt2Addr(vA1(x), vA2(x)),
        0xb2 => InstructionKind::MulInt2Addr(vA1(x), vA2(x)),
        0xb3 => InstructionKind::DivInt2Addr(vA1(x), vA2(x)),
        0xb4 => InstructionKind::RemInt2Addr(vA1(x), vA2(x)),
        0xb5 => InstructionKind::AndInt2Addr(vA1(x), vA2(x)),
        0xb6 => InstructionKind::OrInt2Addr(vA1(x), vA2(x)),
        0xb7 => InstructionKind::XorInt2Addr(vA1(x), vA2(x)),
        0xb8 => InstructionKind::ShlInt2Addr(vA1(x), vA2(x)),
        0xb9 => InstructionKind::ShrInt2Addr(vA1(x), vA2(x)),
        0xba => InstructionKind::UShrInt2Addr(vA1(x), vA2(x)),
        0xbb => InstructionKind::AddLong2Addr(vA1(x), vA2(x)),
        0xbc => InstructionKind::SubLong2Addr(vA1(x), vA2(x)),
        0xbd => InstructionKind::MulLong2Addr(vA1(x), vA2(x)),
        0xbe => InstructionKind::DivLong2Addr(vA1(x), vA2(x)),
        0xbf => InstructionKind::RemLong2Addr(vA1(x), vA2(x)),
        0xc0 => InstructionKind::AndLong2Addr(vA1(x), vA2(x)),
        0xc1 => InstructionKind::OrLong2Addr(vA1(x), vA2(x)),
        0xc2 => InstructionKind::XorLong2Addr(vA1(x), vA2(x)),
        0xc3 => InstructionKind::ShlLong2Addr(vA1(x), vA2(x)),
        0xc4 => InstructionKind::ShrLong2Addr(vA1(x), vA2(x)),
        0xc5 => InstructionKind::UShrLong2Addr(vA1(x), vA2(x)),
        0xc6 => InstructionKind::AddFloat2Addr(vA1(x), vA2(x)),
        0xc7 => InstructionKind::SubFloat2Addr(vA1(x), vA2(x)),
        0xc8 => InstructionKind::MulFloat2Addr(vA1(x), vA2(x)),
        0xc9 => InstructionKind::DivFloat2Addr(vA1(x), vA2(x)),
        0xca => InstructionKind::RemFloat2Addr(vA1(x), vA2(x)),
        0xcb => InstructionKind::AddDouble2Addr(vA1(x), vA2(x)),
        0xcc => InstructionKind::SubDouble2Addr(vA1(x), vA2(x)),
        0xcd => InstructionKind::MulDouble2Addr(vA1(x), vA2(x)),
        0xce => InstructionKind::DivDouble2Addr(vA1(x), vA2(x)),
        0xcf => InstructionKind::RemDouble2Addr(vA1(x), vA2(x)),
        0xd0 => InstructionKind::AddIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd1 => InstructionKind::RSubIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd2 => InstructionKind::MulIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd3 => InstructionKind::DivIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd4 => InstructionKind::RemIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd5 => InstructionKind::AndIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd6 => InstructionKind::OrIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd7 => InstructionKind::XorIntLit16(vA1(x), vA2(x), slAAAA(x)),
        0xd8 => InstructionKind::AddIntLit8(vAA(x), vAA(x), slAA(x)),
        0xd9 => InstructionKind::RSubIntLit8(vAA(x), vAA(x), slAA(x)),
        0xda => InstructionKind::MulIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdb => InstructionKind::DivIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdc => InstructionKind::RemIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdd => InstructionKind::AndIntLit8(vAA(x), vAA(x), slAA(x)),
        0xde => InstructionKind::OrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xdf => InstructionKind::XorIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe0 => InstructionKind::ShlIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe1 => InstructionKind::ShrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xe2 => InstructionKind::UShrIntLit8(vAA(x), vAA(x), slAA(x)),
        0xfa => InstructionKind::InvokePolymorphic,
        0xfb => InstructionKind::InvokePolymorphicRange,
        0xfc => InstructionKind::InvokeCustom,
        0xfd => InstructionKind::InvokeCustomRange,
        0xfe => InstructionKind::ConstMethodHandle,
        0xff => InstructionKind::ConstMethodType,
        _ => {x.take(1); InstructionKind::Unused},
    };
    return Some(res);
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
    let x = v.take(1);
    to_i8(&x) as i32
}

fn slAAAA(v: &mut BinaryParser) -> i32 {
    let x = v.take(2);
    to_i16(&x) as i32
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

fn invoke_kind(v: &mut BinaryParser) -> (Vec<Register>, MethodIndex) {
    let first_byte = v.next();
    let addr = v.take(2);

    let mut args: Vec<Register> = Vec::new();
    let arg_count = first_byte >> 4 & 0b00001111;

    let mut arg_bytes = v.take(2);
    if arg_count > 0 && arg_count <= 5 {
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

fn invoke_kind_range(v: &mut BinaryParser) -> (Register, Register, MethodIndex) {
    let first_byte = v.next() as u32;
    let method_addr = to_decimal_short(&v.take(2));
    let start_register = to_decimal_short(&v.take(2)) as Register;

    return (start_register, (start_register + first_byte - 1), method_addr as MethodIndex);
}
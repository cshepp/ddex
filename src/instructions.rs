use std::fmt;
use std::fmt::Display;
use crate::binary_parser::BinaryParser;
use crate::dex_types::*;
use crate::util::{to_decimal, to_decimal_short, to_hex_string, to_i8};

type Register = u32;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub addr: usize,
    pub kind: InstructionKind,
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
                result.push(Instruction{
                    addr,
                    kind,
                });
            }
            None => break
        }
    }

    return result;
}

pub fn instruction_to_string(i: &Instruction) -> String {
    match &i.kind {
        InstructionKind::Nop => "nop".to_string(),
        InstructionKind::Move(a, b)             => format!("move v{} v{}", a, b),
        InstructionKind::MoveFrom16(a, b)       => format!("move/from16 v{} v{}", a, b),
        InstructionKind::Move16(a, b)           => format!("move/16 v{} v{}", a, b),
        InstructionKind::MoveWide(a, b)         => format!("move-wide v{} v{}", a, b),
        InstructionKind::MoveWideFrom16(a, b)   => format!("move-wide/from16 v{} v{}", a, b),
        InstructionKind::MoveWide16(a, b)       => format!("move-wide/16 v{} v{}", a, b),
        InstructionKind::MoveObject(a, b)       => format!("move-object v{} v{}", a, b),
        InstructionKind::MoveObjectFrom16(a, b) => format!("move-object/from16 v{} v{}", a, b),
        InstructionKind::MoveObject16(a, b)     => format!("move-object/16 v{} v{}", a, b),
        InstructionKind::MoveResult(a)          => format!("move-result v{}", a),
        InstructionKind::MoveResultWide(a)      => format!("move-result-wide v{}", a),
        InstructionKind::MoveResultObject(a)    => format!("move-object-wide v{}", a),
        InstructionKind::MoveException(a)       => format!("move-exception v{}", a),
        InstructionKind::ReturnVoid             => format!("return-void"),
        InstructionKind::Return(a)              => format!("return v{}", a),
        InstructionKind::ReturnWide(a)          => format!("return-wide v{}", a),
        InstructionKind::ReturnObject(a)        => format!("return-object v{}", a),
        InstructionKind::Const4(a, b)           => format!("const/4 v{} {:#x}", a, b),
        InstructionKind::Const16(a, b)          => format!("const/16 v{} {:#x}", a, b),
        InstructionKind::Const(a, b)            => format!("const v{} {:#x}", a, b),
        InstructionKind::ConstHigh16(a, b)      => format!("const/high16 v{} {:#x}", a, b),
        InstructionKind::ConstWide16(a, b)      => format!("const-wide/16 v{} {:#x}", a, b),
        InstructionKind::ConstWide32(a, b)      => format!("const-wide/32 v{} {:#x}", a, b),
        InstructionKind::ConstWide(a, b)        => format!("const-wide v{} {:#x}", a, b),
        InstructionKind::ConstWideHigh16(a, b)  => format!("const-wide/high16 v{} {:#x}", a, b),
        InstructionKind::ConstString(a, b)      => format!("const-string v{} {:#x}", a, b),
        InstructionKind::ConstStringJumbo(a, b) => format!("const-string/jumbo v{} {:#x}", a, b),
        InstructionKind::ConstClass(a, b)       => format!("const-class v{} {:#x}", a, b),
        InstructionKind::MonitorEnter(a)      => format!("monitor-enter v{}", a),
        InstructionKind::MonitorExit(a)       => format!("monitor-exit v{}", a),
        InstructionKind::CheckCast(a, b)      => format!("check-cast v{} {:#x}", a, b),
        InstructionKind::InstanceOf(a, b, c)  => format!("instance-of v{} v{} {:#x}", a, b, c),
        InstructionKind::ArrayLength(a, b)    => format!("array-length v{} v{}", a, b),
        InstructionKind::NewInstance(a, b)    => format!("new-instance v{} {:#x}", a, b),
        InstructionKind::NewArray(a, b, c)    => format!("new-array v{} v{} {:#x}", a, b, c),
        InstructionKind::FilledNewArray(a, b)         => format!("filled-new-array {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::FilledNewArrayRange(a, b, c) => format!("filled-new-array/range {{v{}..v{}}} {:#x}", a, b, c),
        InstructionKind::FillArrayData(a, b)          => format!("fill-array-data v{} {:#x}", a, b),
        InstructionKind::Throw(a)  => format!("throw v{}", a),
        InstructionKind::GoTo(a)   => format!("goto v{}", a),
        InstructionKind::GoTo16(a) => format!("goto/16 v{}", a),
        InstructionKind::GoTo32(a) => format!("goto/32 v{}", a),
        InstructionKind::PackedSwitch(a, b) => format!("packed-switch v{} {:#x}", a, b),
        InstructionKind::SparseSwitch(a, b) => format!("sparse-switch v{} {:#x}", a, b),
        InstructionKind::CmpLFloat(a, b, c)  => format!("cmpl-float v{} v{} v{}", a, b, c),
        InstructionKind::CmpGFloat(a, b, c)  => format!("cmpg-float v{} v{} v{}", a, b, c),
        InstructionKind::CmpLDouble(a, b, c) => format!("cmpl-double v{} v{} v{}", a, b, c),
        InstructionKind::CmpGDouble(a, b, c) => format!("cmpg-double v{} v{} v{}", a, b, c),
        InstructionKind::CmpLong(a, b, c)    => format!("cmp-long v{} v{} v{}", a, b, c),
        InstructionKind::IfEq(a, b, c) => format!("if-eq v{} v{} {:#x}", a, b, c),
        InstructionKind::IfNe(a, b, c) => format!("if-ne v{} v{} {:#x}", a, b, c),
        InstructionKind::IfLt(a, b, c) => format!("if-lt v{} v{} {:#x}", a, b, c),
        InstructionKind::IfGe(a, b, c) => format!("if-ge v{} v{} {:#x}", a, b, c),
        InstructionKind::IfGt(a, b, c) => format!("if-gt v{} v{} {:#x}", a, b, c),
        InstructionKind::IfLe(a, b, c) => format!("if-le v{} v{} {:#x}", a, b, c),
        InstructionKind::IfEqZ(a, b) => format!("if-eqz v{} {:#x}", a, b),
        InstructionKind::IfNeZ(a, b) => format!("if-nez v{} {:#x}", a, b),
        InstructionKind::IfLtZ(a, b) => format!("if-ltz v{} {:#x}", a, b),
        InstructionKind::IfGeZ(a, b) => format!("if-gez v{} {:#x}", a, b),
        InstructionKind::IfGtZ(a, b) => format!("if-gtz v{} {:#x}", a, b),
        InstructionKind::IfLeZ(a, b) => format!("if-lez v{} {:#x}", a, b),
        InstructionKind::AGet(a, b, c)        => format!("aget v{} v{} v{}", a, b, c),
        InstructionKind::AGetWide(a, b, c)    => format!("aget-wide v{} v{} v{}", a, b, c),
        InstructionKind::AGetObject(a, b, c)  => format!("aget-object v{} v{} v{}", a, b, c),
        InstructionKind::AGetBoolean(a, b, c) => format!("aget-boolean v{} v{} v{}", a, b, c),
        InstructionKind::AGetByte(a, b, c)    => format!("aget-byte v{} v{} v{}", a, b, c),
        InstructionKind::AGetChar(a, b, c)    => format!("aget-char v{} v{} v{}", a, b, c),
        InstructionKind::AGetShort(a, b, c)   => format!("aget-short v{} v{} v{}", a, b, c),
        InstructionKind::APut(a, b, c)        => format!("aput v{} v{} v{}", a, b, c),
        InstructionKind::APutWide(a, b, c)    => format!("aput-wide v{} v{} v{}", a, b, c),
        InstructionKind::APutObject(a, b, c)  => format!("aput-object v{} v{} v{}", a, b, c),
        InstructionKind::APutBoolean(a, b, c) => format!("aput-boolean v{} v{} v{}", a, b, c),
        InstructionKind::APutByte(a, b, c)    => format!("aput-byte v{} v{} v{}", a, b, c),
        InstructionKind::APutChar(a, b, c)    => format!("aput-char v{} v{} v{}", a, b, c),
        InstructionKind::APutShort(a, b, c)   => format!("aput-short v{} v{} v{}", a, b, c),
        InstructionKind::IGet(a, b, c)        => format!("iget v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetWide(a, b, c)    => format!("iget-wide v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetObject(a, b, c)  => format!("iget-object v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetBoolean(a, b, c) => format!("iget-boolean v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetByte(a, b, c)    => format!("iget-byte v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetChar(a, b, c)    => format!("iget-char v{} v{} {:#x}", a, b, c),
        InstructionKind::IGetShort(a, b, c)   => format!("iget-short v{} v{} {:#x}", a, b, c),
        InstructionKind::IPut(a, b, c)        => format!("iput v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutWide(a, b, c)    => format!("iput-wide v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutObject(a, b, c)  => format!("iput-object v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutBoolean(a, b, c) => format!("iput-boolean v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutByte(a, b, c)    => format!("iput-byte v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutChar(a, b, c)    => format!("iput-char v{} v{} {:#x}", a, b, c),
        InstructionKind::IPutShort(a, b, c)   => format!("iput-short v{} v{} {:#x}", a, b, c),
        InstructionKind::SGet(a, b)        => format!("sget v{} {:#x}", a, b),
        InstructionKind::SGetWide(a, b)    => format!("sget-wide v{} {:#x}", a, b),
        InstructionKind::SGetObject(a, b)  => format!("sget-object v{} {:#x}", a, b),
        InstructionKind::SGetBoolean(a, b) => format!("sget-boolean v{} {:#x}", a, b),
        InstructionKind::SGetByte(a, b)    => format!("sget-byte v{} {:#x}", a, b),
        InstructionKind::SGetChar(a, b)    => format!("sget-char v{} {:#x}", a, b),
        InstructionKind::SGetShort(a, b)   => format!("sget-short v{} {:#x}", a, b),
        InstructionKind::SPut(a, b)        => format!("sput v{} {:#x}", a, b),
        InstructionKind::SPutWide(a, b)    => format!("sput-wide v{} {:#x}", a, b),
        InstructionKind::SPutObject(a, b)  => format!("sput-object v{} {:#x}", a, b),
        InstructionKind::SPutBoolean(a, b) => format!("sput-boolean v{} {:#x}", a, b),
        InstructionKind::SPutByte(a, b)    => format!("sput-byte v{} {:#x}", a, b),
        InstructionKind::SPutChar(a, b)    => format!("sput-char v{} {:#x}", a, b),
        InstructionKind::SPutShort(a, b)   => format!("sput-short v{} {:#x}", a, b),
        InstructionKind::InvokeVirtual(a, b)           => format!("invoke-virtual {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::InvokeSuper(a, b)             => format!("invoke-super {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::InvokeDirect(a, b)            => format!("invoke-direct {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::InvokeStatic(a, b)            => format!("invoke-static {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::InvokeInterface(a, b)         => format!("invoke-interface {{{}}} {:#x}", register_list_to_string(&a), b),
        InstructionKind::InvokeVirtualRange(a, b, c)   => format!("invoke-virtual/range {{v{}..v{}}} {:#x}", a, b, c), 
        InstructionKind::InvokeSuperRange(a, b, c)     => format!("invoke-super/range {{v{}..v{}}} {:#x}", a, b, c),
        InstructionKind::InvokeDirectRange(a, b, c)    => format!("invoke-direct/range {{v{}..v{}}} {:#x}", a, b, c),
        InstructionKind::InvokeStaticRange(a, b, c)    => format!("invoke-static/range {{v{}..v{}}} {:#x}", a, b, c),
        InstructionKind::InvokeInterfaceRange(a, b, c) => format!("invoke-interface/range {{v{}..v{}}} {:#x}", a, b, c),
        InstructionKind::NegInt(a, b)        => format!("neg-int v{} v{}", a, b),
        InstructionKind::NotInt(a, b)        => format!("not-int v{} v{}", a, b),
        InstructionKind::NegLong(a, b)       => format!("neg-long v{} v{}", a, b),
        InstructionKind::NotLong(a, b)       => format!("not-long v{} v{}", a, b),
        InstructionKind::NegFloat(a, b)      => format!("neg-float v{} v{}", a, b),
        InstructionKind::NegDouble(a, b)     => format!("neg-double v{} v{}", a, b),
        InstructionKind::IntToLong(a, b)     => format!("int-to-long v{} v{}", a, b),
        InstructionKind::IntToFloat(a, b)    => format!("int-to-float v{} v{}", a, b),
        InstructionKind::IntToDouble(a, b)   => format!("int-to-double v{} v{}", a, b),
        InstructionKind::LongToInt(a, b)     => format!("long-to-int v{} v{}", a, b),
        InstructionKind::LongToFloat(a, b)   => format!("long-to-float v{} v{}", a, b),
        InstructionKind::LongToDouble(a, b)  => format!("long-to-double v{} v{}", a, b),
        InstructionKind::FloatToInt(a, b)    => format!("float-to-int v{} v{}", a, b),
        InstructionKind::FloatToLong(a, b)   => format!("float-to-long v{} v{}", a, b),
        InstructionKind::FloatToDouble(a, b) => format!("float-to-double v{} v{}", a, b),
        InstructionKind::DoubleToInt(a, b)   => format!("double-to-int v{} v{}", a, b),
        InstructionKind::DoubleToLong(a, b)  => format!("double-to-long v{} v{}", a, b),
        InstructionKind::DoubleToFloat(a, b) => format!("double-to-float v{} v{}", a, b),
        InstructionKind::IntToByte(a, b)     => format!("int-to-byte v{} v{}", a, b),
        InstructionKind::IntToChar(a, b)     => format!("int-to-char v{} v{}", a, b),
        InstructionKind::IntToShort(a, b)    => format!("int-to-short v{} v{}", a, b),
        InstructionKind::AddInt(a, b, c)    => format!("add-int v{} v{} v{}", a, b, c),
        InstructionKind::SubInt(a, b, c)    => format!("sub-int v{} v{} v{}", a, b, c),
        InstructionKind::MulInt(a, b, c)    => format!("mul-int v{} v{} v{}", a, b, c),
        InstructionKind::DivInt(a, b, c)    => format!("div-int v{} v{} v{}", a, b, c),
        InstructionKind::RemInt(a, b, c)    => format!("rem-int v{} v{} v{}", a, b, c),
        InstructionKind::AndInt(a, b, c)    => format!("and-int v{} v{} v{}", a, b, c),
        InstructionKind::OrInt(a, b, c)     => format!("or-int v{} v{} v{}", a, b, c),
        InstructionKind::XorInt(a, b, c)    => format!("xor-int v{} v{} v{}", a, b, c),
        InstructionKind::ShlInt(a, b, c)    => format!("shl-int v{} v{} v{}", a, b, c),
        InstructionKind::ShrInt(a, b, c)    => format!("shr-int v{} v{} v{}", a, b, c),
        InstructionKind::UShrInt(a, b, c)   => format!("ushr-int v{} v{} v{}", a, b, c),
        InstructionKind::AddLong(a, b, c)   => format!("add-long v{} v{} v{}", a, b, c),
        InstructionKind::SubLong(a, b, c)   => format!("sub-long v{} v{} v{}", a, b, c),
        InstructionKind::MulLong(a, b, c)   => format!("mul-long v{} v{} v{}", a, b, c),
        InstructionKind::DivLong(a, b, c)   => format!("div-long v{} v{} v{}", a, b, c),
        InstructionKind::RemLong(a, b, c)   => format!("rem-long v{} v{} v{}", a, b, c),
        InstructionKind::AndLong(a, b, c)   => format!("and-long v{} v{} v{}", a, b, c),
        InstructionKind::OrLong(a, b, c)    => format!("or-long v{} v{} v{}", a, b, c),
        InstructionKind::XorLong(a, b, c)   => format!("xor-long v{} v{} v{}", a, b, c),
        InstructionKind::ShlLong(a, b, c)   => format!("shl-long v{} v{} v{}", a, b, c),
        InstructionKind::ShrLong(a, b, c)   => format!("shr-long v{} v{} v{}", a, b, c),
        InstructionKind::UShrLong(a, b, c)  => format!("ushr-long v{} v{} v{}", a, b, c),
        InstructionKind::AddFloat(a, b, c)  => format!("add-float v{} v{} v{}", a, b, c),
        InstructionKind::SubFloat(a, b, c)  => format!("sub-float v{} v{} v{}", a, b, c),
        InstructionKind::MulFloat(a, b, c)  => format!("mul-float v{} v{} v{}", a, b, c),
        InstructionKind::DivFloat(a, b, c)  => format!("div-float v{} v{} v{}", a, b, c),
        InstructionKind::RemFloat(a, b, c)  => format!("rem-float v{} v{} v{}", a, b, c),
        InstructionKind::AddDouble(a, b, c) => format!("add-double v{} v{} v{}", a, b, c),
        InstructionKind::SubDouble(a, b, c) => format!("sub-double v{} v{} v{}", a, b, c),
        InstructionKind::MulDouble(a, b, c) => format!("mul-double v{} v{} v{}", a, b, c),
        InstructionKind::DivDouble(a, b, c) => format!("div-double v{} v{} v{}", a, b, c),
        InstructionKind::RemDouble(a, b, c) => format!("rem-double v{} v{} v{}", a, b, c),
        InstructionKind::AddInt2Addr(a, b)    => format!("add-int/2addr v{} v{}", a, b),
        InstructionKind::SubInt2Addr(a, b)    => format!("sub-int/2addr v{} v{}", a, b),
        InstructionKind::MulInt2Addr(a, b)    => format!("mul-int/2addr v{} v{}", a, b),
        InstructionKind::DivInt2Addr(a, b)    => format!("div-int/2addr v{} v{}", a, b),
        InstructionKind::RemInt2Addr(a, b)    => format!("rem-int/2addr v{} v{}", a, b),
        InstructionKind::AndInt2Addr(a, b)    => format!("and-int/2addr v{} v{}", a, b),
        InstructionKind::OrInt2Addr(a, b)     => format!("or-int/2addr v{} v{}", a, b),
        InstructionKind::XorInt2Addr(a, b)    => format!("xor-int/2addr v{} v{}", a, b),
        InstructionKind::ShlInt2Addr(a, b)    => format!("shl-int/2addr v{} v{}", a, b),
        InstructionKind::ShrInt2Addr(a, b)    => format!("shr-int/2addr v{} v{}", a, b),
        InstructionKind::UShrInt2Addr(a, b)   => format!("ushr-int/2addr v{} v{}", a, b),
        InstructionKind::AddLong2Addr(a, b)   => format!("add-long/2addr v{} v{}", a, b),
        InstructionKind::SubLong2Addr(a, b)   => format!("sub-long/2addr v{} v{}", a, b),
        InstructionKind::MulLong2Addr(a, b)   => format!("mul-long/2addr v{} v{}", a, b),
        InstructionKind::DivLong2Addr(a, b)   => format!("div-long/2addr v{} v{}", a, b),
        InstructionKind::RemLong2Addr(a, b)   => format!("rem-long/2addr v{} v{}", a, b),
        InstructionKind::AndLong2Addr(a, b)   => format!("and-long/2addr v{} v{}", a, b),
        InstructionKind::OrLong2Addr(a, b)    => format!("or-long/2addr v{} v{}", a, b),
        InstructionKind::XorLong2Addr(a, b)   => format!("xor-long/2addr v{} v{}", a, b),
        InstructionKind::ShlLong2Addr(a, b)   => format!("shl-long/2addr v{} v{}", a, b),
        InstructionKind::ShrLong2Addr(a, b)   => format!("shr-long/2addr v{} v{}", a, b),
        InstructionKind::UShrLong2Addr(a, b)  => format!("ushr-long/2addr v{} v{}", a, b),
        InstructionKind::AddFloat2Addr(a, b)  => format!("add-float/2addr v{} v{}", a, b),
        InstructionKind::SubFloat2Addr(a, b)  => format!("sub-float/2addr v{} v{}", a, b),
        InstructionKind::MulFloat2Addr(a, b)  => format!("mul-float/2addr v{} v{}", a, b),
        InstructionKind::DivFloat2Addr(a, b)  => format!("div-float/2addr v{} v{}", a, b),
        InstructionKind::RemFloat2Addr(a, b)  => format!("rem-float/2addr v{} v{}", a, b),
        InstructionKind::AddDouble2Addr(a, b) => format!("add-double/2addr v{} v{}", a, b),
        InstructionKind::SubDouble2Addr(a, b) => format!("sub-double/2addr v{} v{}", a, b),
        InstructionKind::MulDouble2Addr(a, b) => format!("mul-double/2addr v{} v{}", a, b),
        InstructionKind::DivDouble2Addr(a, b) => format!("div-double/2addr v{} v{}", a, b),
        InstructionKind::RemDouble2Addr(a, b) => format!("rem-double/2addr v{} v{}", a, b),
        InstructionKind::AddIntLit16(a, b, c)  => format!("add-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::RSubIntLit16(a, b, c) => format!("rsub-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::MulIntLit16(a, b, c)  => format!("mul-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::DivIntLit16(a, b, c)  => format!("div-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::RemIntLit16(a, b, c)  => format!("rem-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::AndIntLit16(a, b, c)  => format!("and-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::OrIntLit16(a, b, c)   => format!("or-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::XorIntLit16(a, b, c)  => format!("xor-int/lit16 v{} v{} {:#x}", a, b, c),
        InstructionKind::AddIntLit8(a, b, c)   => format!("add-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::RSubIntLit8(a, b, c)  => format!("rsub-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::MulIntLit8(a, b, c)   => format!("mul-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::DivIntLit8(a, b, c)   => format!("div-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::RemIntLit8(a, b, c)   => format!("rem-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::AndIntLit8(a, b, c)   => format!("and-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::OrIntLit8(a, b, c)    => format!("or-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::XorIntLit8(a, b, c)   => format!("xor-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::ShlIntLit8(a, b, c)   => format!("shl-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::ShrIntLit8(a, b, c)   => format!("shr-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::UShrIntLit8(a, b, c)  => format!("ushr-int/lit8 v{} v{} {:#x}", a, b, c),
        InstructionKind::InvokePolymorphic => format!(""),      // TODO 
        InstructionKind::InvokePolymorphicRange => format!(""), // TODO 
        InstructionKind::InvokeCustom => format!(""),           // TODO 
        InstructionKind::InvokeCustomRange => format!(""),      // TODO 
        InstructionKind::ConstMethodHandle => format!(""),      // TODO 
        InstructionKind::ConstMethodType => format!(""),        // TODO 
        InstructionKind::Unused => format!(""),
        _ => "".to_string(),
    }
}

fn register_list_to_string(v: &Vec<Register>) -> String {
    v.iter().map(|x| format!("v{}", x)).collect::<Vec<String>>().join(", ")
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


#[test]
pub fn test_stuff() {
    let mut bytecode = vec![0x01, 0x01];
    let mut parser = BinaryParser::new(bytecode);

    let instruction = bytecode_to_instruction_kind(&mut parser);
    match instruction {
        Some(x) => {
            match x {
                InstructionKind::Move(a, b) => {
                    assert_eq!(a, 1);
                    assert_eq!(b, 0);
                },
                _ => panic!()
            }
        }
        None => panic!()
    }
}

#[test]
pub fn test_args_arity_one() {
    let mut bytecode = vec![0x10, 0xff, 0xff, 0x04, 0x00];
    let mut parser = BinaryParser::new(bytecode);

    let (args, i) = invoke_kind(&mut parser);
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], 4);
}

#[test]
pub fn test_args_arity_two() {
    let mut bytecode = vec![0x5f, 0x2c, 0x00, 0xb0, 0x5f];
    let mut parser = BinaryParser::new(bytecode);

    let (args, i) = invoke_kind(&mut parser);
    assert_eq!(args.len(), 5);
    assert_eq!(args, vec![0, 11, 15, 5, 15]);
}

#[test]
pub fn test_invoke_kind_range() {
    let mut bytecode = vec![0x0a, 0x8f, 0x11, 0x04, 0x00];
    let mut parser = BinaryParser::new(bytecode);

    let (r1, r2, method) = invoke_kind_range(&mut parser);
    assert_eq!(r1, 4);
    assert_eq!(r2, 13);
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x} {}", self.addr, instruction_to_string(self))
    }
}
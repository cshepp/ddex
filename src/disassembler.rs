use std::fmt;
use std::fmt::{Display, Write};

use crate::dex_types::*;
use crate::instructions::*;

pub struct Disassembler {
    pub strings: Vec<String>,
    pub types: Vec<DexType>,
    pub protos: Vec<DexProto>,
    pub fields: Vec<DexField>,
    pub methods: Vec<DexMethod>,
    pub classes: Vec<DexClassDef>,
    pub instructions: Vec<Instruction>,
}

impl Disassembler {

    pub fn print(&self) {
        for i in &self.instructions {
            
            match self.instruction_comment(i) {
                Some(s) => println!("{:#x} {: <12} {: <64} ; {}", i.addr, to_hex(&i.bytecode), self.instruction_to_string(i), s),
                None => println!("{:#x} {: <12} {}", i.addr, to_hex(&i.bytecode), self.instruction_to_string(i)),
            };
        }
    }

    fn instruction_to_string(&self, i: &Instruction) -> String {
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
            InstructionKind::GoTo(a)   => format!("goto {:#x}", i.addr as i32 + *a),
            InstructionKind::GoTo16(a) => format!("goto/16 {:#x}", i.addr as i32 + *a),
            InstructionKind::GoTo32(a) => format!("goto/32 {:#x}", i.addr as i32 + *a),
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
            InstructionKind::InvokePolymorphic      => format!(""), // TODO 
            InstructionKind::InvokePolymorphicRange => format!(""), // TODO 
            InstructionKind::InvokeCustom           => format!(""), // TODO 
            InstructionKind::InvokeCustomRange      => format!(""), // TODO 
            InstructionKind::ConstMethodHandle      => format!(""), // TODO 
            InstructionKind::ConstMethodType        => format!(""), // TODO 
            InstructionKind::Unused                 => format!(""),
            _ => "".to_string(),
        }
    }

    fn instruction_comment(&self, i: &Instruction) -> Option<String> {
        match i.kind {
            InstructionKind::ConstString(_, i)             => Some(self.string_at_index(i)),
            InstructionKind::ConstStringJumbo(_, i)        => Some(self.string_at_index(i)),
            InstructionKind::ConstClass(_, i)              => Some(self.type_at_index(i)),
            InstructionKind::CheckCast(_, i)               => Some(self.type_at_index(i)),
            InstructionKind::InstanceOf(_, _, i)           => Some(self.type_at_index(i)),
            InstructionKind::NewInstance(_, i)             => Some(self.type_at_index(i)),
            InstructionKind::NewArray(_, _, i)             => Some(self.type_at_index(i)),
            InstructionKind::FilledNewArray(_, i)          => Some(self.type_at_index(i)),
            InstructionKind::FilledNewArrayRange(_, _, i)  => Some(self.type_at_index(i)),
            InstructionKind::IGet(_, _, i)                 => Some(self.field_at_index(i)),
            InstructionKind::IGetWide(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IGetObject(_, _, i)           => Some(self.field_at_index(i)),
            InstructionKind::IGetBoolean(_, _, i)          => Some(self.field_at_index(i)),
            InstructionKind::IGetByte(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IGetChar(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IGetShort(_, _, i)            => Some(self.field_at_index(i)),
            InstructionKind::IPut(_, _, i)                 => Some(self.field_at_index(i)),
            InstructionKind::IPutWide(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IPutObject(_, _, i)           => Some(self.field_at_index(i)),
            InstructionKind::IPutBoolean(_, _, i)          => Some(self.field_at_index(i)),
            InstructionKind::IPutByte(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IPutChar(_, _, i)             => Some(self.field_at_index(i)),
            InstructionKind::IPutShort(_, _, i)            => Some(self.field_at_index(i)),
            InstructionKind::SGet(_, i)                    => Some(self.field_at_index(i)),
            InstructionKind::SGetWide(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SGetObject(_, i)              => Some(self.field_at_index(i)),
            InstructionKind::SGetBoolean(_, i)             => Some(self.field_at_index(i)),
            InstructionKind::SGetByte(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SGetChar(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SGetShort(_, i)               => Some(self.field_at_index(i)),
            InstructionKind::SPut(_, i)                    => Some(self.field_at_index(i)),
            InstructionKind::SPutWide(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SPutObject(_, i)              => Some(self.field_at_index(i)),
            InstructionKind::SPutBoolean(_, i)             => Some(self.field_at_index(i)),
            InstructionKind::SPutByte(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SPutChar(_, i)                => Some(self.field_at_index(i)),
            InstructionKind::SPutShort(_, i)               => Some(self.field_at_index(i)),
            InstructionKind::InvokeVirtual(_, i)           => Some(self.method_at_index(i)),
            InstructionKind::InvokeSuper(_, i)             => Some(self.method_at_index(i)),
            InstructionKind::InvokeDirect(_, i)            => Some(self.method_at_index(i)),
            InstructionKind::InvokeStatic(_, i)            => Some(self.method_at_index(i)),
            InstructionKind::InvokeInterface(_, i)         => Some(self.method_at_index(i)),
            InstructionKind::InvokeVirtualRange(_, _, i)   => Some(self.method_at_index(i)),
            InstructionKind::InvokeSuperRange(_, _, i)     => Some(self.method_at_index(i)),
            InstructionKind::InvokeDirectRange(_, _, i)    => Some(self.method_at_index(i)),
            InstructionKind::InvokeStaticRange(_, _, i)    => Some(self.method_at_index(i)),
            InstructionKind::InvokeInterfaceRange(_, _, i) => Some(self.method_at_index(i)),
            _ => None,
        }
    }

    fn string_at_index(&self, i: StringIndex) -> String {
        self.strings[i].clone()
    }

    fn type_at_index(&self, i: TypeIndex) -> String {
        if i >= self.types.len() {
            return format!("TypeIndex out of bounds: {}", i);
        }
        format!("{}", self.types[i].parsed)
    }

    fn field_at_index(&self, i: FieldIndex) -> String {
        if i >= self.fields.len() {
            return format!("FieldIndex ouf of bounds: {}", i);
        }
        let field = self.fields[i].clone();
        let name = self.strings[field.name_idx].clone();
        let field_type = self.types[field.type_idx].clone();
        format!("{} ({})", name, field_type.parsed)
    }

    fn method_at_index(&self, i: MethodIndex) -> String {
        if i >= self.methods.len() {
            return format!("MethodIndex out of bounds: {}", i);
        }
        let method = &self.methods[i];
        let method_proto = &self.protos[method.proto_idx as usize];
        let method_name = &self.strings[method.name_idx as usize];

        let return_type = &self.types[method_proto.return_type_idx as usize];
        let param_types = method_proto.parameter_type_idx_list.iter()
            .map(|idx: &TypeIndex| {
                let t = &self.types[*idx].parsed;
                return format!("{}", t);
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("{}({}) -> {}", method_name, param_types, return_type.parsed)
    }
}

fn register_list_to_string(v: &Vec<Register>) -> String {
    v.iter().map(|x| format!("v{}", x)).collect::<Vec<String>>().join(", ")
}

fn to_hex(ls: &Vec<u8>) -> String {
    let mut output = String::new();
    for i in ls {
        write!(&mut output, "{:01$x}", i, 2).expect("Couldn't write to string.");
    }
    return output;
}
use strum_macros::EnumIter;

use crate::{BranchParams, DropKeep, Fuel, Index, JumpDest, Offset, UntypedValue};

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum OpCode {
    // common opcode family
    Unreachable,
    ConsumeFuel(Fuel),
    Drop,
    Select,

    // local opcode family
    LocalGet(Index),
    LocalSet(Index),
    LocalTee(Index),

    // control float opcode family
    Br(BranchParams),
    BrIfEqz(BranchParams),
    BrIfNez(BranchParams),
    BrTable(Index),
    Return(DropKeep),
    ReturnIfNez(DropKeep),
    ReturnCall(Index, DropKeep),
    ReturnCallIndirect(Index, DropKeep),
    Call(Index),
    CallIndirect(Index),

    // global opcode family
    GlobalGet(Index),
    GlobalSet(Index),

    // memory opcode family
    I32Load(Offset),
    I64Load(Offset),
    F32Load(Offset),
    F64Load(Offset),
    I32Load8S(Offset),
    I32Load8U(Offset),
    I32Load16S(Offset),
    I32Load16U(Offset),
    I64Load8S(Offset),
    I64Load8U(Offset),
    I64Load16S(Offset),
    I64Load16U(Offset),
    I64Load32S(Offset),
    I64Load32U(Offset),
    I32Store(Offset),
    I64Store(Offset),
    F32Store(Offset),
    F64Store(Offset),
    I32Store8(Offset),
    I32Store16(Offset),
    I64Store8(Offset),
    I64Store16(Offset),
    I64Store32(Offset),

    // data memory opcodes (?)
    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit(Index),
    DataDrop(Index),
    TableSize(Index),
    TableGrow(Index),
    TableFill(Index),
    TableGet(Index),
    TableSet(Index),
    TableCopy { dst: Index, src: Index },
    TableInit { table: Index, elem: Index },
    ElemDrop(Index),
    RefFunc(Index),

    // i32/i64 opcode family
    I64Const(UntypedValue),
    I32Const(UntypedValue),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    // f32/f64 opcode family
    F64Const(UntypedValue),
    F32Const(UntypedValue),
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
}

impl OpCode {
    pub fn const_i32<C>(value: C) -> Self
    where
        C: Into<UntypedValue>,
    {
        Self::I32Const(value.into())
    }

    pub fn const_i64<C>(value: C) -> Self
    where
        C: Into<UntypedValue>,
    {
        Self::I64Const(value.into())
    }

    /// Creates a new `local.get` instruction from the given local depth.
    pub fn local_get(local_depth: usize) -> Self {
        Self::LocalGet(Index::from(local_depth as u32))
    }

    /// Creates a new `local.set` instruction from the given local depth.
    pub fn local_set(local_depth: usize) -> Self {
        Self::LocalSet(Index::from(local_depth as u32))
    }

    /// Creates a new `local.tee` instruction from the given local depth.
    pub fn local_tee(local_depth: usize) -> Self {
        Self::LocalTee(Index::from(local_depth as u32))
    }

    /// Convenience method to create a new `ConsumeFuel` instruction.
    pub fn consume_fuel(amount: u64) -> Self {
        Self::ConsumeFuel(Fuel::from(amount))
    }

    pub fn get_jump_offset(&self) -> Option<JumpDest> {
        match self {
            OpCode::Br(offset) => Some(offset.offset),
            OpCode::BrIfEqz(offset) => Some(offset.offset),
            OpCode::BrIfNez(offset) => Some(offset.offset),
            _ => None,
        }
    }

    pub fn update_branch_offset(&mut self, offset: JumpDest) {
        *self = self.rewrite_jump_offset(offset);
    }

    pub fn rewrite_jump_offset(&self, new_offset: JumpDest) -> OpCode {
        match self {
            OpCode::Br(mut branch_params) => {
                branch_params.offset = new_offset;
                OpCode::Br(branch_params)
            }
            OpCode::BrIfEqz(mut branch_params) => {
                branch_params.offset = new_offset;
                OpCode::BrIfEqz(branch_params)
            }
            OpCode::BrIfNez(mut branch_params) => {
                branch_params.offset = new_offset;
                OpCode::BrIfNez(branch_params)
            }
            _ => unreachable!("branch offset override is not supported for opcode: {:?}", self),
        }
    }

    pub fn add_offset(&self, offset_diff: u32) -> OpCode {
        match *self {
            OpCode::I32Load(offset) => OpCode::I32Load(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load(offset) => OpCode::I64Load(Offset::from(offset.0 + offset_diff)),
            OpCode::F32Load(offset) => OpCode::F32Load(Offset::from(offset.0 + offset_diff)),
            OpCode::F64Load(offset) => OpCode::F64Load(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Load8S(offset) => OpCode::I32Load8S(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Load8U(offset) => OpCode::I32Load8U(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Load16S(offset) => OpCode::I32Load16S(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Load16U(offset) => OpCode::I32Load16U(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load8S(offset) => OpCode::I64Load8S(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load8U(offset) => OpCode::I64Load8U(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load16S(offset) => OpCode::I64Load16S(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load16U(offset) => OpCode::I64Load16U(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load32S(offset) => OpCode::I64Load32S(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Load32U(offset) => OpCode::I64Load32U(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Store(offset) => OpCode::I32Store(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Store(offset) => OpCode::I64Store(Offset::from(offset.0 + offset_diff)),
            OpCode::F32Store(offset) => OpCode::F32Store(Offset::from(offset.0 + offset_diff)),
            OpCode::F64Store(offset) => OpCode::F64Store(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Store8(offset) => OpCode::I32Store8(Offset::from(offset.0 + offset_diff)),
            OpCode::I32Store16(offset) => OpCode::I32Store16(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Store8(offset) => OpCode::I64Store8(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Store16(offset) => OpCode::I64Store16(Offset::from(offset.0 + offset_diff)),
            OpCode::I64Store32(offset) => OpCode::I64Store32(Offset::from(offset.0 + offset_diff)),
            _ => unreachable!("offset override is not supported for opcode: {:?}", self),
        }
    }

    /// Increases the fuel consumption of the [`ConsumeFuel`] instruction by `delta`.
    ///
    /// # Panics
    ///
    /// - If `self` is not a [`ConsumeFuel`] instruction.
    /// - If the new fuel consumption overflows the internal `u64` value.
    ///
    /// [`ConsumeFuel`]: Instruction::ConsumeFuel
    pub fn bump_fuel_consumption(&mut self, delta: u64) {
        match self {
            Self::ConsumeFuel(fuel) => {
                let amount = fuel.0;
                fuel.0 = amount
                    .checked_add(delta)
                    .unwrap_or_else(|| panic!("overflowed fuel consumption. current = {amount}, delta = {delta}",))
            }
            instr => panic!("expected Instruction::ConsumeFuel but found: {instr:?}"),
        }
    }
}

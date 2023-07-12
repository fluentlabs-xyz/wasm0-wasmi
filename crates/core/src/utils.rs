use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::{
    io::Cursor,
    mem::take,
    ops::{Add, Neg, Sub},
};

use crate::{BinaryFormat, OpCode, UntypedValue, WazmError, WazmResult};

impl<'a> BinaryFormat<'a> for u32 {
    type SelfType = u32;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        let mut buf: [u8; 4] = [0; 4];
        BigEndian::write_u32(&mut buf, *self);
        sink.extend_from_slice(&buf);
        // leb128::write::unsigned(sink, *self as u64).map_err(|_| WazmError::OutOfBuffer)?;
        Ok(())
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self::SelfType> {
        let raw_offset = sink.read_u32::<BigEndian>().map_err(|_| WazmError::OutOfBuffer)?;
        // let raw_offset = leb128::read::unsigned(sink).unwrap() as u32;
        Ok(raw_offset)
    }
}

impl<'a> BinaryFormat<'a> for i32 {
    type SelfType = i32;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        let mut buf: [u8; 4] = [0; 4];
        BigEndian::write_i32(&mut buf, *self);
        sink.extend_from_slice(&buf);
        // leb128::write::signed(sink, *self as i64).map_err(|_| WazmError::OutOfBuffer)?;
        Ok(())
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self::SelfType> {
        let raw_offset = sink.read_i32::<BigEndian>().map_err(|_| WazmError::OutOfBuffer)?;
        // let raw_offset = leb128::read::signed(sink).unwrap() as i32;
        Ok(raw_offset)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct JumpDest(pub i32);

impl<'a> BinaryFormat<'a> for JumpDest {
    type SelfType = JumpDest;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self> {
        Ok(Self(i32::read_binary(sink)?))
    }
}

impl From<i32> for JumpDest {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl JumpDest {
    pub fn neg(&self) -> JumpDest {
        Self(self.0.neg())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Offset(pub u32);

impl<'a> BinaryFormat<'a> for Offset {
    type SelfType = Offset;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self> {
        Ok(Self(u32::read_binary(sink)?))
    }
}

impl From<u32> for Offset {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Add<i32> for Offset {
    type Output = Offset;

    fn add(self, rhs: i32) -> Self::Output {
        Offset((self.0 as i32 + rhs) as u32)
    }
}
impl Sub<i32> for Offset {
    type Output = Offset;

    fn sub(self, rhs: i32) -> Self::Output {
        Offset((self.0 as i32 - rhs) as u32)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct Index(pub u32);

impl<'a> BinaryFormat<'a> for Index {
    type SelfType = Index;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self> {
        Ok(Self(u32::read_binary(sink)?))
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Fuel(pub u32);

impl From<u32> for Fuel {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl<'a> BinaryFormat<'a> for Fuel {
    type SelfType = Fuel;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self> {
        Ok(Self(u32::read_binary(sink)?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DropKeep {
    pub drop: Offset,
    pub keep: Offset,
}

impl DropKeep {
    pub fn new(drop: Offset, keep: Offset) -> Self {
        Self { drop, keep }
    }

    pub fn non_empty(&self) -> bool {
        self.drop.0 + self.keep.0 > 0
    }

    pub fn drop(&self) -> usize {
        self.drop.0 as usize
    }

    pub fn keep(&self) -> usize {
        self.keep.0 as usize
    }
}

impl<'a> BinaryFormat<'a> for DropKeep {
    type SelfType = DropKeep;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        self.drop.write_binary(sink)?;
        self.keep.write_binary(sink)?;
        Ok(())
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<DropKeep> {
        Ok(Self {
            drop: Offset::read_binary(sink)?,
            keep: Offset::read_binary(sink)?,
        })
    }
}

#[derive(Default, Clone)]
pub struct InstructionSet(pub Vec<OpCode>);

macro_rules! impl_opcode {
    ($name:ident, $opcode:ident($into:ident)) => {
        pub fn $name<I: Into<$into>>(&mut self, value: I) {
            self.0.push(OpCode::$opcode(value.into()));
        }
    };
    ($name:ident, $opcode:ident) => {
        pub fn $name(&mut self) {
            self.0.push(OpCode::$opcode);
        }
    };
}

impl From<Vec<OpCode>> for InstructionSet {
    fn from(value: Vec<OpCode>) -> Self {
        Self(value)
    }
}

impl InstructionSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, opcode: OpCode) -> u32 {
        let opcode_pos = self.len();
        self.0.push(opcode);
        opcode_pos
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    impl_opcode!(op_unreachable, Unreachable);
    impl_opcode!(op_consume_fuel, ConsumeFuel(Fuel));
    impl_opcode!(op_drop, Drop);
    impl_opcode!(op_select, Select);
    impl_opcode!(op_local_get, LocalGet(Index));
    impl_opcode!(op_local_set, LocalSet(Index));
    impl_opcode!(op_local_tee, LocalTee(Index));
    impl_opcode!(op_br, Br(JumpDest));
    impl_opcode!(op_br_if_eqz, BrIfEqz(JumpDest));
    impl_opcode!(op_br_if_nez, BrIfNez(JumpDest));
    impl_opcode!(op_br_table, BrTable(Index));
    impl_opcode!(op_return, Return);
    impl_opcode!(op_return_call_indirect, ReturnCallIndirect(Index));
    impl_opcode!(op_call, Call(JumpDest));
    impl_opcode!(op_call_host, CallHost(Index));
    impl_opcode!(op_call_indirect, CallIndirect(Index));
    impl_opcode!(op_global_get, GlobalGet(Index));
    impl_opcode!(op_global_set, GlobalSet(Index));
    // add more opcodes
    impl_opcode!(op_i32_const, I32Const(UntypedValue));
    impl_opcode!(op_i64_const, I64Const(UntypedValue));

    pub fn extend<I: Into<InstructionSet>>(&mut self, with: I) {
        self.0.extend(Into::<InstructionSet>::into(with).0);
    }

    pub fn finalize(&mut self) -> Vec<OpCode> {
        take(&mut self.0)
    }
}

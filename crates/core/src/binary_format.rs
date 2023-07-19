use crate::{BranchParams, DropKeep, Fuel, Index, InstructionSet, JumpDest, Offset, OpCode, UntypedValue};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Copy, Clone)]
pub enum BinaryFormatError {
    NeedMore(usize),
    IllegalOpcode(u8),
}

pub struct BinaryFormatWriter<'a> {
    pub sink: &'a mut [u8],
    pos: usize,
}

impl<'a> BinaryFormatWriter<'a> {
    pub fn new(sink: &'a mut [u8]) -> Self {
        Self { sink, pos: 0 }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), BinaryFormatError> {
        self.require(1)?;
        self.sink[self.pos] = value;
        self.skip(1)
    }

    pub fn write_u16_be(&mut self, value: u16) -> Result<(), BinaryFormatError> {
        self.require(2)?;
        BigEndian::write_u16(&mut self.sink[self.pos..], value);
        self.skip(2)
    }

    pub fn write_i16_be(&mut self, value: i16) -> Result<(), BinaryFormatError> {
        self.require(2)?;
        BigEndian::write_i16(&mut self.sink[self.pos..], value);
        self.skip(3)
    }

    pub fn write_u32_be(&mut self, value: u32) -> Result<(), BinaryFormatError> {
        self.require(4)?;
        BigEndian::write_u32(&mut self.sink[self.pos..], value);
        self.skip(4)
    }

    pub fn write_i32_be(&mut self, value: i32) -> Result<(), BinaryFormatError> {
        self.require(4)?;
        BigEndian::write_i32(&mut self.sink[self.pos..], value);
        self.skip(4)
    }

    pub fn write_u64_be(&mut self, value: u64) -> Result<(), BinaryFormatError> {
        self.require(8)?;
        BigEndian::write_u64(&mut self.sink[self.pos..], value);
        self.skip(8)
    }

    pub fn write_i64_be(&mut self, value: i64) -> Result<(), BinaryFormatError> {
        self.require(8)?;
        BigEndian::write_i64(&mut self.sink[self.pos..], value);
        self.skip(8)
    }

    fn require(&self, n: usize) -> Result<(), BinaryFormatError> {
        if self.sink.len() < self.pos + n {
            Err(BinaryFormatError::NeedMore(self.pos + n - self.sink.len()))
        } else {
            Ok(())
        }
    }

    fn skip(&mut self, n: usize) -> Result<(), BinaryFormatError> {
        assert!(self.sink.len() >= self.pos + n);
        self.pos += n;
        Ok(())
    }

    #[cfg(feature = "std")]
    pub fn to_vec(&self) -> Vec<u8> {
        self.sink[0..self.pos].to_vec()
    }
}

pub struct BinaryFormatReader<'a> {
    pub sink: &'a [u8],
    pos: usize,
}

impl<'a> BinaryFormatReader<'a> {
    pub fn new(sink: &'a [u8]) -> Self {
        Self { sink, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.sink.len()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn read_u8(&mut self) -> Result<u8, BinaryFormatError> {
        self.require(1)?;
        let result = self.sink[self.pos];
        self.skip(1)?;
        Ok(result)
    }

    pub fn read_u16_be(&mut self) -> Result<u16, BinaryFormatError> {
        self.require(2)?;
        let result = BigEndian::read_u16(&self.sink[self.pos..]);
        self.skip(2)?;
        Ok(result)
    }

    pub fn read_i16_be(&mut self) -> Result<i16, BinaryFormatError> {
        self.require(2)?;
        let result = BigEndian::read_i16(&self.sink[self.pos..]);
        self.skip(2)?;
        Ok(result)
    }

    pub fn read_u32_be(&mut self) -> Result<u32, BinaryFormatError> {
        self.require(4)?;
        let result = BigEndian::read_u32(&self.sink[self.pos..]);
        self.skip(4)?;
        Ok(result)
    }

    pub fn read_i32_be(&mut self) -> Result<i32, BinaryFormatError> {
        self.require(4)?;
        let result = BigEndian::read_i32(&self.sink[self.pos..]);
        self.skip(4)?;
        Ok(result)
    }

    pub fn read_u64_be(&mut self) -> Result<u64, BinaryFormatError> {
        self.require(8)?;
        let result = BigEndian::read_u64(&self.sink[self.pos..]);
        self.skip(8)?;
        Ok(result)
    }

    pub fn read_i64_be(&mut self) -> Result<i64, BinaryFormatError> {
        self.require(8)?;
        let result = BigEndian::read_i64(&self.sink[self.pos..]);
        self.skip(8)?;
        Ok(result)
    }

    fn require(&self, n: usize) -> Result<(), BinaryFormatError> {
        if self.sink.len() < self.pos + n {
            Err(BinaryFormatError::NeedMore(self.pos + n - self.sink.len()))
        } else {
            Ok(())
        }
    }

    fn skip(&mut self, n: usize) -> Result<(), BinaryFormatError> {
        assert!(self.sink.len() >= self.pos + n);
        self.pos += n;
        Ok(())
    }
}

pub trait BinaryFormat<'a> {
    type SelfType;

    fn write_binary_to_vec(&self, buf: &'a mut Vec<u8>) -> Result<(), BinaryFormatError> {
        let buf = unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.capacity()) };
        let mut sink = BinaryFormatWriter::<'a>::new(buf);
        self.write_binary(&mut sink)?;
        Ok(())
    }

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError>;
    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<Self::SelfType, BinaryFormatError>;
}

impl<'a> BinaryFormat<'a> for u32 {
    type SelfType = u32;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        sink.write_u32_be(*self)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<u32, BinaryFormatError> {
        sink.read_u32_be()
    }
}

impl<'a> BinaryFormat<'a> for i32 {
    type SelfType = i32;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        sink.write_i32_be(*self)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<i32, BinaryFormatError> {
        sink.read_i32_be()
    }
}

impl<'a> BinaryFormat<'a> for u64 {
    type SelfType = u64;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        sink.write_u64_be(*self)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<u64, BinaryFormatError> {
        sink.read_u64_be()
    }
}

impl<'a> BinaryFormat<'a> for i64 {
    type SelfType = i64;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        sink.write_i64_be(*self)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<i64, BinaryFormatError> {
        sink.read_i64_be()
    }
}

impl<'a> BinaryFormat<'a> for JumpDest {
    type SelfType = JumpDest;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<JumpDest, BinaryFormatError> {
        Ok(Self(i32::read_binary(sink)?))
    }
}

impl<'a> BinaryFormat<'a> for Offset {
    type SelfType = Offset;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<Offset, BinaryFormatError> {
        Ok(Self(u32::read_binary(sink)?))
    }
}

impl<'a> BinaryFormat<'a> for Index {
    type SelfType = Index;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<Index, BinaryFormatError> {
        Ok(Self(u32::read_binary(sink)?))
    }
}

impl<'a> BinaryFormat<'a> for Fuel {
    type SelfType = Fuel;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.0.write_binary(sink)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<Fuel, BinaryFormatError> {
        Ok(Self(u64::read_binary(sink)?))
    }
}

impl<'a> BinaryFormat<'a> for DropKeep {
    type SelfType = DropKeep;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        sink.write_u32_be(self.drop)?;
        sink.write_u32_be(self.keep)?;
        Ok(())
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<DropKeep, BinaryFormatError> {
        Ok(Self {
            drop: sink.read_u32_be()?,
            keep: sink.read_u32_be()?,
        })
    }
}

impl<'a> BinaryFormat<'a> for UntypedValue {
    type SelfType = UntypedValue;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.bits.write_binary(sink)
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<UntypedValue, BinaryFormatError> {
        Ok(UntypedValue::from_bits(u64::read_binary(sink)?))
    }
}

impl<'a> BinaryFormat<'a> for BranchParams {
    type SelfType = BranchParams;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        self.offset.write_binary(sink)?;
        self.drop_keep.write_binary(sink)?;
        Ok(())
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<BranchParams, BinaryFormatError> {
        Ok(BranchParams::new(
            JumpDest::read_binary(sink)?,
            DropKeep::read_binary(sink)?,
        ))
    }
}

impl<'a> BinaryFormat<'a> for OpCode {
    type SelfType = OpCode;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        match self {
            OpCode::Unreachable => sink.write_u8(0x00)?,
            OpCode::ConsumeFuel(u) => {
                sink.write_u8(0x01)?;
                u.write_binary(sink)?;
            }
            OpCode::Drop => sink.write_u8(0x02)?,
            OpCode::Select => sink.write_u8(0x04)?,

            // local opcode family
            OpCode::LocalGet(index) => {
                sink.write_u8(0x10)?;
                index.write_binary(sink)?;
            }
            OpCode::LocalSet(index) => {
                sink.write_u8(0x11)?;
                index.write_binary(sink)?;
            }
            OpCode::LocalTee(index) => {
                sink.write_u8(0x12)?;
                index.write_binary(sink)?;
            }

            // control flow opcode family
            OpCode::Br(branch_params) => {
                sink.write_u8(0x20)?;
                branch_params.write_binary(sink)?;
            }
            OpCode::BrIfEqz(branch_params) => {
                sink.write_u8(0x21)?;
                branch_params.write_binary(sink)?;
            }
            OpCode::BrIfNez(branch_params) => {
                sink.write_u8(0x22)?;
                branch_params.write_binary(sink)?;
            }
            OpCode::BrTable(index) => {
                sink.write_u8(0x23)?;
                index.write_binary(sink)?;
            }
            OpCode::Return(drop_keep) => {
                sink.write_u8(0x24)?;
                drop_keep.write_binary(sink)?;
            }
            OpCode::ReturnIfNez(drop_keep) => {
                sink.write_u8(0x25)?;
                drop_keep.write_binary(sink)?;
            }
            OpCode::ReturnCall(table, drop_keep) => {
                sink.write_u8(0x26)?;
                table.write_binary(sink)?;
                drop_keep.write_binary(sink)?;
            }
            OpCode::ReturnCallIndirect(table, drop_keep) => {
                sink.write_u8(0x27)?;
                table.write_binary(sink)?;
                drop_keep.write_binary(sink)?;
            }
            OpCode::Call(jump_dest) => {
                sink.write_u8(0x28)?;
                jump_dest.write_binary(sink)?;
            }
            OpCode::CallIndirect(table) => {
                sink.write_u8(0x2A)?;
                table.write_binary(sink)?;
            }

            // global opcode family
            OpCode::GlobalGet(index) => {
                sink.write_u8(0x30)?;
                index.write_binary(sink)?;
            }
            OpCode::GlobalSet(index) => {
                sink.write_u8(0x31)?;
                index.write_binary(sink)?;
            }

            // memory opcode family
            OpCode::I32Load(offset) => {
                sink.write_u8(0x40)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load(offset) => {
                sink.write_u8(0x41)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Load8S(offset) => {
                sink.write_u8(0x42)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Load8U(offset) => {
                sink.write_u8(0x43)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Load16S(offset) => {
                sink.write_u8(0x44)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Load16U(offset) => {
                sink.write_u8(0x45)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load8S(offset) => {
                sink.write_u8(0x46)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load8U(offset) => {
                sink.write_u8(0x47)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load16S(offset) => {
                sink.write_u8(0x48)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load16U(offset) => {
                sink.write_u8(0x49)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load32S(offset) => {
                sink.write_u8(0x4A)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Load32U(offset) => {
                sink.write_u8(0x4B)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Store(offset) => {
                sink.write_u8(0x4C)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Store(offset) => {
                sink.write_u8(0x4D)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Store8(offset) => {
                sink.write_u8(0x4E)?;
                offset.write_binary(sink)?;
            }
            OpCode::I32Store16(offset) => {
                sink.write_u8(0x4F)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Store8(offset) => {
                sink.write_u8(0x50)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Store16(offset) => {
                sink.write_u8(0x51)?;
                offset.write_binary(sink)?;
            }
            OpCode::I64Store32(offset) => {
                sink.write_u8(0x52)?;
                offset.write_binary(sink)?;
            }

            // memory data opcode family (?)
            OpCode::MemorySize => sink.write_u8(0x53)?,
            OpCode::MemoryGrow => sink.write_u8(0x54)?,
            OpCode::MemoryFill => sink.write_u8(0x55)?,
            OpCode::MemoryCopy => sink.write_u8(0x56)?,
            OpCode::MemoryInit(index) => {
                sink.write_u8(0x57)?;
                index.write_binary(sink)?;
            }
            OpCode::DataDrop(index) => {
                sink.write_u8(0x58)?;
                index.write_binary(sink)?;
            }
            OpCode::TableSize(index) => {
                sink.write_u8(0x59)?;
                index.write_binary(sink)?;
            }
            OpCode::TableGrow(index) => {
                sink.write_u8(0x5a)?;
                index.write_binary(sink)?;
            }
            OpCode::TableFill(index) => {
                sink.write_u8(0x5b)?;
                index.write_binary(sink)?;
            }
            OpCode::TableGet(index) => {
                sink.write_u8(0x5c)?;
                index.write_binary(sink)?;
            }
            OpCode::TableSet(index) => {
                sink.write_u8(0x5d)?;
                index.write_binary(sink)?;
            }
            OpCode::TableCopy { dst, src } => {
                sink.write_u8(0x5e)?;
                dst.write_binary(sink)?;
                src.write_binary(sink)?;
            }
            OpCode::TableInit { table, elem } => {
                sink.write_u8(0x5f)?;
                table.write_binary(sink)?;
                elem.write_binary(sink)?;
            }
            // OpCode::ElemDrop(index) => {
            //     sink.write_u8(0x60)?;
            //     index.write_binary(sink)?;
            // }
            // OpCode::RefFunc(index) => {
            //     sink.write_u8(0x61)?;
            //     index.write_binary(sink)?;
            // }

            // i32/i64 opcode family
            OpCode::I64Const(untyped_value) => {
                sink.write_u8(0x60)?;
                untyped_value.write_binary(sink)?;
            }
            OpCode::I32Const(untyped_value) => {
                sink.write_u8(0x61)?;
                untyped_value.write_binary(sink)?;
            }
            OpCode::I32Eqz => sink.write_u8(0x62)?,
            OpCode::I32Eq => sink.write_u8(0x63)?,
            OpCode::I32Ne => sink.write_u8(0x64)?,
            OpCode::I32LtS => sink.write_u8(0x65)?,
            OpCode::I32LtU => sink.write_u8(0x66)?,
            OpCode::I32GtS => sink.write_u8(0x67)?,
            OpCode::I32GtU => sink.write_u8(0x68)?,
            OpCode::I32LeS => sink.write_u8(0x69)?,
            OpCode::I32LeU => sink.write_u8(0x6A)?,
            OpCode::I32GeS => sink.write_u8(0x6B)?,
            OpCode::I32GeU => sink.write_u8(0x6C)?,
            OpCode::I64Eqz => sink.write_u8(0x6D)?,
            OpCode::I64Eq => sink.write_u8(0x6E)?,
            OpCode::I64Ne => sink.write_u8(0x6F)?,
            OpCode::I64LtS => sink.write_u8(0x70)?,
            OpCode::I64LtU => sink.write_u8(0x71)?,
            OpCode::I64GtS => sink.write_u8(0x72)?,
            OpCode::I64GtU => sink.write_u8(0x73)?,
            OpCode::I64LeS => sink.write_u8(0x74)?,
            OpCode::I64LeU => sink.write_u8(0x75)?,
            OpCode::I64GeS => sink.write_u8(0x76)?,
            OpCode::I64GeU => sink.write_u8(0x77)?,
            OpCode::I32Clz => sink.write_u8(0x78)?,
            OpCode::I32Ctz => sink.write_u8(0x79)?,
            OpCode::I32Popcnt => sink.write_u8(0x7A)?,
            OpCode::I32Add => sink.write_u8(0x7B)?,
            OpCode::I32Sub => sink.write_u8(0x7C)?,
            OpCode::I32Mul => sink.write_u8(0x7D)?,
            OpCode::I32DivS => sink.write_u8(0x7E)?,
            OpCode::I32DivU => sink.write_u8(0x7F)?,
            OpCode::I32RemS => sink.write_u8(0x80)?,
            OpCode::I32RemU => sink.write_u8(0x81)?,
            OpCode::I32And => sink.write_u8(0x82)?,
            OpCode::I32Or => sink.write_u8(0x83)?,
            OpCode::I32Xor => sink.write_u8(0x84)?,
            OpCode::I32Shl => sink.write_u8(0x85)?,
            OpCode::I32ShrS => sink.write_u8(0x86)?,
            OpCode::I32ShrU => sink.write_u8(0x87)?,
            OpCode::I32Rotl => sink.write_u8(0x88)?,
            OpCode::I32Rotr => sink.write_u8(0x89)?,
            OpCode::I64Clz => sink.write_u8(0x8A)?,
            OpCode::I64Ctz => sink.write_u8(0x8B)?,
            OpCode::I64Popcnt => sink.write_u8(0x8C)?,
            OpCode::I64Add => sink.write_u8(0x8D)?,
            OpCode::I64Sub => sink.write_u8(0x8E)?,
            OpCode::I64Mul => sink.write_u8(0x8F)?,
            OpCode::I64DivS => sink.write_u8(0x90)?,
            OpCode::I64DivU => sink.write_u8(0x91)?,
            OpCode::I64RemS => sink.write_u8(0x92)?,
            OpCode::I64RemU => sink.write_u8(0x93)?,
            OpCode::I64And => sink.write_u8(0x94)?,
            OpCode::I64Or => sink.write_u8(0x95)?,
            OpCode::I64Xor => sink.write_u8(0x96)?,
            OpCode::I64Shl => sink.write_u8(0x97)?,
            OpCode::I64ShrS => sink.write_u8(0x98)?,
            OpCode::I64ShrU => sink.write_u8(0x99)?,
            OpCode::I64Rotl => sink.write_u8(0x9A)?,
            OpCode::I64Rotr => sink.write_u8(0x9B)?,
            OpCode::I32WrapI64 => sink.write_u8(0x9C)?,
            OpCode::I64ExtendI32S => sink.write_u8(0x9D)?,
            OpCode::I64ExtendI32U => sink.write_u8(0x9E)?,
            OpCode::I32Extend8S => sink.write_u8(0x9F)?,
            OpCode::I32Extend16S => sink.write_u8(0xA0)?,
            OpCode::I64Extend8S => sink.write_u8(0xA1)?,
            OpCode::I64Extend16S => sink.write_u8(0xA2)?,
            OpCode::I64Extend32S => sink.write_u8(0xA3)?,

            _ => return Ok(()),
        }
        Ok(())
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<OpCode, BinaryFormatError> {
        let byte = sink.read_u8()?;
        Ok(match byte {
            0x00 => OpCode::Unreachable,
            0x01 => OpCode::ConsumeFuel(Fuel::read_binary(sink)?),
            0x02 => OpCode::Drop,
            0x04 => OpCode::Select,

            // local opcode family
            0x10 => OpCode::LocalGet(Index::read_binary(sink)?),
            0x11 => OpCode::LocalSet(Index::read_binary(sink)?),
            0x12 => OpCode::LocalTee(Index::read_binary(sink)?),

            // control flow opcode family
            0x20 => OpCode::Br(BranchParams::read_binary(sink)?),
            0x21 => OpCode::BrIfEqz(BranchParams::read_binary(sink)?),
            0x22 => OpCode::BrIfNez(BranchParams::read_binary(sink)?),
            0x23 => OpCode::BrTable(Index::read_binary(sink)?),
            0x24 => OpCode::Return(DropKeep::read_binary(sink)?),
            0x25 => OpCode::ReturnIfNez(DropKeep::read_binary(sink)?),
            0x26 => OpCode::ReturnCall(Index::read_binary(sink)?, DropKeep::read_binary(sink)?),
            0x27 => OpCode::ReturnCallIndirect(Index::read_binary(sink)?, DropKeep::read_binary(sink)?),
            0x28 => OpCode::Call(Index::read_binary(sink)?),
            0x2A => OpCode::CallIndirect(Index::read_binary(sink)?),

            // global opcode family
            0x30 => OpCode::GlobalGet(Index::read_binary(sink)?),
            0x31 => OpCode::GlobalSet(Index::read_binary(sink)?),

            // memory opcode family
            0x40 => OpCode::I32Load(Offset::read_binary(sink)?),
            0x41 => OpCode::I64Load(Offset::read_binary(sink)?),
            0x42 => OpCode::I32Load8S(Offset::read_binary(sink)?),
            0x43 => OpCode::I32Load8U(Offset::read_binary(sink)?),
            0x44 => OpCode::I32Load16S(Offset::read_binary(sink)?),
            0x45 => OpCode::I32Load16U(Offset::read_binary(sink)?),
            0x46 => OpCode::I64Load8S(Offset::read_binary(sink)?),
            0x47 => OpCode::I64Load8U(Offset::read_binary(sink)?),
            0x48 => OpCode::I64Load16S(Offset::read_binary(sink)?),
            0x49 => OpCode::I64Load16U(Offset::read_binary(sink)?),
            0x4A => OpCode::I64Load32S(Offset::read_binary(sink)?),
            0x4B => OpCode::I64Load32U(Offset::read_binary(sink)?),
            0x4C => OpCode::I32Store(Offset::read_binary(sink)?),
            0x4D => OpCode::I64Store(Offset::read_binary(sink)?),
            0x4E => OpCode::I32Store8(Offset::read_binary(sink)?),
            0x4F => OpCode::I32Store16(Offset::read_binary(sink)?),
            0x50 => OpCode::I64Store8(Offset::read_binary(sink)?),
            0x51 => OpCode::I64Store16(Offset::read_binary(sink)?),
            0x52 => OpCode::I64Store32(Offset::read_binary(sink)?),

            // memory data opcode family (?)
            0x53 => OpCode::MemorySize,
            0x54 => OpCode::MemoryGrow,
            0x55 => OpCode::MemoryFill,
            0x56 => OpCode::MemoryCopy,
            0x57 => OpCode::MemoryInit(Index::read_binary(sink)?),
            0x58 => OpCode::DataDrop(Index::read_binary(sink)?),
            0x59 => OpCode::TableSize(Index::read_binary(sink)?),
            0x5A => OpCode::TableGrow(Index::read_binary(sink)?),
            0x5B => OpCode::TableFill(Index::read_binary(sink)?),
            0x5C => OpCode::TableGet(Index::read_binary(sink)?),
            0x5D => OpCode::TableSet(Index::read_binary(sink)?),
            0x5E => OpCode::TableCopy {
                dst: Index::read_binary(sink)?,
                src: Index::read_binary(sink)?,
            },
            0x5F => OpCode::TableInit {
                table: Index::read_binary(sink)?,
                elem: Index::read_binary(sink)?,
            },
            // 0x60 => OpCode::ElemDrop(Index::read_binary(sink)?),
            // 0x61 => OpCode::RefFunc(Index::read_binary(sink)?),

            // i32/i64 opcode family
            0x60 => OpCode::I64Const(UntypedValue::read_binary(sink)?),
            0x61 => OpCode::I32Const(UntypedValue::read_binary(sink)?),
            0x62 => OpCode::I32Eqz,
            0x63 => OpCode::I32Eq,
            0x64 => OpCode::I32Ne,
            0x65 => OpCode::I32LtS,
            0x66 => OpCode::I32LtU,
            0x67 => OpCode::I32GtS,
            0x68 => OpCode::I32GtU,
            0x69 => OpCode::I32LeS,
            0x6A => OpCode::I32LeU,
            0x6B => OpCode::I32GeS,
            0x6C => OpCode::I32GeU,
            0x6D => OpCode::I64Eqz,
            0x6E => OpCode::I64Eq,
            0x6F => OpCode::I64Ne,
            0x70 => OpCode::I64LtS,
            0x71 => OpCode::I64LtU,
            0x72 => OpCode::I64GtS,
            0x73 => OpCode::I64GtU,
            0x74 => OpCode::I64LeS,
            0x75 => OpCode::I64LeU,
            0x76 => OpCode::I64GeS,
            0x77 => OpCode::I64GeU,
            0x78 => OpCode::I32Clz,
            0x79 => OpCode::I32Ctz,
            0x7A => OpCode::I32Popcnt,
            0x7B => OpCode::I32Add,
            0x7C => OpCode::I32Sub,
            0x7D => OpCode::I32Mul,
            0x7E => OpCode::I32DivS,
            0x7F => OpCode::I32DivU,
            0x80 => OpCode::I32RemS,
            0x81 => OpCode::I32RemU,
            0x82 => OpCode::I32And,
            0x83 => OpCode::I32Or,
            0x84 => OpCode::I32Xor,
            0x85 => OpCode::I32Shl,
            0x86 => OpCode::I32ShrS,
            0x87 => OpCode::I32ShrU,
            0x88 => OpCode::I32Rotl,
            0x89 => OpCode::I32Rotr,
            0x8A => OpCode::I64Clz,
            0x8B => OpCode::I64Ctz,
            0x8C => OpCode::I64Popcnt,
            0x8D => OpCode::I64Add,
            0x8E => OpCode::I64Sub,
            0x8F => OpCode::I64Mul,
            0x90 => OpCode::I64DivS,
            0x91 => OpCode::I64DivU,
            0x92 => OpCode::I64RemS,
            0x93 => OpCode::I64RemU,
            0x94 => OpCode::I64And,
            0x95 => OpCode::I64Or,
            0x96 => OpCode::I64Xor,
            0x97 => OpCode::I64Shl,
            0x98 => OpCode::I64ShrS,
            0x99 => OpCode::I64ShrU,
            0x9A => OpCode::I64Rotl,
            0x9B => OpCode::I64Rotr,
            0x9C => OpCode::I32WrapI64,
            0x9D => OpCode::I64ExtendI32S,
            0x9E => OpCode::I64ExtendI32U,
            0x9F => OpCode::I32Extend8S,
            0xA0 => OpCode::I32Extend16S,
            0xA1 => OpCode::I64Extend8S,
            0xA2 => OpCode::I64Extend16S,
            0xA3 => OpCode::I64Extend32S,

            _ => return Err(BinaryFormatError::IllegalOpcode(byte)),
        })
    }
}

impl<'a> BinaryFormat<'a> for InstructionSet {
    type SelfType = InstructionSet;

    fn write_binary(&self, sink: &mut BinaryFormatWriter<'a>) -> Result<(), BinaryFormatError> {
        for opcode in self.0.iter() {
            opcode.write_binary(sink)?;
        }
        Ok(())
    }

    fn read_binary(sink: &mut BinaryFormatReader<'a>) -> Result<InstructionSet, BinaryFormatError> {
        let mut result = InstructionSet::new();
        while !sink.is_empty() {
            result.push(OpCode::read_binary(sink)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryFormat, BinaryFormatReader, BinaryFormatWriter, OpCode};
    use strum::IntoEnumIterator;

    #[test]
    fn test_opcode_encoding() {
        for opcode in OpCode::iter() {
            let mut buf = vec![0; 100];
            let mut writer = BinaryFormatWriter::new(buf.as_mut_slice());
            opcode.write_binary(&mut writer).unwrap();
            let mut reader = BinaryFormatReader::new(buf.as_slice());
            let opcode2 = OpCode::read_binary(&mut reader).unwrap();
            // TODO: "skip unreachable for now, these are floating point ops"
            if opcode2 == OpCode::Unreachable {
                println!("{:?} <> {:?}", opcode, opcode2);
                continue;
            }
            assert_eq!(opcode, opcode2);
        }
    }
}

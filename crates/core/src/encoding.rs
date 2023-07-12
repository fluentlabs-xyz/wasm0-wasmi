use crate::{BinaryFormat, Fuel, Index, JumpDest, Offset, OpCode, UntypedValue, WazmError, WazmResult};
use std::io::{Cursor, Read};

impl<'a> BinaryFormat<'a> for OpCode {
    type SelfType = OpCode;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()> {
        match self {
            OpCode::Unreachable => sink.push(0x00),
            OpCode::ConsumeFuel(u) => {
                sink.push(0x01);
                u.write_binary(sink)?;
            }
            OpCode::Drop => sink.push(0x02),
            OpCode::Select => sink.push(0x04),

            // local opcode family
            OpCode::LocalGet(index) => {
                sink.push(0x10);
                index.write_binary(sink)?;
            }
            OpCode::LocalSet(index) => {
                sink.push(0x11);
                index.write_binary(sink)?;
            }
            OpCode::LocalTee(index) => {
                sink.push(0x12);
                index.write_binary(sink)?;
            }

            // control flow opcode family
            OpCode::Br(branch_params) => {
                sink.push(0x20);
                branch_params.write_binary(sink)?;
            }
            OpCode::BrIfEqz(branch_params) => {
                sink.push(0x21);
                branch_params.write_binary(sink)?;
            }
            OpCode::BrIfNez(branch_params) => {
                sink.push(0x22);
                branch_params.write_binary(sink)?;
            }
            OpCode::BrTable(index) => {
                sink.push(0x23);
                index.write_binary(sink)?;
            }
            OpCode::Return => {
                sink.push(0x24);
            }
            OpCode::ReturnCallIndirect(table) => {
                sink.push(0x27);
                table.write_binary(sink)?;
            }
            OpCode::Call(jump_dest) => {
                sink.push(0x28);
                jump_dest.write_binary(sink)?;
            }
            OpCode::CallHost(index) => {
                sink.push(0x29);
                index.write_binary(sink)?;
            }
            OpCode::CallIndirect(table) => {
                sink.push(0x2A);
                table.write_binary(sink)?;
            }

            // global opcode family
            OpCode::GlobalGet(index) => {
                sink.push(0x30);
                index.write_binary(sink)?;
            }
            OpCode::GlobalSet(index) => {
                sink.push(0x31);
                index.write_binary(sink)?;
            }

            // memory opcode family
            OpCode::I32Load(offset) => {
                sink.push(0x40);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load(offset) => {
                sink.push(0x41);
                offset.write_binary(sink)?;
            }
            OpCode::I32Load8S(offset) => {
                sink.push(0x42);
                offset.write_binary(sink)?;
            }
            OpCode::I32Load8U(offset) => {
                sink.push(0x43);
                offset.write_binary(sink)?;
            }
            OpCode::I32Load16S(offset) => {
                sink.push(0x44);
                offset.write_binary(sink)?;
            }
            OpCode::I32Load16U(offset) => {
                sink.push(0x45);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load8S(offset) => {
                sink.push(0x46);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load8U(offset) => {
                sink.push(0x47);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load16S(offset) => {
                sink.push(0x48);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load16U(offset) => {
                sink.push(0x49);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load32S(offset) => {
                sink.push(0x4A);
                offset.write_binary(sink)?;
            }
            OpCode::I64Load32U(offset) => {
                sink.push(0x4B);
                offset.write_binary(sink)?;
            }
            OpCode::I32Store(offset) => {
                sink.push(0x4C);
                offset.write_binary(sink)?;
            }
            OpCode::I64Store(offset) => {
                sink.push(0x4D);
                offset.write_binary(sink)?;
            }
            OpCode::I32Store8(offset) => {
                sink.push(0x4E);
                offset.write_binary(sink)?;
            }
            OpCode::I32Store16(offset) => {
                sink.push(0x4F);
                offset.write_binary(sink)?;
            }
            OpCode::I64Store8(offset) => {
                sink.push(0x50);
                offset.write_binary(sink)?;
            }
            OpCode::I64Store16(offset) => {
                sink.push(0x51);
                offset.write_binary(sink)?;
            }
            OpCode::I64Store32(offset) => {
                sink.push(0x52);
                offset.write_binary(sink)?;
            }

            // memory data opcode family (?)
            // OpCode::MemorySize => sink.push(0x53),
            // OpCode::MemoryGrow => sink.push(0x54),
            // OpCode::MemoryFill => sink.push(0x55),
            // OpCode::MemoryCopy => sink.push(0x56),
            // Instruction::DataDrop(_) => {}
            // Instruction::TableSize(_) => {}
            // Instruction::TableGrow(_) => {}
            // Instruction::TableFill(_) => {}
            // Instruction::TableGet(_) => {}
            // Instruction::TableSet(_) => {}
            // Instruction::TableCopy { .. } => {}
            // Instruction::TableInit { .. } => {}
            // Instruction::ElemDrop(_) => {}
            // Instruction::RefFunc { .. } => {}

            // i32/i64 opcode family
            OpCode::I64Const(untyped_value) => {
                sink.push(0x60);
                untyped_value.write_binary(sink)?;
            }
            OpCode::I32Const(untyped_value) => {
                sink.push(0x61);
                untyped_value.write_binary(sink)?;
            }
            OpCode::I32Eqz => sink.push(0x62),
            OpCode::I32Eq => sink.push(0x63),
            OpCode::I32Ne => sink.push(0x64),
            OpCode::I32LtS => sink.push(0x65),
            OpCode::I32LtU => sink.push(0x66),
            OpCode::I32GtS => sink.push(0x67),
            OpCode::I32GtU => sink.push(0x68),
            OpCode::I32LeS => sink.push(0x69),
            OpCode::I32LeU => sink.push(0x6A),
            OpCode::I32GeS => sink.push(0x6B),
            OpCode::I32GeU => sink.push(0x6C),
            OpCode::I64Eqz => sink.push(0x6D),
            OpCode::I64Eq => sink.push(0x6E),
            OpCode::I64Ne => sink.push(0x6F),
            OpCode::I64LtS => sink.push(0x70),
            OpCode::I64LtU => sink.push(0x71),
            OpCode::I64GtS => sink.push(0x72),
            OpCode::I64GtU => sink.push(0x73),
            OpCode::I64LeS => sink.push(0x74),
            OpCode::I64LeU => sink.push(0x75),
            OpCode::I64GeS => sink.push(0x76),
            OpCode::I64GeU => sink.push(0x77),
            OpCode::I32Clz => sink.push(0x78),
            OpCode::I32Ctz => sink.push(0x79),
            OpCode::I32Popcnt => sink.push(0x7A),
            OpCode::I32Add => sink.push(0x7B),
            OpCode::I32Sub => sink.push(0x7C),
            OpCode::I32Mul => sink.push(0x7D),
            OpCode::I32DivS => sink.push(0x7E),
            OpCode::I32DivU => sink.push(0x7F),
            OpCode::I32RemS => sink.push(0x80),
            OpCode::I32RemU => sink.push(0x81),
            OpCode::I32And => sink.push(0x82),
            OpCode::I32Or => sink.push(0x83),
            OpCode::I32Xor => sink.push(0x84),
            OpCode::I32Shl => sink.push(0x85),
            OpCode::I32ShrS => sink.push(0x86),
            OpCode::I32ShrU => sink.push(0x87),
            OpCode::I32Rotl => sink.push(0x88),
            OpCode::I32Rotr => sink.push(0x89),
            OpCode::I64Clz => sink.push(0x8A),
            OpCode::I64Ctz => sink.push(0x8B),
            OpCode::I64Popcnt => sink.push(0x8C),
            OpCode::I64Add => sink.push(0x8D),
            OpCode::I64Sub => sink.push(0x8E),
            OpCode::I64Mul => sink.push(0x8F),
            OpCode::I64DivS => sink.push(0x90),
            OpCode::I64DivU => sink.push(0x91),
            OpCode::I64RemS => sink.push(0x92),
            OpCode::I64RemU => sink.push(0x93),
            OpCode::I64And => sink.push(0x94),
            OpCode::I64Or => sink.push(0x95),
            OpCode::I64Xor => sink.push(0x96),
            OpCode::I64Shl => sink.push(0x97),
            OpCode::I64ShrS => sink.push(0x98),
            OpCode::I64ShrU => sink.push(0x99),
            OpCode::I64Rotl => sink.push(0x9A),
            OpCode::I64Rotr => sink.push(0x9B),
            OpCode::I32WrapI64 => sink.push(0x9C),
            OpCode::I64ExtendI32S => sink.push(0x9D),
            OpCode::I64ExtendI32U => sink.push(0x9E),
            OpCode::I32Extend8S => sink.push(0x9F),
            OpCode::I32Extend16S => sink.push(0xA0),
            OpCode::I64Extend8S => sink.push(0xA1),
            OpCode::I64Extend16S => sink.push(0xA2),
            OpCode::I64Extend32S => sink.push(0xA3),

            _ => return Ok(()),
        }
        Ok(())
    }

    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self> {
        let mut buf = [0; 1];
        sink.read_exact(&mut buf).map_err(|_| WazmError::OutOfBuffer)?;
        let byte = buf[0];
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
            0x20 => OpCode::Br(JumpDest::read_binary(sink)?),
            0x21 => OpCode::BrIfEqz(JumpDest::read_binary(sink)?),
            0x22 => OpCode::BrIfNez(JumpDest::read_binary(sink)?),
            0x23 => OpCode::BrTable(Index::read_binary(sink)?),
            0x24 => OpCode::Return,
            // 0x26 => OpCode::ReturnCall(Index::read_binary(sink)),
            0x27 => OpCode::ReturnCallIndirect(Index::read_binary(sink)?),
            0x28 => OpCode::Call(JumpDest::read_binary(sink)?),
            0x29 => OpCode::CallHost(Index::read_binary(sink)?),
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
            // 0x53 => OpCode::MemorySize,
            // 0x54 => OpCode::MemoryGrow,
            // 0x55 => OpCode::MemoryFill,
            // 0x56 => OpCode::MemoryCopy,
            // Instruction::DataDrop(_) => {}
            // Instruction::TableSize(_) => {}
            // Instruction::TableGrow(_) => {}
            // Instruction::TableFill(_) => {}
            // Instruction::TableGet(_) => {}
            // Instruction::TableSet(_) => {}
            // Instruction::TableCopy { .. } => {}
            // Instruction::TableInit { .. } => {}
            // Instruction::ElemDrop(_) => {}
            // Instruction::RefFunc { .. } => {}

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

            _ => return Err(WazmError::IllegalOpcode(byte)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryFormat, OpCode};
    use std::io::Cursor;
    use strum::IntoEnumIterator;

    #[test]
    fn test_opcode_encoding() {
        for opcode in OpCode::iter() {
            let mut buf = Vec::new();
            opcode.write_binary(&mut buf).unwrap();
            // TODO: "skip empty encoding for now, these are floating point ops"
            if buf.is_empty() {
                continue;
            }
            let mut cur = Cursor::new(buf.as_slice());
            let opcode2 = OpCode::read_binary(&mut cur).unwrap();
            assert_eq!(opcode, opcode2);
        }
    }
}

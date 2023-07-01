use crate::engine::{bytecode::Instruction, DropKeep};

#[derive(Debug, Clone)]
pub struct OpCode(pub Instruction);

impl OpCode {
    pub fn name(&self) -> &'static str {
        use Instruction::*;
        match self.0 {
            LocalGet { .. } => "local_get",
            LocalSet { .. } => "local_set",
            LocalTee { .. } => "local_tee",
            Br(_) => "br",
            BrIfEqz(_) => "br_if_eqz",
            BrIfNez(_) => "br_if_nez",
            BrTable { .. } => "br_table",
            Unreachable => "unreachable",
            ConsumeFuel { .. } => "consume_fuel",
            Return(_) => "return",
            ReturnIfNez(_) => "return_if_nez",
            ReturnCall { .. } => "return_call",
            ReturnCallIndirect { .. } => "return_call_indirect",
            Call(_) => "call",
            CallIndirect { .. } => "call_indirect",
            Drop => "drop",
            Select => "select",
            GlobalGet(_) => "global_get",
            GlobalSet(_) => "global_set",
            I32Load(_) => "i32_load",
            I64Load(_) => "i64_load",
            F32Load(_) => "f32_load",
            F64Load(_) => "f64_load",
            I32Load8S(_) => "i32_load8_s",
            I32Load8U(_) => "i32_load8_u",
            I32Load16S(_) => "i32_load16_s",
            I32Load16U(_) => "i32_load16_u",
            I64Load8S(_) => "i64_load8_s",
            I64Load8U(_) => "i64_load8_u",
            I64Load16S(_) => "i64_load16_s",
            I64Load16U(_) => "i64_load16_u",
            I64Load32S(_) => "i64_load32_s",
            I64Load32U(_) => "i64_load32_u",
            I32Store(_) => "i32_store",
            I64Store(_) => "i64_store",
            F32Store(_) => "f32_store",
            F64Store(_) => "f64_store",
            I32Store8(_) => "i32_store8",
            I32Store16(_) => "i32_store16",
            I64Store8(_) => "i64_store8",
            I64Store16(_) => "i64_store16",
            I64Store32(_) => "i64_store32",
            MemorySize => "memory_size",
            MemoryGrow => "memory_grow",
            MemoryFill => "memory_fill",
            MemoryCopy => "memory_copy",
            MemoryInit(_) => "memory_init",
            DataDrop(_) => "data_drop",
            TableSize { .. } => "table_size",
            TableGrow { .. } => "table_grow",
            TableFill { .. } => "table_fill",
            TableGet { .. } => "table_get",
            TableSet { .. } => "table_set",
            TableCopy { .. } => "table_copy",
            TableInit { .. } => "table_init",
            ElemDrop(_) => "elem_drop",
            RefFunc { .. } => "ref_func",
            I32Const(_) => "i32_const",
            I64Const(_) => "i64_const",
            I32Eqz => "i32_eqz",
            I32Eq => "i32_eq",
            I32Ne => "i32_ne",
            I32LtS => "i32_lt_s",
            I32LtU => "i32_lt_u",
            I32GtS => "i32_gt_s",
            I32GtU => "i32_gt_u",
            I32LeS => "i32_le_s",
            I32LeU => "i32_le_u",
            I32GeS => "i32_ge_s",
            I32GeU => "i32_ge_u",
            I64Eqz => "i64_eqz",
            I64Eq => "i64_eq",
            I64Ne => "i64_ne",
            I64LtS => "i64_lt_s",
            I64LtU => "i64_lt_u",
            I64GtS => "i64_gt_s",
            I64GtU => "i64_gt_u",
            I64LeS => "i64_le_s",
            I64LeU => "i64_le_u",
            I64GeS => "i64_ge_s",
            I64GeU => "i64_ge_u",
            F32Eq => "f32_eq",
            F32Ne => "f32_ne",
            F32Lt => "f32_lt",
            F32Gt => "f32_gt",
            F32Le => "f32_le",
            F32Ge => "f32_ge",
            F64Eq => "f64_eq",
            F64Ne => "f64_ne",
            F64Lt => "f64_lt",
            F64Gt => "f64_gt",
            F64Le => "f64_le",
            F64Ge => "f64_ge",
            I32Clz => "i32_clz",
            I32Ctz => "i32_ctz",
            I32Popcnt => "i32_popcnt",
            I32Add => "i32_add",
            I32Sub => "i32_sub",
            I32Mul => "i32_mul",
            I32DivS => "i32_div_s",
            I32DivU => "i32_div_u",
            I32RemS => "i32_rem_s",
            I32RemU => "i32_rem_u",
            I32And => "i32_and",
            I32Or => "i32_or",
            I32Xor => "i32_xor",
            I32Shl => "i32_shl",
            I32ShrS => "i32_shr_s",
            I32ShrU => "i32_shr_u",
            I32Rotl => "i32_rotl",
            I32Rotr => "i32_rotr",
            I64Clz => "i64_clz",
            I64Ctz => "i64_ctz",
            I64Popcnt => "i64_popcnt",
            I64Add => "i64_add",
            I64Sub => "i64_sub",
            I64Mul => "i64_mul",
            I64DivS => "i64_div_s",
            I64DivU => "i64_div_u",
            I64RemS => "i64_rem_s",
            I64RemU => "i64_rem_u",
            I64And => "i64_and",
            I64Or => "i64_or",
            I64Xor => "i64_xor",
            I64Shl => "i64_shl",
            I64ShrS => "i64_shr_s",
            I64ShrU => "i64_shr_u",
            I64Rotl => "i64_rotl",
            I64Rotr => "i64_rotr",
            F32Abs => "f32_abs",
            F32Neg => "f32_neg",
            F32Ceil => "f32_ceil",
            F32Floor => "f32_floor",
            F32Trunc => "f32_trunc",
            F32Nearest => "f32_nearest",
            F32Sqrt => "f32_sqrt",
            F32Add => "f32_add",
            F32Sub => "f32_sub",
            F32Mul => "f32_mul",
            F32Div => "f32_div",
            F32Min => "f32_min",
            F32Max => "f32_max",
            F32Copysign => "f32_copysign",
            F64Abs => "f64_abs",
            F64Neg => "f64_neg",
            F64Ceil => "f64_ceil",
            F64Floor => "f64_floor",
            F64Trunc => "f64_trunc",
            F64Nearest => "f64_nearest",
            F64Sqrt => "f64_sqrt",
            F64Add => "f64_add",
            F64Sub => "f64_sub",
            F64Mul => "f64_mul",
            F64Div => "f64_div",
            F64Min => "f64_min",
            F64Max => "f64_max",
            F64Copysign => "f64_copysign",
            I32WrapI64 => "i32_wrap_i64",
            I32TruncF32S => "i32_trunc_f32_s",
            I32TruncF32U => "i32_trunc_f32_u",
            I32TruncF64S => "i32_trunc_f64_s",
            I32TruncF64U => "i32_trunc_f64_u",
            I64ExtendI32S => "i64_extend_i32_s",
            I64ExtendI32U => "i64_extend_i32_u",
            I64TruncF32S => "i64_trunc_f32_s",
            I64TruncF32U => "i64_trunc_f32_u",
            I64TruncF64S => "i64_trunc_f64_s",
            I64TruncF64U => "i64_trunc_f64_u",
            F32ConvertI32S => "f32_convert_i32_s",
            F32ConvertI32U => "f32_convert_i32_u",
            F32ConvertI64S => "f32_convert_i64_s",
            F32ConvertI64U => "f32_convert_i64_u",
            F32DemoteF64 => "f32_demote_f64",
            F64ConvertI32S => "f64_convert_i32_s",
            F64ConvertI32U => "f64_convert_i32_u",
            F64ConvertI64S => "f64_convert_i64_s",
            F64ConvertI64U => "f64_convert_i64_u",
            F64PromoteF32 => "f64_promote_f32",
            I32Extend8S => "i32_extend8_s",
            I32Extend16S => "i32_extend16_s",
            I64Extend8S => "i64_extend8_s",
            I64Extend16S => "i64_extend16_s",
            I64Extend32S => "i64_extend32_s",
            I32TruncSatF32S => "i32_trunc_sat_f32_s",
            I32TruncSatF32U => "i32_trunc_sat_f32_u",
            I32TruncSatF64S => "i32_trunc_sat_f64_s",
            I32TruncSatF64U => "i32_trunc_sat_f64_u",
            I64TruncSatF32S => "i64_trunc_sat_f32_s",
            I64TruncSatF32U => "i64_trunc_sat_f32_u",
            I64TruncSatF64S => "i64_trunc_sat_f64_s",
            I64TruncSatF64U => "i64_trunc_sat_f64_u",
        }
    }

    pub fn params(&self) -> Option<Vec<u64>> {
        let params = match self.0 {
            Instruction::LocalGet { local_depth }
            | Instruction::LocalSet { local_depth }
            | Instruction::LocalTee { local_depth } => vec![local_depth.into_inner() as u64],
            Instruction::Br(bp) | Instruction::BrIfEqz(bp) | Instruction::BrIfNez(bp) => {
                vec![bp.offset().into_i32() as u64]
            }
            Instruction::BrTable { len_targets } => vec![len_targets as u64],
            Instruction::ConsumeFuel { amount } => vec![amount as u64],
            Instruction::ReturnCall { func, .. } => vec![func.into_inner() as u64],
            Instruction::ReturnCallIndirect {
                table, func_type, ..
            } => vec![table.into_inner() as u64, func_type.into_inner() as u64],
            Instruction::Call(func) => vec![func.into_inner() as u64],
            Instruction::CallIndirect { table, func_type } => {
                vec![table.into_inner() as u64, func_type.into_inner() as u64]
            }
            Instruction::GlobalGet(global_index) | Instruction::GlobalSet(global_index) => {
                vec![global_index.into_inner() as u64]
            }
            Instruction::I32Load(offset)
            | Instruction::I64Load(offset)
            | Instruction::F32Load(offset)
            | Instruction::F64Load(offset)
            | Instruction::I32Load8S(offset)
            | Instruction::I32Load8U(offset)
            | Instruction::I32Load16S(offset)
            | Instruction::I32Load16U(offset)
            | Instruction::I64Load8S(offset)
            | Instruction::I64Load8U(offset)
            | Instruction::I64Load16S(offset)
            | Instruction::I64Load16U(offset)
            | Instruction::I64Load32S(offset)
            | Instruction::I64Load32U(offset)
            | Instruction::I32Store(offset)
            | Instruction::I64Store(offset)
            | Instruction::F32Store(offset)
            | Instruction::F64Store(offset)
            | Instruction::I32Store8(offset)
            | Instruction::I32Store16(offset)
            | Instruction::I64Store8(offset)
            | Instruction::I64Store16(offset)
            | Instruction::I64Store32(offset) => vec![offset.into_inner() as u64],
            Instruction::MemoryInit(data_segment) | Instruction::DataDrop(data_segment) => {
                vec![data_segment.into_inner() as u64]
            }
            Instruction::TableSize { table }
            | Instruction::TableGrow { table }
            | Instruction::TableFill { table }
            | Instruction::TableGet { table }
            | Instruction::TableSet { table } => vec![table.into_inner() as u64],
            Instruction::TableCopy { dst, src } => {
                vec![dst.into_inner() as u64, src.into_inner() as u64]
            }
            Instruction::TableInit { table, elem } => {
                vec![table.into_inner() as u64, elem.into_inner() as u64]
            }
            Instruction::ElemDrop(es) => vec![es.into_inner() as u64],
            Instruction::RefFunc { func_index } => vec![func_index.into_inner() as u64],
            _ => vec![],
        };
        Some(params).filter(|v| !v.is_empty())
    }

    pub fn drop_keep(&self) -> Option<DropKeep> {
        let drop_keep = match self.0 {
            // branch param
            Instruction::Br(bp) | Instruction::BrIfEqz(bp) | Instruction::BrIfNez(bp) => {
                Some(bp.drop_keep())
            }
            // drop keep
            Instruction::Return(drop_keep) | Instruction::ReturnIfNez(drop_keep) => Some(drop_keep),
            // objs
            Instruction::ReturnCall { drop_keep, .. }
            | Instruction::ReturnCallIndirect { drop_keep, .. } => Some(drop_keep),
            // no drop keep
            _ => None,
        };
        // replace empty drop keep with none
        drop_keep.filter(|drop_keep| *drop_keep != DropKeep::none())
    }
}
